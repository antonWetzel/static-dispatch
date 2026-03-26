#[static_dispatch::setup]
trait SomethingBehavior<const X: usize> {
    fn something(&self);
    fn another<const Y: usize>(&self);
}

struct A;
impl<const X: usize> SomethingBehavior<X> for A {
    fn something(&self) {}
    fn another<const Y: usize>(&self) {}
}

struct B;
impl<const X: usize> SomethingBehavior<X> for B {
    fn something(&self) {}
    fn another<const Y: usize>(&self) {}
}

#[static_dispatch::setup]
enum Something {
    A(A),
    B(B),
}

static_dispatch::implementation!(SomethingBehavior for Something);

#[test]
fn generics() {
    let mut something = Something::A(A);
    <Something as SomethingBehavior<0>>::something(&something);
    <Something as SomethingBehavior<0>>::another::<0>(&something);
    something = Something::B(B);
    <Something as SomethingBehavior<0>>::something(&something);
    <Something as SomethingBehavior<0>>::another::<0>(&something);
}
