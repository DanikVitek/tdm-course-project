use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num_rational::BigRational;
use num_traits::{One, Zero};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Default,
    derive_more::Add,
    derive_more::Sub,
    derive_more::AddAssign,
    derive_more::SubAssign,
)]
// #[display(
//     fmt = "{}{}{}",
//     r#"if big_part != &0. && big_part != &1. && big_part != &-1. {
//         std::borrow::Cow::Owned(format!("{}M", big_part))
//     } else if big_part == &1. {
//         std::borrow::Cow::Borrowed("M")
//     } else if big_part == &-1. {
//         std::borrow::Cow::Borrowed("-M")
//     } else {
//         std::borrow::Cow::Borrowed("")
//     }"#,
//     r#"if small_part > &0. && big_part != &0. { "+" } else { "" } "#,
//     r#"if small_part != &0. || big_part == &0. {
//         std::borrow::Cow::Owned(small_part.to_string())
//     } else { std::borrow::Cow::Borrowed("") }"#
// )]
pub struct BigNumber<T> {
    big_part: T,
    small_part: T,
}

impl<'a, T: 'a> fmt::Display for BigNumber<T>
where
    T: Zero + One + PartialEq + PartialOrd + Neg<Output = T> + Clone + fmt::Display,
    &'a T: fmt::Display + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let big_part = self.big_part.clone();
        let small_part = self.small_part.clone();
        write!(
            f,
            "{}{}{}",
            if !big_part.is_zero() && !big_part.is_one() && !(-(big_part.to_owned())).is_one() {
                std::borrow::Cow::Owned(format!("{big_part}M"))
            } else if big_part.is_one() {
                std::borrow::Cow::Borrowed("M")
            } else if (-(big_part.to_owned())).is_one() {
                std::borrow::Cow::Borrowed("-M")
            } else {
                std::borrow::Cow::Borrowed("")
            },
            if small_part > T::zero() && !big_part.is_zero() {
                "+"
            } else {
                ""
            },
            if !small_part.is_zero() || big_part.is_zero() {
                std::borrow::Cow::Owned(format!("{small_part}"))
            } else {
                std::borrow::Cow::Borrowed("")
            }
        )
    }
}

impl<T> BigNumber<T> {
    #[inline(always)]
    pub const fn new(big_part: T, small_part: T) -> Self {
        Self {
            big_part,
            small_part,
        }
    }

    pub const fn big_part(&self) -> &T {
        &self.big_part
    }

    pub const fn small_part(&self) -> &T {
        &self.small_part
    }
}

impl<T: One + Zero> BigNumber<T> {
    pub fn one_big() -> Self {
        Self {
            big_part: T::one(),
            small_part: T::zero(),
        }
    }
}

impl BigNumber<f64> {
    pub fn total_cmp(&self, other: &Self) -> Ordering {
        match self.big_part.total_cmp(&other.big_part) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.small_part.total_cmp(&other.small_part)
    }
}

impl TryFrom<BigNumber<BigRational>> for BigRational {
    type Error = String;

    fn try_from(value: BigNumber<BigRational>) -> Result<Self, Self::Error> {
        if !value.big_part.is_zero() {
            return Err(format!("The number is too big: {value}"));
        }
        Ok(value.small_part)
    }
}

impl TryFrom<BigNumber<f64>> for f64 {
    type Error = String;

    fn try_from(value: BigNumber<f64>) -> Result<Self, Self::Error> {
        if value.big_part != 0. {
            return Err(format!("The number is too big: {value}"));
        }
        Ok(value.small_part)
    }
}

impl<T> From<T> for BigNumber<T>
where
    T: Zero,
{
    #[inline]
    fn from(val: T) -> Self {
        Self {
            big_part: T::zero(),
            small_part: val,
        }
    }
}

macro_rules! impl_from_number {
    ($($num: ident),+ $(,)?) => {
        $(
            impl<T> Add<$num> for BigNumber<T>
            where
                T: Add<T, Output = T> + From<$num>
            {
                type Output = Self;

                fn add(self, other: $num) -> Self {
                    Self {
                        big_part: self.big_part,
                        small_part: self.small_part + T::from(other)
                    }
                }
            }

            impl<T> AddAssign<$num> for BigNumber<T>
            where
                T: AddAssign<T> + From<$num>
            {
                fn add_assign(&mut self, other: $num) {
                    self.small_part += T::from(other);
                }
            }

            impl<T> Sub<$num> for BigNumber<T>
            where
                T: Sub<T, Output = T> + From<$num>
            {
                type Output = Self;

                fn sub(self, other: $num) -> Self {
                    Self {
                        big_part: self.big_part,
                        small_part: self.small_part - T::from(other)
                    }
                }
            }

            impl<T> SubAssign<$num> for BigNumber<T>
            where
                T: SubAssign<T> + From<$num>
            {
                fn sub_assign(&mut self, other: $num) {
                    self.small_part -= T::from(other)
                }
            }

            impl<T> Mul<$num> for BigNumber<T>
            where
                T: Mul<T, Output = T> + From<$num> + Clone
            {
                type Output = Self;

                fn mul(self, other: $num) -> Self {
                    let other = T::from(other);
                    Self {
                        big_part: self.big_part * other.clone(),
                        small_part: self.small_part * other
                    }
                }
            }

            impl<T> MulAssign<$num> for BigNumber<T>
            where
                T: MulAssign<T> + From<$num> + Clone
            {
                fn mul_assign(&mut self, other: $num) {
                    let other = T::from(other);
                    self.big_part *= other.clone();
                    self.small_part *= other;
                }
            }

            impl<T> Div<$num> for BigNumber<T>
            where
                T: Div<T, Output = T> + From<$num> + Clone
            {
                type Output = Self;

                fn div(self, other: $num) -> Self {
                    let other = T::from(other);
                    Self {
                        big_part: self.big_part / other.clone(),
                        small_part: self.small_part / other
                    }
                }
            }

            impl<T> DivAssign<$num> for BigNumber<T>
            where
                T: DivAssign<T> + From<$num> + Clone
            {
                fn div_assign(&mut self, other: $num) {
                    let other = T::from(other);
                    self.big_part /= other.clone();
                    self.small_part /= other;
                }
            }
        )+
    };
}

impl_from_number!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

impl<T> Neg for BigNumber<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            big_part: -self.big_part,
            small_part: -self.small_part,
        }
    }
}

impl<T: Zero> Zero for BigNumber<T> {
    fn zero() -> Self {
        Self {
            big_part: T::zero(),
            small_part: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.big_part.is_zero() && self.small_part.is_zero()
    }
}

impl<T> One for BigNumber<T>
where
    T: One + Zero + Mul<Output = T> + Clone,
{
    fn one() -> Self {
        Self {
            big_part: T::zero(),
            small_part: T::one(),
        }
    }
}

/// It won't be used for multiplication of two big numbers with big parts both
impl<T> Mul for BigNumber<T>
where
    T: Add<Output = T> + Mul<Output = T> + Clone
{
    type Output = BigNumber<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        // (ai + b) * (xi + y) = axii + ayi + bxi + by = axii + (ay + bx)i + by
        BigNumber {
            big_part: /* self.big_part * rhs.big_part */
                self.big_part * rhs.small_part.clone()
                + self.small_part.clone() * rhs.big_part,
            small_part: self.small_part * rhs.small_part,
        }
    }
}

// impl<'a, T: 'a> Mul for BigNumber<T>
// where
//     Self: 'a,
//     T: Add<Output = T>/*  + Mul<Output = T> */,
//     &'a T: Mul<&'a T, Output = T>
// {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         let lhs = &self;
//         let rhs = &rhs;
//         lhs * rhs
//     }
// }

impl<'a, T: 'a> MulAssign for BigNumber<T>
where
    Self: Clone,
    T: Add<Output = T> + Mul<Output = T> + Clone
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs
    }
}

#[cfg(test)]
mod tests {
    use proptest::{prop_assert_eq, proptest};

    use super::*;

    proptest! {
        #[test]
        fn comparison(
            big_part1 in -1000.0..1000.0,
            small_part1 in -1000.0..1000.0,
            big_part2 in -1000.0..1000.0,
            small_part2 in -1000.0..1000.0,
        ) {
            let a = BigNumber::<f64> {
                big_part: big_part1,
                small_part: small_part1,
            };
            let b = BigNumber::<f64> {
                big_part: big_part2,
                small_part: small_part2,
            };

            if big_part1 != big_part2 {
                prop_assert_eq!(
                    big_part1.partial_cmp(&big_part2),
                    a.partial_cmp(&b)
                )
            } else {
                prop_assert_eq!(
                    small_part1.partial_cmp(&small_part2),
                    a.partial_cmp(&b)
                )
            }
        }

        #[test]
        fn display(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
        ) {
            let num = BigNumber::<f64> { big_part, small_part };

            if big_part != 0. && big_part != 1. && big_part != -1. {
                prop_assert_eq!(
                    format!("{big_part}M{}{small_part}", if small_part > 0. { "+" } else { "" }),
                    num.to_string()
                )
            } else if big_part == 1. {
                prop_assert_eq!(
                    format!("M{}{small_part}", if small_part > 0. { "+" } else { "" }),
                    num.to_string()
                )
            } else if big_part == -1. {
                prop_assert_eq!(
                    format!("-M{}{small_part}", if small_part > 0. { "+" } else { "" }),
                    num.to_string()
                )
            } else {
                prop_assert_eq!(
                    small_part.to_string(),
                    num.to_string()
                )
            }
        }

        #[test]
        fn display_big_zero(
            small_part in -1000.0..1000.0,
        ) {
            let num = BigNumber::<f64> { big_part: 0., small_part };

            prop_assert_eq!(
                small_part.to_string(),
                num.to_string()
            )
        }

        #[test]
        fn display_big_one(
            small_part in -1000.0..1000.0,
        ) {
            let num = BigNumber::<f64> { big_part: 1., small_part };

            prop_assert_eq!(
                format!("M{}{small_part}", if small_part > 0. { "+" } else { "" }),
                num.to_string()
            )
        }

        #[test]
        fn display_big_minus_one(
            small_part in -1000.0..1000.0,
        ) {
            let num = BigNumber::<f64> { big_part: -1., small_part };

            prop_assert_eq!(
                format!("-M{}{small_part}", if small_part > 0. { "+" } else { "" }),
                num.to_string()
            )
        }

        #[test]
        fn add(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
            rhs in -1000.0..1000.0,
        ) {
            let num = BigNumber::<f64> { big_part, small_part };

            prop_assert_eq!(
                num + rhs,
                BigNumber::<f64> {
                    big_part,
                    small_part: small_part + rhs,
                }
            )
        }

        #[test]
        fn sub(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
            rhs in -1000.0..1000.0,
        ) {
            let num = BigNumber::<f64> { big_part, small_part };

            prop_assert_eq!(
                num - rhs,
                BigNumber::<f64> {
                    big_part,
                    small_part: small_part - rhs,
                }
            )
        }

        #[test]
        fn mul(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
            rhs in -1000.0..1000.0,
        ) {
            let num = BigNumber::<f64> { big_part, small_part };

            prop_assert_eq!(
                num * rhs,
                BigNumber::<f64> {
                    big_part: big_part * rhs,
                    small_part: small_part * rhs,
                }
            )
        }

        #[test]
        fn div(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
            rhs in -1000.0..1000.0,
        ) {
            let num = BigNumber::<f64> { big_part, small_part };

            prop_assert_eq!(
                num / rhs,
                BigNumber::<f64> {
                    big_part: big_part / rhs,
                    small_part: small_part / rhs,
                }
            )
        }

        #[test]
        fn add_assign(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
            rhs in -1000.0..1000.0,
        ) {
            let mut num = BigNumber::<f64> { big_part, small_part };
            num += rhs;
            prop_assert_eq!(
                num,
                BigNumber::<f64> {
                    big_part,
                    small_part: small_part + rhs,
                }
            )
        }

        #[test]
        fn sub_assign(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
            rhs in -1000.0..1000.0,
        ) {
            let mut num = BigNumber::<f64> { big_part, small_part };
            num -= rhs;
            prop_assert_eq!(
                num,
                BigNumber::<f64> {
                    big_part,
                    small_part: small_part - rhs,
                }
            )
        }

        #[test]
        fn mul_assign(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
            rhs in -1000.0..1000.0,
        ) {
            let mut num = BigNumber::<f64> { big_part, small_part };
            num *= rhs;
            prop_assert_eq!(
                num,
                BigNumber::<f64> {
                    big_part: big_part * rhs,
                    small_part: small_part * rhs,
                }
            )
        }

        #[test]
        fn div_assign(
            big_part in -1000.0..1000.0,
            small_part in -1000.0..1000.0,
            rhs in -1000.0..1000.0,
        ) {
            let mut num = BigNumber::<f64> { big_part, small_part };
            num /= rhs;
            prop_assert_eq!(
                num,
                BigNumber::<f64> {
                    big_part: big_part / rhs,
                    small_part: small_part / rhs,
                }
            )
        }
    }
}
