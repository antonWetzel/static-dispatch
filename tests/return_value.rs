#[static_dispatch::setup]
trait SomethingBehavior {
    fn something(&self) -> i32;
}

struct A;
impl SomethingBehavior for A {
    fn something(&self) -> i32 {
        0
    }
}

struct B;
impl SomethingBehavior for B {
    fn something(&self) -> i32 {
        1
    }
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
    assert_eq!(something.something(), 0);
    something = Something::B(B);
    assert_eq!(something.something(), 1);
}
