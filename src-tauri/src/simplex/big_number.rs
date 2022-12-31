use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use derive_more::Display;
use num_traits::{One, Zero};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Default,
    Display,
    derive_more::Add,
    derive_more::Sub,
    derive_more::AddAssign,
    derive_more::SubAssign,
)]
#[display(
    fmt = "{}{}{}",
    r#"if big_part != &0. && big_part != &1. && big_part != &-1. { 
        std::borrow::Cow::Owned(format!("{}M", big_part))
    } else if big_part == &1. { 
        std::borrow::Cow::Borrowed("M")
    } else if big_part == &-1. { 
        std::borrow::Cow::Borrowed("-M")
    } else { 
        std::borrow::Cow::Borrowed("")
    }"#,
    r#"if small_part > &0. && big_part != &0. { "+" } else { "" } "#,
    r#"if small_part != &0. || big_part == &0. { 
        std::borrow::Cow::Owned(small_part.to_string()) 
    } else { std::borrow::Cow::Borrowed("") }"#
)]
pub struct BigNumber {
    big_part: f64,
    small_part: f64,
}

impl BigNumber {
    #[inline(always)]
    pub const fn new(big_part: f64, small_part: f64) -> Self {
        Self {
            big_part,
            small_part,
        }
    }

    pub const fn one_big() -> Self {
        Self {
            big_part: 1.0,
            small_part: 0.0,
        }
    }

    pub fn total_cmp(&self, other: &Self) -> Ordering {
        match self.big_part.total_cmp(&other.big_part) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.small_part.total_cmp(&other.small_part)
    }

    pub const fn big_part(&self) -> f64 {
        self.big_part
    }

    pub const fn small_part(&self) -> f64 {
        self.small_part
    }
}

impl TryFrom<BigNumber> for f64 {
    type Error = String;

    fn try_from(value: BigNumber) -> Result<Self, Self::Error> {
        if value.big_part != 0. {
            return Err(format!("The number is too big: {value}"));
        }
        Ok(value.small_part)
    }
}

macro_rules! impl_from_number {
    ($($num: ident),+ $(,)?) => {
        $(
            impl From<$num> for BigNumber {
                #[inline]
                fn from(val: $num) -> Self {
                    Self {
                        big_part: 0.0,
                        small_part: val as f64
                    }
                }
            }

            impl Add<$num> for BigNumber {
                type Output = Self;

                fn add(self, other: $num) -> Self {
                    Self {
                        big_part: self.big_part,
                        small_part: self.small_part + other as f64
                    }
                }
            }

            impl AddAssign<$num> for BigNumber {
                fn add_assign(&mut self, other: $num) {
                    self.small_part += other as f64;
                }
            }

            impl Sub<$num> for BigNumber {
                type Output = Self;

                fn sub(self, other: $num) -> Self {
                    Self {
                        big_part: self.big_part,
                        small_part: self.small_part - other as f64
                    }
                }
            }

            impl SubAssign<$num> for BigNumber {
                fn sub_assign(&mut self, other: $num) {
                    self.small_part -= other as f64;
                }
            }

            impl Mul<$num> for BigNumber {
                type Output = Self;

                fn mul(self, other: $num) -> Self {
                    Self {
                        big_part: self.big_part * other as f64,
                        small_part: self.small_part * other as f64
                    }
                }
            }

            impl MulAssign<$num> for BigNumber {
                fn mul_assign(&mut self, other: $num) {
                    self.big_part *= other as f64;
                    self.small_part *= other as f64;
                }
            }

            impl Div<$num> for BigNumber {
                type Output = Self;

                fn div(self, other: $num) -> Self {
                    Self {
                        big_part: self.big_part / other as f64,
                        small_part: self.small_part / other as f64
                    }
                }
            }

            impl DivAssign<$num> for BigNumber {
                fn div_assign(&mut self, other: $num) {
                    self.big_part /= other as f64;
                    self.small_part /= other as f64;
                }
            }
        )+
    };
}

impl_from_number!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

impl Neg for BigNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            big_part: -self.big_part,
            small_part: -self.small_part,
        }
    }
}

impl Zero for BigNumber {
    fn zero() -> Self {
        Self {
            big_part: 0.,
            small_part: 0.,
        }
    }

    fn is_zero(&self) -> bool {
        self.big_part == 0. && self.small_part == 0.
    }
}

impl One for BigNumber {
    fn one() -> Self {
        Self {
            big_part: 0.,
            small_part: 1.,
        }
    }
}

/// It won't be used for multiplication of two big numbers with big parts both
impl Mul for BigNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // (ai + b) * (xi + y) = axii + ayi + bxi + by = axii + (ay + bx)i + by
        Self {
            big_part: /* self.big_part * rhs.big_part */
                self.big_part * rhs.small_part
                + self.small_part * rhs.big_part,
            small_part: self.small_part * rhs.small_part,
        }
    }
}

impl MulAssign for BigNumber {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
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
            let a = BigNumber {
                big_part: big_part1,
                small_part: small_part1,
            };
            let b = BigNumber {
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
            let num = BigNumber { big_part, small_part };

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
            let num = BigNumber { big_part: 0., small_part };

            prop_assert_eq!(
                small_part.to_string(),
                num.to_string()
            )
        }

        #[test]
        fn display_big_one(
            small_part in -1000.0..1000.0,
        ) {
            let num = BigNumber { big_part: 1., small_part };

            prop_assert_eq!(
                format!("M{}{small_part}", if small_part > 0. { "+" } else { "" }),
                num.to_string()
            )
        }

        #[test]
        fn display_big_minus_one(
            small_part in -1000.0..1000.0,
        ) {
            let num = BigNumber { big_part: -1., small_part };

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
            let num = BigNumber { big_part, small_part };

            prop_assert_eq!(
                num + rhs,
                BigNumber {
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
            let num = BigNumber { big_part, small_part };

            prop_assert_eq!(
                num - rhs,
                BigNumber {
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
            let num = BigNumber { big_part, small_part };

            prop_assert_eq!(
                num * rhs,
                BigNumber {
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
            let num = BigNumber { big_part, small_part };

            prop_assert_eq!(
                num / rhs,
                BigNumber {
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
            let mut num = BigNumber { big_part, small_part };
            num += rhs;
            prop_assert_eq!(
                num,
                BigNumber {
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
            let mut num = BigNumber { big_part, small_part };
            num -= rhs;
            prop_assert_eq!(
                num,
                BigNumber {
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
            let mut num = BigNumber { big_part, small_part };
            num *= rhs;
            prop_assert_eq!(
                num,
                BigNumber {
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
            let mut num = BigNumber { big_part, small_part };
            num /= rhs;
            prop_assert_eq!(
                num,
                BigNumber {
                    big_part: big_part / rhs,
                    small_part: small_part / rhs,
                }
            )
        }
    }
}
