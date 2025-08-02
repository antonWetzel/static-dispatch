use extern_crate::ExternBehavior;

struct A;
impl ExternBehavior for A {
    fn something(&self) {}
}

struct B;
impl ExternBehavior for B {
    fn something(&self) {}
}

#[static_dispatch::dispatch(extern_crate::ExternBehavior)]
enum Something {
    A(A),
    B(B),
}

#[test]
fn other_module() {
    let mut something = Something::A(A);
    something.something();
    something = Something::B(B);
    something.something();
}
