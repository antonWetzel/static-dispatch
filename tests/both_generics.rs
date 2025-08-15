#[static_dispatch::dispatch]
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

#[static_dispatch::dispatch(impl<'a> SomethingBehavior<'a, (), 0> for Something<'a>)]
enum Something<'a> {
    A(A<'a>),
    B(B),
}

#[test]
fn generics() {
    let mut something = Something::A(A(core::marker::PhantomData));
    something.something();
    something = Something::B(B);
    something.something();
}
