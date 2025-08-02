#[static_dispatch::dispatch]
trait SomethingBehavior {
    fn something(&self, value: impl Into<i32>);
}

struct A;
impl SomethingBehavior for A {
    fn something(&self, _value: impl Into<i32>) {}
}

struct B;
impl SomethingBehavior for B {
    fn something(&self, _value: impl Into<i32>) {}
}

#[static_dispatch::dispatch(SomethingBehavior)]
enum Something {
    A(A),
    B(B),
}

#[test]
fn rename() {
    let mut something = Something::A(A);
    something.something(0i16);
    something.something(0i32);
    something = Something::B(B);
    something.something(0i16);
    something.something(0i32);
}
