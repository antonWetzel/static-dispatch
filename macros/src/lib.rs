use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Error, Fields, FnArg, GenericParam, Item, ItemEnum, ItemTrait, Path, Token, TraitItem,
    WhereClause, parse::Parse, parse_macro_input,
};

/// Setup this type for static dispatch with [`implementation`].
///
/// See the module for documentation.
#[proc_macro_attribute]
pub fn setup(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // todo: avoid double parse, we just need the name
    let input = parse_macro_input!(item as Item);
    let name = match &input {
        Item::Trait(value) => &value.ident,
        Item::Enum(value) => &value.ident,
        _ => {
            return Error::new_spanned(&input, "dispatch is only valid on traits or enums")
                .to_compile_error()
                .into();
        }
    };

    let save = macro_data::save(name, &input);

    quote! {
        #input
        #save
    }
    .into()
}

struct GenerateInput {
    trait_name: Path,
    _for: Token![for],
    enum_name: Path,
}

impl Parse for GenerateInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            trait_name: input.parse()?,
            _for: input.parse()?,
            enum_name: input.parse()?,
        })
    }
}

/// Syntax: `implementation!(<trait> for <enum>)`
///
/// Generate the trait implementation for the enum.
/// Both require a `![setup]` annotation.
///
/// Because this uses macros to read the data, use `<crate>::<name>`
/// for external types.
#[proc_macro]
pub fn implementation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GenerateInput);

    let data = FinalTransfer {
        trait_item: macro_data::request(&input.trait_name),
        comma: syn::token::Comma(Span::mixed_site()),
        enum_item: macro_data::request(&input.enum_name),
    };

    macro_data::transfer("static_dispatch", "generate_final", &data).into()
}

struct FinalTransfer<S: macro_data::Storage> {
    trait_item: macro_data::Transfer<ItemTrait, S>,
    comma: Token![,],
    enum_item: macro_data::Transfer<ItemEnum, S>,
}

impl ToTokens for FinalTransfer<macro_data::Request> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.trait_item.to_tokens(tokens);
        self.comma.to_tokens(tokens);
        self.enum_item.to_tokens(tokens);
    }
}

impl Parse for FinalTransfer<macro_data::Load> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            trait_item: input.parse()?,
            comma: input.parse()?,
            enum_item: input.parse()?,
        })
    }
}

/// This macro is designed to be called by other macros, not in normal code.
///
/// See the module for documentation.
#[doc(hidden)]
#[proc_macro]
pub fn generate_final(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FinalTransfer<macro_data::Load>);
    let trait_item = input.trait_item.0;
    let enum_item = input.enum_item.0;

    let trait_ident = &trait_item.ident;
    let enum_ident = &enum_item.ident;

    // Combine generic parameters from trait and enum.
    let mut all_params = Vec::new();
    for param in &trait_item.generics.params {
        all_params.push(param.clone());
    }
    for param in &enum_item.generics.params {
        all_params.push(param.clone());
    }
    all_params.sort_by_key(|param| match param {
        GenericParam::Lifetime(_) => 0,
        GenericParam::Const(_) => 1,
        GenericParam::Type(_) => 2,
    });

    let impl_generics = if all_params.is_empty() {
        quote! {}
    } else {
        quote! { < #(#all_params),* > }
    };

    // Combine where clauses from trait and enum.
    let mut where_predicates = Vec::new();
    if let Some(wc) = &trait_item.generics.where_clause {
        where_predicates.extend(wc.predicates.iter().cloned());
    }
    if let Some(wc) = &enum_item.generics.where_clause {
        where_predicates.extend(wc.predicates.iter().cloned());
    }
    all_params.sort_by_key(|param| match param {
        GenericParam::Lifetime(_) => 0,
        GenericParam::Const(_) => 1,
        GenericParam::Type(_) => 2,
    });

    let where_clause = if where_predicates.is_empty() {
        None
    } else {
        Some(WhereClause {
            where_token: syn::token::Where::default(),
            predicates: syn::punctuated::Punctuated::from_iter(where_predicates),
        })
    };
    let trait_args = generic_args(&trait_item.generics);
    let enum_args = generic_args(&enum_item.generics);

    // Generate methods
    let impl_methods = trait_item
        .items
        .iter()
        .map(|item| {
            let TraitItem::Fn(method) = item else {
                return Error::new_spanned(item, "Only methods are supported").to_compile_error();
            };
            let sig = &method.sig;
            let method_name = &sig.ident;
            let method_gen = sig
                .generics
                .params
                .iter()
                .filter_map(|param| match param {
                    GenericParam::Lifetime(_) => None,
                    GenericParam::Const(param) => Some(&param.ident),
                    GenericParam::Type(param) => Some(&param.ident),
                })
                .collect::<Vec<_>>();

            let mut args = sig.inputs.iter();
            let self_arg = match args.next() {
                Some(FnArg::Receiver(rec)) => &rec.self_token,
                _ => {
                    return Error::new_spanned(sig, "Function requires self argument")
                        .to_compile_error();
                }
            };

            let args = sig
                .inputs
                .iter()
                .skip(1)
                .map(|arg| {
                    if let syn::FnArg::Typed(pat_type) = arg {
                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                            pat_ident.ident.clone()
                        } else {
                            panic!("Unsupported argument pattern");
                        }
                    } else {
                        panic!("Expected typed argument");
                    }
                })
                .collect::<Vec<_>>();

            let async_suffix = match sig.asyncness {
                None => quote! {},
                Some(_) => quote! {.await},
            };

            // Build match arms
            let arms = enum_item
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let Fields::Unnamed(fields) = &variant.fields else {
                        panic!("Only enum tuples supported");
                    };
                    let field = fields.unnamed.iter().next().expect("expected a field");
                    let field_type = &field.ty;
                    let method_gen = quote! { ::<#(#method_gen,)*> };
                    quote! {
                        #enum_ident::#variant_ident(__static_dispatch_value) =>
                            <#field_type as #trait_ident #trait_args>::#method_name #method_gen(
                            __static_dispatch_value,
                            #(#args),*
                        ) #async_suffix
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                #sig {
                    match #self_arg {
                        #(#arms,)*
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl #impl_generics #trait_ident #trait_args for #enum_ident #enum_args #where_clause {
            #(#impl_methods)*
        }
    };

    expanded.into()
}

fn generic_args(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let args: Vec<_> = generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Type(ty) => {
                let ident = &ty.ident;
                quote! { #ident }
            }
            GenericParam::Lifetime(lifetime) => {
                let lt = &lifetime.lifetime;
                quote! { #lt }
            }
            GenericParam::Const(c) => {
                let ident = &c.ident;
                quote! { #ident }
            }
        })
        .collect();
    if args.is_empty() {
        quote! {}
    } else {
        quote! { < #(#args),* > }
    }
}
