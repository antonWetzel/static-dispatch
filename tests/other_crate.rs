use extern_crate::ExternBehavior;

struct A;
impl ExternBehavior for A {
    fn something(&self) {}
}

struct B;
impl ExternBehavior for B {
    fn something(&self) {}
}

#[static_dispatch::setup]
enum Something {
    A(A),
    B(B),
}

static_dispatch::implementation!(extern_crate::ExternBehavior for Something);

#[test]
fn other_module() {
    let mut something = Something::A(A);
    something.something();
    something = Something::B(B);
    something.something();
}
