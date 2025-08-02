# Static Dispatch

Implement a trait for an enum, where all variants implement the trait.

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

## Supported

- Async methods
- Trait and enum in different modules or crates
- Trait with generics, lifetimes or const generics.
- `no_std` support
- Implement trait with multiple enums
- Implement multiple traits for an enum

## Not Supported

- Use a type alias for the trait
- Concrete implementation for a generic trait
- Traits with items, which are not function with `self`, `&self` or `&mut self`
- `impl` in return position

## Comparison to `enum_dispatch`

- This crate only generates the trait implementation for the enum, nothing more
- Data between the trait and the enum is passed with a `macro_rule`, not as a side effect in memory
