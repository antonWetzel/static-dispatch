#[static_dispatch::setup]
pub trait ExternBehavior {
    fn something(&self);
}
