#[static_dispatch::setup]
trait ExplicitSelfBehavior {
    fn as_value(self: Self);
    fn as_ref(self: &Self);
    fn as_mut(self: &mut Self);
}

struct A;
impl ExplicitSelfBehavior for A {
    fn as_value(self) {}
    fn as_ref(&self) {}
    fn as_mut(&mut self) {}
}

struct B;
impl ExplicitSelfBehavior for B {
    fn as_value(self: Self) {}
    fn as_ref(self: &Self) {}
    fn as_mut(self: &mut Self) {}
}

#[static_dispatch::setup]
enum ImplicitSelf {
    A(A),
    B(B),
}

static_dispatch::implementation!(ExplicitSelfBehavior for ImplicitSelf);

#[test]
fn explicit_self() {
    let mut value = ImplicitSelf::A(A);
    value.as_mut();
    value.as_ref();
    value.as_value();
    value = ImplicitSelf::B(B);
    value.as_mut();
    value.as_ref();
    value.as_value();
}
