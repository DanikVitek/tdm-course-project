mod ratio_ext;

pub use ratio_ext::*;

#[macro_export]
macro_rules! ensure_eq {
    ($expr1: expr, $expr2: expr) => {
        if $expr1 != $expr2 {
            return Err(format!(
                "{0} != {1}\n{0} = {2:?}\n{1} = {3:?}", 
                stringify!($expr1), stringify!($expr2), $expr1, $expr2
            ))?
        }
    };
}