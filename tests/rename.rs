#[static_dispatch::setup]
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
