#[static_dispatch::setup]
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

#[static_dispatch::setup]
enum Something<'a> {
    A(A<'a>),
    B(B),
}

static_dispatch::implementation!(SomethingBehavior for Something);

#[test]
fn enum_generics() {
    let mut something = Something::A(A(core::marker::PhantomData));
    something.something();
    something = Something::B(B);
    something.something();
}
