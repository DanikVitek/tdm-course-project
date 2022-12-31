#[macro_export]
macro_rules! reclone {
    ($($clonable: ident),+ $(,)?) => {
        $(
            let $clonable = $clonable.clone();
        )+
    };
}

pub fn f32_rounded_string(val: &f32, precision: usize) -> String {
    format!("{val:.precision$}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

pub fn f64_rounded_string(val: &f64, precision: usize) -> String {
    format!("{val:.precision$}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}
