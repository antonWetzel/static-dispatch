#[static_dispatch::dispatch]
#[allow(async_fn_in_trait)]
trait AsyncBehavior {
    async fn something(&self) -> i32;
}

struct A;
impl AsyncBehavior for A {
    async fn something(&self) -> i32 {
        0
    }
}

struct B;
impl AsyncBehavior for B {
    async fn something(&self) -> i32 {
        1
    }
}

#[static_dispatch::dispatch(AsyncBehavior)]
enum Something {
    A(A),
    B(B),
}

#[pollster::test]
async fn async_trait() {
    let mut something = Something::A(A);
    assert_eq!(something.something().await, 0);
    something = Something::B(B);
    assert_eq!(something.something().await, 1);
}
