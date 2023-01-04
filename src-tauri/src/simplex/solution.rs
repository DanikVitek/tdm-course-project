use ratio_extension::BigRationalExt;
use std::borrow::Cow;
use std::fmt;

use derive_more::IsVariant;

#[derive(Debug, Clone, PartialEq, IsVariant)]
pub enum Solution {
    Finite {
        fn_val: BigRationalExt,
        vars: Vec<BigRationalExt>,
    },
    Infinite,
    Absent,
}

impl Solution {
    pub fn unwrap_finite(self) -> (BigRationalExt, Vec<BigRationalExt>) {
        if let Self::Finite { fn_val, vars } = self {
            return (fn_val, vars);
        }
        panic!("Solution is not finite")
    }

    pub fn unwrap_finite_ref(&self) -> (&BigRationalExt, &[BigRationalExt]) {
        if let Self::Finite { fn_val, vars } = self {
            return (fn_val, vars);
        }
        panic!("Solution is not finite")
    }

    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Solution::Finite { fn_val, vars } => format!(
                "Змінні:\n[{}]\nЗначення функції: {fn_val}",
                vars.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )
            .into(),
            Solution::Infinite => "Розв'язок нескінченний".into(),
            Solution::Absent => "Розв'язок відсутній".into(),
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}