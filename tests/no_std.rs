#![no_std]

#[static_dispatch::setup]
trait SomethingBehavior {
    fn something(&self);
}

struct A;
impl SomethingBehavior for A {
    fn something(&self) {}
}

struct B;
impl SomethingBehavior for B {
    fn something(&self) {}
}

#[static_dispatch::setup]
enum Something {
    A(A),
    B(B),
}

static_dispatch::implementation!(SomethingBehavior for Something);

#[test]
fn rename() {
    let mut something = Something::A(A);
    something.something();
    something = Something::B(B);
    something.something();
}
