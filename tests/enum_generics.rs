#[static_dispatch::dispatch]
trait SomethingBehavior {
    fn something(&self);
}

struct A<'a>(core::marker::PhantomData<&'a ()>);

impl<'a> SomethingBehavior for A<'a> {
    fn something(&self) {}
}

struct B;
impl SomethingBehavior for B {
    fn something(&self) {}
}

#[static_dispatch::dispatch(impl<'a> SomethingBehavior for Something<'a>)]
enum Something<'a> {
    A(A<'a>),
    B(B),
}

#[test]
fn enum_generics() {
    let mut something = Something::A(A(core::marker::PhantomData));
    something.something();
    something = Something::B(B);
    something.something();
}
