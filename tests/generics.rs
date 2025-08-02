#[static_dispatch::dispatch]
trait SomethingBehavior<A, B: Copy>
where
    A: Copy,
{
    fn something(&self, _a: A, _b: B);
    fn another<C, D: Copy>(&self, _a: A, _b: B, _c: C, _d: D)
    where
        C: Copy;
}

struct StructA;
impl<A, B: Copy> SomethingBehavior<A, B> for StructA
where
    A: Copy,
{
    fn something(&self, _a: A, _b: B) {}
    fn another<C, D: Copy>(&self, _a: A, _b: B, _c: C, _d: D)
    where
        C: Copy,
    {
    }
}

struct StructB;
impl<A, B: Copy> SomethingBehavior<A, B> for StructB
where
    A: Copy,
{
    fn something(&self, _a: A, _b: B) {}
    fn another<C, D: Copy>(&self, _a: A, _b: B, _c: C, _d: D)
    where
        C: Copy,
    {
    }
}

#[static_dispatch::dispatch(SomethingBehavior)]
enum Something {
    A(StructA),
    B(StructB),
}

#[test]
fn generics() {
    let mut something = Something::A(StructA);
    something.something((), ());
    something.another((), (), (), ());
    something = Something::B(StructB);
    something.something((), ());
    something.another((), (), (), ());
}

// todo: impl for SomethingBehavior<(), i32> ...
