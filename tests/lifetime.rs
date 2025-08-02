#[static_dispatch::dispatch]
trait SomethingBehavior<'a> {
    fn something(&'a self);
    fn another<'b>(&'a self, _value: &'b ());
}

struct A;
impl<'a> SomethingBehavior<'a> for A {
    fn something(&'a self) {}
    fn another<'b>(&'a self, _value: &'b ()) {}
}

struct B;
impl<'a> SomethingBehavior<'a> for B {
    fn something(&'a self) {}
    fn another<'b>(&'a self, _value: &'b ()) {}
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
    something.another(&());
    something = Something::B(B);
    something.something();
    something.another(&());
}
