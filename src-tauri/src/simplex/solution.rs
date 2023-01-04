use ratio_extension::BigRationalExt;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    pub fn_val: BigRationalExt,
    pub vars: Vec<BigRationalExt>,
}

pub type SolutionResult = Result<Solution, SolutionError>;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Error,
    derive_more::IsVariant,
)]
pub enum SolutionError {
    #[display(fmt = "Розв'язок нескінченний")]
    Infinite,
    #[display(fmt = "Розв'язок відсутній")]
    Absent,
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Змінні:\n[{}]\nЗначення функції: {}",
            self.vars
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.fn_val
        )
    }
}
