#[static_dispatch::dispatch]
trait FirstBehavior {
    fn first(&self);
}

#[static_dispatch::dispatch]
trait SecondBehavior {
    fn second(&self);
}

struct A;
impl FirstBehavior for A {
    fn first(&self) {}
}
impl SecondBehavior for A {
    fn second(&self) {}
}

struct B;
impl FirstBehavior for B {
    fn first(&self) {}
}
impl SecondBehavior for B {
    fn second(&self) {}
}

#[static_dispatch::dispatch(FirstBehavior)]
#[static_dispatch::dispatch(SecondBehavior)]
enum Other {
    A(A),
    B(B),
}

#[test]
fn multiple_traits() {
    let mut other = Other::A(A);
    other.first();
    other.second();
    other = Other::B(B);
    other.first();
    other.second();
}
