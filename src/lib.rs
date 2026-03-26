#![doc = include_str!("../readme.md")]
#![no_std]

pub use static_dispatch_macros::*;

#[doc(hidden)]
#[macro_export]
macro_rules! dispatch_helper {
    (__arg, $target:path, ($macro:path, $($macros:path,)*), $args:tt) => {
        $macro!($crate::dispatch_helper, (__ret, $target, ($($macros,)*), $args));
    };

    ((__ret, $target:path, $macros:tt, ($($args:tt,)*)), $arg:tt) => {
        $crate::dispatch_helper!(__arg, $target, $macros, ($($args,)* $arg,));
    };

    (__arg, $target:path, (), $args:tt) => {
        $target! $args;
    };
}
