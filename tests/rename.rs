#[static_dispatch::dispatch]
trait SomethingBehavior {
    fn something(&self);
}

use SomethingBehavior as OtherBehavior;

struct A;
impl OtherBehavior for A {
    fn something(&self) {}
}

struct B;
impl OtherBehavior for B {
    fn something(&self) {}
}

// only original name works
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
