use ratio_extension::BigRationalExt;
use std::borrow::Cow;

use derive_more::IsVariant;

#[derive(Debug, Clone, PartialEq, IsVariant)]
pub enum Solution {
    Finite {
        variables: Vec<BigRationalExt>,
        function_value: BigRationalExt,
    },
    Infinite,
    Absent,
}

impl Solution {
    pub fn unwrap_finite(self) -> (Vec<BigRationalExt>, BigRationalExt) {
        if let Self::Finite {
            variables,
            function_value,
        } = self
        {
            return (variables, function_value);
        }
        panic!("Solution is not finite")
    }

    pub fn unwrap_finite_ref(&self) -> (&[BigRationalExt], &BigRationalExt) {
        if let Self::Finite {
            variables,
            function_value,
        } = self
        {
            return (variables, function_value);
        }
        panic!("Solution is not finite")
    }

    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Solution::Finite {
                variables,
                function_value,
            } => format!(
                "Змінні:\n[{}]\nЗначення функції: {function_value}",
                variables
                    .iter()
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
