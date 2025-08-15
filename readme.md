# Static Dispatch

Derive a trait for an enum, where all variants implement the trait.

```rust
#[static_dispatch::dispatch]
trait ExampleBehavior {
    fn as_value(self);
    fn as_ref(&self) {}
    fn as_mut(&mut self) {}
    async fn async_fn(&self) {}
    fn generic<T>(&self, value: T) {}
    fn impl_arg(&self, value: impl Into<i32>) {}
}

struct A;
impl ExampleBehavior for A {
    fn as_value(self) {}
    // ...
}

struct B;
impl ExampleBehavior for B {
    fn as_value(self) {}
    // ...
}

#[static_dispatch::dispatch(ExampleBehavior)]
enum Example {
    A(A),
    B(B),
}

async fn example() {
    for mut value in [
        Example::A(A),
        Example::B(B),
    ] {
        value.as_mut();
        value.as_ref();
        value.async_fn().await;
        value.generic(0);
        value.generic(Some(0));
        value.impl_arg(0i16);
        value.impl_arg(0i32);
        value.as_value();
    }
}
```

## Overview

- Annotate the trait with `static_dispatch::dispatch`
    - This generates a macro with `macro_rules`
    - Use `static_dispatch::dispatch(macro_export)` to export the macro 
- Annotate the struct with `static_dispatch::dispatch(<TraitName>)`
    - This invokes the macro to generate the trait implementation
    - Use `static_dispatch::dispatch(<crate>::<TraitName>)` for use with `macro_export`

## Supported

- Async methods
- Trait and enum in different modules or crates
- Trait with generics, lifetimes or const generics.
- `no_std` support
- Implement trait with multiple enums
- Implement multiple traits for an enum

## Example for generics

```rust
#[static_dispatch::dispatch]
trait SomethingBehavior<V> {
    fn something(&self, value: V);
}

struct A<'a>(&'a i32);

impl<'a, V> SomethingBehavior<V> for A<'a> {
    fn something(&self, _value: V) {}
}

struct B<T>(T);
impl<T, V> SomethingBehavior<V> for B<T> {
    fn something(&self, _value: V) {}
}

#[static_dispatch::dispatch(impl<'a, T, V> SomethingBehavior<V> for Something<'a, T>)]
enum Something<'a, T> {
    A(A<'a>),
    B(B<T>),
}

#[test]
fn generic_example() {
    let mut something = Something::A(A(&0));
    something.something();
    something = Something::B(B(0));
    something.something();
}
```

## Not Supported

- Use a type alias for the trait
- Traits with items, which are not function with `self`, `&self` or `&mut self`
- `impl` in return position

## Comparison to `enum_dispatch`

- This crate only generates the trait implementation for the enum, nothing more
- Data between the trait and the enum is passed with a `macro_rule`, not as a side effect in memory
