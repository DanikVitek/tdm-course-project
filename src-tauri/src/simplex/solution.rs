use ratio_extension::BigRationalExt;
use std::borrow::Cow;

use derive_more::IsVariant;
use nalgebra::DVector;

#[derive(Debug, Clone, PartialEq, IsVariant)]
pub enum Solution {
    Finite {
        variables: DVector<BigRationalExt>,
        function_value: BigRationalExt,
    },
    Infinite,
    Absent,
}

impl Solution {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Solution::Finite {
                variables,
                function_value,
            } => format!("Змінні:\n{variables}\nЗначення функції: {function_value}").into(),
            Solution::Infinite => "Розв'язок нескінченний".into(),
            Solution::Absent => "Розв'язок відсутній".into(),
        }
    }
}
