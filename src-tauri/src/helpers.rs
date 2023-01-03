#[macro_export]
macro_rules! ensure_eq {
    ($expr1: expr, $expr2: expr) => {
        if $expr1 != $expr2 {
            return Err(format!(
                "{0} != {1}\n{0} = {2:?}\n{1} = {3:?}",
                stringify!($expr1),
                stringify!($expr2),
                $expr1,
                $expr2
            ))?;
        }
    };
}

#[macro_export]
macro_rules! dbg_display {
    ($e: expr) => {{
        let val = $e;
        log::debug!(
            "[{}/{}:{}] {} = {}",
            file!(),
            line!(),
            column!(),
            stringify!($e),
            val
        );
        val
    }};
}
