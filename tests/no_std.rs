#![no_std]

#[static_dispatch::dispatch]
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

#[static_dispatch::dispatch(SomethingBehavior)]
enum Something {
    A(A),
    B(B),
}

#[test]
fn rename() {
    let mut something = Something::A(A);
    something.something();
    something = Something::B(B);
    something.something();
}
