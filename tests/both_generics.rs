#![expect(unused)]

#[static_dispatch::setup]
trait SomethingBehavior<'a, T, const X: usize> {
    fn something(&self);
}

struct A<'a>(core::marker::PhantomData<&'a ()>);

impl<'a, T, const X: usize> SomethingBehavior<'a, T, X> for A<'a> {
    fn something(&self) {}
}

struct B;
impl<'a> SomethingBehavior<'a, (), 0> for B {
    fn something(&self) {}
}

#[static_dispatch::setup]
enum Something<'a> {
    A(A<'a>),
    B(B),
}

// todo: conflict with generics (rename or merge?)
// todo: implement with specific generics/where
// static_dispatch::implementation!(SomethingBehavior for Something);

// #[test]
// fn generics() {
//     let mut something = Something::A(A(core::marker::PhantomData));
//     something.something();
//     something = Something::B(B);
//     something.something();
// }
