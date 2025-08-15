use std::ops::Not;

use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Error, FnArg, GenericParam, Generics, Ident, ItemEnum, ItemTrait, Path, PathArguments,
    ReturnType, Token, TraitItem, TraitItemFn, Type, TypeGenerics, TypeReference, parse::Parse,
};

#[proc_macro_attribute]
/// See the module for documentation.
pub fn dispatch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);

    let output = if let Ok(input_trait) = syn::parse2(item.clone()) {
        dispatch_trait(attr, input_trait)
    } else if let Ok(input_trait) = syn::parse2(item.clone()) {
        dispatch_enum(attr, input_trait)
    } else {
        Error::new_spanned(&item, "Could not parse as trait or enum").to_compile_error()
    };

    quote! {
        #item
        #output
    }
    .into()
}

fn is_self_type(ty: &Type) -> bool {
    match ty {
        // Check for plain `Self`
        Type::Path(type_path) => {
            type_path.qself.is_none()
                && type_path.path.segments.len() == 1
                && type_path.path.segments[0].ident == "Self"
                && matches!(type_path.path.segments[0].arguments, PathArguments::None)
        }
        // Check for `&Self` or `&mut Self`
        Type::Reference(TypeReference { elem, .. }) => is_self_type(elem),
        _ => false,
    }
}

fn is_valid_self(arg: Option<&FnArg>) -> bool {
    let Some(FnArg::Receiver(receiver)) = arg else {
        return false;
    };
    receiver.colon_token.is_none() || is_self_type(&receiver.ty)
}

fn generics_for_method(generics: &Generics) -> proc_macro2::TokenStream {
    let mut generics = generics.params.iter().filter_map(|generic| match generic {
        GenericParam::Lifetime(_) => None,
        GenericParam::Const(const_generic) => Some(&const_generic.ident),
        GenericParam::Type(type_generic) => Some(&type_generic.ident),
    });
    let Some(first) = generics.next() else {
        return proc_macro2::TokenStream::new();
    };
    let mut res = quote! {::<#first};
    for generic in generics {
        quote! {, #generic}.to_tokens(&mut res);
    }
    quote! {>}.to_tokens(&mut res);
    res
}

fn create_trait_item_macro(
    trait_name: &Ident,
    trait_generic: &TypeGenerics,
    method: &TraitItemFn,
    long_form: bool,
) -> proc_macro2::TokenStream {
    let TraitItemFn {
        attrs,
        sig,
        default: _,
        semi_token: _,
    } = method;

    let name = &sig.ident;

    if is_valid_self(sig.inputs.first()).not() {
        return Error::new_spanned(
            method,
            "Only methods with `self`, `&self` or `&mut self` are supported",
        )
        .to_compile_error();
    }

    let suffix = match sig.asyncness.is_some() {
        false => quote! {},
        true => quote! { .await },
    };

    if let ReturnType::Type(_, ty) = &sig.output
        && let Type::ImplTrait(impl_trait) = ty.as_ref()
    {
        return Error::new_spanned(impl_trait, "Return impl is not supported").to_compile_error();
    }

    let remaining_inputs = sig.inputs.iter().skip(1).map(|arg| match arg {
        FnArg::Receiver(rec) => {
            Error::new_spanned(rec, "Self only as first argument please").to_compile_error()
        }
        FnArg::Typed(typed) => {
            let name = typed.pat.as_ref();
            quote! { , #name }
        }
    });

    let generics = generics_for_method(&sig.generics);

    let trait_type = match long_form {
        false => quote! { #trait_name #trait_generic },
        true => quote! { $trait_type },
    };

    quote! {
        #(#attrs)* #sig {
            match self {
                $(
                    Self::$variant_name(__static_dispatch_value) => <$variant_type as #trait_type>::#name #generics(
                        __static_dispatch_value
                        #(#remaining_inputs)*
                    )#suffix,
                )*
            }
        }
    }
}

fn macro_name(ident: &Ident) -> Ident {
    format_ident!("{}_static_dispatch_macro", ident)
}

fn dispatch_trait(attr: TokenStream, input: ItemTrait) -> proc_macro2::TokenStream {
    let export = if attr.is_empty() {
        false
    } else {
        let ident = match syn::parse::<Ident>(attr) {
            Ok(ident) => ident,
            Err(err) => return err.to_compile_error(),
        };
        if ident != "macro_export" {
            return Error::new_spanned(&ident, "Only \"macro_export\" is allowed as attribute.")
                .to_compile_error();
        }
        true
    };

    let trait_name = &input.ident;
    let macro_name = macro_name(trait_name);
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let short_items = input.items.iter().map(|item| match item {
        TraitItem::Fn(method) => create_trait_item_macro(trait_name, ty_generics, method, false),
        item => Error::new_spanned(item, "Only methods are supported").to_compile_error(),
    });

    let long_items = input.items.iter().map(|item| match item {
        TraitItem::Fn(method) => create_trait_item_macro(trait_name, ty_generics, method, true),
        item => Error::new_spanned(item, "Only methods are supported").to_compile_error(),
    });

    let export_prefix = match export {
        false => quote! {},
        true => quote! { #[macro_export] },
    };

    let visibility = &input.vis;
    let use_statement = match export {
        false => quote! { #visibility use #macro_name; },
        true => quote! {},
    };

    quote! {
        /// This is just the macro static dispatch uses to create the implementation for the enum.
        #export_prefix
        macro_rules! #macro_name {
            (
                short
                $vis:vis enum $name:ident {
                    $($variant_name:ident($variant_type:ty),)*
                }
            ) => {
                impl #impl_generics #trait_name #ty_generics for $name #where_clause {
                    #(#short_items)*
                }
            };
            (
                long
                $trait_type:ty
                {
                    $($variant_name:ident($variant_type:ty),)*
                }
                $($rem:tt)*
            ) => {
                $($rem)* {
                    #(#long_items)*
                }
            };
        }
        #use_statement
    }
}

fn edit_trait_path(trait_path: &mut Path) -> Result<(), proc_macro2::TokenStream> {
    match trait_path.segments.last_mut() {
        Some(segment) => {
            segment.ident = macro_name(&segment.ident);
            segment.arguments = PathArguments::None;
            Ok(())
        }
        None => Err(
            Error::new_spanned(trait_path, "Name or Path of the trait required").to_compile_error(),
        ),
    }
}

struct LongImpl {
    _impl: Token![impl],
    generics: Generics,
    trait_: Path,
    _for: Token![for],
    self_ty: Type,
}

impl Parse for LongImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _impl: input.parse()?,
            generics: input.parse()?,
            trait_: input.parse()?,
            _for: input.parse()?,
            self_ty: input.parse()?,
        })
    }
}

fn dispatch_enum(attr: TokenStream, input: ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let vis = &input.vis;
    let variants = input.variants.iter();

    let attr = proc_macro2::TokenStream::from(attr);

    if let Ok(mut trait_path) = syn::parse2::<Path>(attr.clone()) {
        if let Err(err) = edit_trait_path(&mut trait_path) {
            return err;
        }
        return quote! {
            #trait_path! {
                short
                #vis enum #enum_name {
                    #(#variants,)*
                }
            }
        };
    }

    let item_impl = match syn::parse2::<LongImpl>(attr) {
        Ok(item_impl) => item_impl,
        Err(err) => return err.into_compile_error(),
    };

    let mut trait_path = item_impl.trait_.clone();
    if let Err(err) = edit_trait_path(&mut trait_path) {
        return err;
    }

    let (impl_generics, _ty_generics, where_clause) = item_impl.generics.split_for_impl();
    let trait_name = &item_impl.trait_;
    let name = item_impl.self_ty;

    quote! {

        #trait_path! {
            long
            #trait_name
            {
                #(#variants,)*
            }
            impl #impl_generics #trait_name for #name #where_clause
        }
    }
}
