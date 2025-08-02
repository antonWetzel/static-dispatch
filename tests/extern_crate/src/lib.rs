#[static_dispatch::dispatch(macro_export)]
pub trait ExternBehavior {
    fn something(&self);
}
