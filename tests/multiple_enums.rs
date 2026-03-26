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
enum First {
    A(A),
    B(B),
}

static_dispatch::implementation!(SomethingBehavior for First);

#[static_dispatch::setup]
enum Second {
    A(A),
    B(B),
}

static_dispatch::implementation!(SomethingBehavior for Second);

#[test]
fn multiple_enums() {
    let mut first = First::A(A);
    first.something();
    first = First::B(B);
    first.something();

    let mut second = Second::A(A);
    second.something();
    second = Second::B(B);
    second.something();
}
