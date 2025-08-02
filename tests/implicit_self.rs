#[static_dispatch::dispatch]
trait ImplicitSelfBehavior {
    fn as_value(self);
    fn as_ref(&self);
    fn as_mut(&mut self);
}

struct A;
impl ImplicitSelfBehavior for A {
    fn as_value(self) {}
    fn as_ref(&self) {}
    fn as_mut(&mut self) {}
}

struct B;
impl ImplicitSelfBehavior for B {
    fn as_value(self: Self) {}
    fn as_ref(self: &Self) {}
    fn as_mut(self: &mut Self) {}
}

#[static_dispatch::dispatch(ImplicitSelfBehavior)]
enum ImplicitSelf {
    A(A),
    B(B),
}

#[test]
fn implicit_self() {
    let mut value = ImplicitSelf::A(A);
    value.as_mut();
    value.as_ref();
    value.as_value();
    value = ImplicitSelf::B(B);
    value.as_mut();
    value.as_ref();
    value.as_value();
}
