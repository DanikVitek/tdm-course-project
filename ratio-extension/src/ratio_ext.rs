use std::{
    cmp::Ordering,
    fmt,
    hint::{self, unreachable_unchecked},
    iter::{Product, Sum},
    mem,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use derive_more::IsVariant;
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::{float::FloatCore, FromPrimitive, One, Zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, IsVariant, Serialize, Deserialize)]
pub enum RatioExt<T: Clone + Integer> {
    Inf,
    Finite(Ratio<T>),
    MinusInf,
    Nan,
}

impl<T> Default for RatioExt<T>
where
    T: Clone + Integer,
{
    fn default() -> Self {
        Self::Finite(Default::default())
    }
}

impl<T> RatioExt<T>
where
    T: Integer + Clone,
{
    pub fn from_integer(integer: T) -> RatioExt<T> {
        Self::Finite(Ratio::<T>::from_integer(integer))
    }

    pub fn finite(self) -> Option<Ratio<T>> {
        if let Self::Finite(value) = self {
            return Some(value);
        }
        None
    }

    pub const fn finite_as_ref(&self) -> Option<&Ratio<T>> {
        if let Self::Finite(value) = self {
            return Some(value);
        }
        None
    }

    pub const unsafe fn finite_as_ref_unchecked(&self) -> &Ratio<T> {
        if let Self::Finite(value) = self {
            return value;
        }
        hint::unreachable_unchecked()
    }
}

impl RatioExt<BigInt> {
    pub fn from_float<T: FloatCore>(f: T) -> Self {
        Ratio::<BigInt>::from_float(f).map_or_else(
            || {
                if f.is_nan() {
                    return RatioExt::Nan;
                }

                if f.is_sign_positive() {
                    RatioExt::Inf
                } else {
                    RatioExt::MinusInf
                }
            },
            RatioExt::Finite,
        )
    }
}

impl FromPrimitive for RatioExt<BigInt> {
    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::from_integer(n.into()))
    }

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Some(Self::from_integer(n.into()))
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::from_integer(n.into()))
    }

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(Self::from_integer(n.into()))
    }

    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        Some(Self::from_float(n))
    }

    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        Some(Self::from_float(n))
    }
}

impl<T> PartialEq for RatioExt<T>
where
    T: Clone + Integer,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RatioExt::Inf, RatioExt::Inf) | (RatioExt::MinusInf, RatioExt::MinusInf) => true,
            (RatioExt::Finite(lhs), RatioExt::Finite(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl<T> PartialOrd for RatioExt<T>
where
    T: Clone + Integer,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RatioExt::Nan, _)
            | (_, RatioExt::Nan)
            | (RatioExt::Inf, RatioExt::Inf)
            | (RatioExt::MinusInf, RatioExt::MinusInf) => None,
            (RatioExt::MinusInf, _) | (RatioExt::Finite(_), RatioExt::Inf) => Some(Ordering::Less),
            (RatioExt::Inf, _) | (RatioExt::Finite(_), RatioExt::MinusInf) => {
                Some(Ordering::Greater)
            }
            (RatioExt::Finite(lhs), RatioExt::Finite(rhs)) => lhs.partial_cmp(rhs),
        }
    }
}

impl<T> RatioExt<T>
where
    T: Integer + Clone,
{
    pub fn total_cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (RatioExt::Finite(lhs), RatioExt::Finite(rhs)) => lhs.cmp(rhs),
            (RatioExt::Inf, RatioExt::Inf)
            | (RatioExt::MinusInf, RatioExt::MinusInf)
            | (RatioExt::Nan, RatioExt::Nan) => Ordering::Equal,
            (RatioExt::Inf, _) | (_, RatioExt::Nan) => Ordering::Greater,
            (_, RatioExt::Inf) | (RatioExt::MinusInf, _) | (RatioExt::Nan, _) => Ordering::Less,
            (_, RatioExt::MinusInf) => Ordering::Greater,
        }
    }
}

impl<T, U> From<T> for RatioExt<U>
where
    Ratio<U>: From<T>,
    U: Clone + Integer,
{
    fn from(value: T) -> Self {
        Self::Finite(value.into())
    }
}

impl<T> Sum for RatioExt<T>
where
    T: Clone + Integer,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |sum, el| sum + el)
    }
}

impl<T> Product for RatioExt<T>
where
    T: Clone + Integer,
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |sum, el| sum * el)
    }
}

impl<T> Zero for RatioExt<T>
where
    T: Clone + Integer,
{
    #[inline]
    fn zero() -> Self {
        Self::Finite(Ratio::zero())
    }

    fn is_zero(&self) -> bool {
        if let Self::Finite(ratio) = self {
            return ratio.is_zero();
        }
        false
    }
}

impl<T> One for RatioExt<T>
where
    T: Clone + Integer,
{
    #[inline]
    fn one() -> Self {
        Self::Finite(Ratio::one())
    }

    fn is_one(&self) -> bool {
        if let Self::Finite(ratio) = self {
            return ratio.is_one();
        }
        false
    }
}

impl<T> Add for RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<T> Add for &RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return RatioExt::Nan;
        }
        match self {
            RatioExt::Inf => match rhs {
                RatioExt::Inf => RatioExt::Inf,
                RatioExt::Finite(_) => RatioExt::Inf,
                RatioExt::MinusInf => RatioExt::Nan,
                _ => unsafe { unreachable_unchecked() },
            },
            RatioExt::Finite(lhs) => match rhs {
                RatioExt::Inf => RatioExt::Inf,
                RatioExt::Finite(rhs) => RatioExt::Finite(lhs + rhs),
                RatioExt::MinusInf => RatioExt::MinusInf,
                _ => unsafe { unreachable_unchecked() },
            },
            RatioExt::MinusInf => match rhs {
                RatioExt::Inf => RatioExt::Nan,
                RatioExt::MinusInf => RatioExt::MinusInf,
                RatioExt::Finite(_) => RatioExt::MinusInf,
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T> AddAssign for RatioExt<T>
where
    T: Clone + Integer,
{
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs
    }
}

impl<T> AddAssign<&Self> for RatioExt<T>
where
    T: Clone + Integer,
{
    fn add_assign(&mut self, rhs: &Self) {
        *self = 'b: {
            if self.is_nan() || rhs.is_nan() {
                break 'b Self::Nan;
            }
            match self {
                Self::Inf => match rhs {
                    Self::Inf => Self::Inf,
                    Self::Finite(_) => Self::Inf,
                    Self::MinusInf => Self::Nan,
                    _ => unsafe { unreachable_unchecked() },
                },
                Self::Finite(lhs) => match rhs {
                    Self::Inf => Self::Inf,
                    Self::Finite(rhs) => Self::Finite(mem::take(lhs) + rhs),
                    Self::MinusInf => Self::MinusInf,
                    _ => unsafe { unreachable_unchecked() },
                },
                Self::MinusInf => match rhs {
                    Self::Inf => Self::Nan,
                    Self::MinusInf => Self::MinusInf,
                    Self::Finite(_) => Self::MinusInf,
                    _ => unsafe { unreachable_unchecked() },
                },
                _ => unsafe { unreachable_unchecked() },
            }
        }
    }
}

impl<T> Sub for RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl<T> Sub for &RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return RatioExt::Nan;
        }
        match self {
            RatioExt::Inf => match rhs {
                RatioExt::Inf => RatioExt::Nan,
                RatioExt::Finite(_) => RatioExt::Inf,
                RatioExt::MinusInf => RatioExt::Inf,
                _ => unsafe { unreachable_unchecked() },
            },
            RatioExt::Finite(lhs) => match rhs {
                RatioExt::Inf => RatioExt::MinusInf,
                RatioExt::Finite(rhs) => RatioExt::Finite(lhs - rhs),
                RatioExt::MinusInf => RatioExt::Inf,
                _ => unsafe { unreachable_unchecked() },
            },
            RatioExt::MinusInf => match rhs {
                RatioExt::Inf => RatioExt::MinusInf,
                RatioExt::MinusInf => RatioExt::Nan,
                RatioExt::Finite(_) => RatioExt::MinusInf,
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T> SubAssign for RatioExt<T>
where
    T: Clone + Integer,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs
    }
}

impl<T> SubAssign<&Self> for RatioExt<T>
where
    T: Clone + Integer,
{
    fn sub_assign(&mut self, rhs: &Self) {
        *self = 'b: {
            if self.is_nan() || rhs.is_nan() {
                break 'b Self::Nan;
            }
            match self {
                Self::Inf => match rhs {
                    Self::Inf => Self::Nan,
                    Self::Finite(_) => Self::Inf,
                    Self::MinusInf => Self::Inf,
                    _ => unsafe { unreachable_unchecked() },
                },
                Self::Finite(lhs) => match rhs {
                    Self::Inf => Self::MinusInf,
                    Self::Finite(rhs) => Self::Finite(mem::take(lhs) - rhs),
                    Self::MinusInf => Self::Inf,
                    _ => unsafe { unreachable_unchecked() },
                },
                Self::MinusInf => match rhs {
                    Self::Inf => Self::MinusInf,
                    Self::MinusInf => Self::Nan,
                    Self::Finite(_) => Self::MinusInf,
                    _ => unsafe { unreachable_unchecked() },
                },
                _ => unsafe { unreachable_unchecked() },
            }
        }
    }
}

impl<T> Mul for RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<T> Mul for &RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return RatioExt::Nan;
        }
        match self {
            RatioExt::Inf => match rhs {
                RatioExt::Inf => RatioExt::Inf,
                RatioExt::Finite(rhs) => match rhs {
                    _ if *rhs < Ratio::zero() => RatioExt::MinusInf,
                    _ if rhs.is_zero() => RatioExt::zero(),
                    _ => RatioExt::Inf,
                },
                RatioExt::MinusInf => RatioExt::MinusInf,
                _ => unsafe { unreachable_unchecked() },
            },
            RatioExt::Finite(lhs) => match rhs {
                RatioExt::Inf => match lhs {
                    _ if *lhs < Ratio::zero() => RatioExt::MinusInf,
                    _ if lhs.is_zero() => RatioExt::zero(),
                    _ => RatioExt::Inf,
                },
                RatioExt::Finite(rhs) => RatioExt::Finite(lhs * rhs),
                RatioExt::MinusInf => RatioExt::MinusInf,
                _ => unsafe { unreachable_unchecked() },
            },
            RatioExt::MinusInf => match rhs {
                RatioExt::Inf => RatioExt::MinusInf,
                RatioExt::MinusInf => RatioExt::Inf,
                RatioExt::Finite(rhs) => match rhs {
                    _ if *rhs < Ratio::zero() => RatioExt::Inf,
                    _ if rhs.is_zero() => RatioExt::zero(),
                    _ => RatioExt::MinusInf,
                },
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T> MulAssign for RatioExt<T>
where
    T: Clone + Integer,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self += &rhs
    }
}

impl<T> MulAssign<&Self> for RatioExt<T>
where
    T: Clone + Integer,
{
    fn mul_assign(&mut self, rhs: &Self) {
        *self = 'b: {
            if self.is_nan() || rhs.is_nan() {
                break 'b Self::Nan;
            }
            match self {
                Self::Inf => match rhs {
                    Self::Inf => Self::Inf,
                    Self::Finite(rhs) => match rhs {
                        _ if *rhs < Ratio::zero() => Self::MinusInf,
                        _ if rhs.is_zero() => Self::zero(),
                        _ => Self::Inf,
                    },
                    Self::MinusInf => Self::MinusInf,
                    _ => unsafe { unreachable_unchecked() },
                },
                Self::Finite(lhs) => match rhs {
                    Self::Inf => match lhs {
                        _ if *lhs < Ratio::zero() => Self::MinusInf,
                        _ if lhs.is_zero() => Self::zero(),
                        _ => Self::Inf,
                    },
                    Self::Finite(rhs) => Self::Finite(mem::take(lhs) * rhs),
                    Self::MinusInf => Self::MinusInf,
                    _ => unsafe { unreachable_unchecked() },
                },
                Self::MinusInf => match rhs {
                    Self::Inf => Self::MinusInf,
                    Self::MinusInf => Self::Inf,
                    Self::Finite(rhs) => match rhs {
                        _ if *rhs < Ratio::zero() => Self::Inf,
                        _ if rhs.is_zero() => Self::zero(),
                        _ => Self::MinusInf,
                    },
                    _ => unsafe { unreachable_unchecked() },
                },
                _ => unsafe { unreachable_unchecked() },
            }
        }
    }
}

impl<T> Div for RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl<T> Div for &RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return RatioExt::Nan;
        }
        match self {
            RatioExt::Inf => match rhs {
                RatioExt::Finite(rhs) => match rhs {
                    _ if *rhs < Ratio::zero() => RatioExt::MinusInf,
                    _ => RatioExt::Inf,
                },
                RatioExt::Inf | RatioExt::MinusInf => RatioExt::Nan,
                _ => unsafe { unreachable_unchecked() },
            },
            RatioExt::Finite(lhs) => match rhs {
                RatioExt::Finite(rhs) => match rhs {
                    _ if rhs.is_zero() => match lhs {
                        _ if lhs.is_zero() => RatioExt::Nan,
                        _ if *lhs < Ratio::zero() => RatioExt::MinusInf,
                        _ => RatioExt::Inf,
                    },
                    _ => RatioExt::Finite(lhs / rhs),
                },
                RatioExt::Inf | RatioExt::MinusInf => RatioExt::zero(),
                _ => unsafe { unreachable_unchecked() },
            },
            RatioExt::MinusInf => match rhs {
                RatioExt::Finite(rhs) => match rhs {
                    _ if *rhs < Ratio::zero() => RatioExt::Inf,
                    _ => RatioExt::MinusInf,
                },
                RatioExt::Inf | RatioExt::MinusInf => RatioExt::Nan,
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T> DivAssign for RatioExt<T>
where
    T: Clone + Integer,
{
    fn div_assign(&mut self, rhs: Self) {
        *self *= &rhs;
    }
}

impl<T> DivAssign<&Self> for RatioExt<T>
where
    T: Integer + Clone,
{
    fn div_assign(&mut self, rhs: &Self) {
        *self = 'b: {
            if self.is_nan() || rhs.is_nan() {
                break 'b Self::Nan;
            }
            match self {
                Self::Inf => match rhs {
                    Self::Finite(rhs) => match rhs {
                        _ if *rhs < Ratio::zero() => Self::MinusInf,
                        _ => Self::Inf,
                    },
                    Self::Inf | Self::MinusInf => Self::Nan,
                    _ => unsafe { unreachable_unchecked() },
                },
                Self::Finite(lhs) => match rhs {
                    Self::Finite(rhs) => match rhs {
                        _ if rhs.is_zero() => match lhs {
                            _ if lhs.is_zero() => Self::Nan,
                            _ if *lhs < Ratio::zero() => Self::MinusInf,
                            _ => Self::Inf,
                        },
                        _ => Self::Finite(mem::take(lhs) / rhs),
                    },
                    Self::Inf | Self::MinusInf => Self::zero(),
                    _ => unsafe { unreachable_unchecked() },
                },
                Self::MinusInf => match rhs {
                    Self::Finite(rhs) => match rhs {
                        _ if *rhs < Ratio::zero() => Self::Inf,
                        _ => Self::MinusInf,
                    },
                    Self::Inf | Self::MinusInf => Self::Nan,
                    _ => unsafe { unreachable_unchecked() },
                },
                _ => unsafe { unreachable_unchecked() },
            }
        }
    }
}

impl<T> Neg for RatioExt<T>
where
    T: Clone + Integer + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Inf => Self::MinusInf,
            Self::Finite(ratio) => Self::Finite(-ratio),
            Self::MinusInf => Self::Inf,
            Self::Nan => Self::Nan,
        }
    }
}

impl<T> fmt::Display for RatioExt<T>
where
    T: fmt::Display + Integer + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Finite(ratio) => write!(f, "{ratio}"),
            Self::Inf => write!(f, "Inf"),
            Self::MinusInf => write!(f, "-Inf"),
            Self::Nan => write!(f, "NaN"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn total_cmp() {
        #[derive(Debug)]
        struct GoodBoy {
            _name: String,
            weight: RatioExt<i32>,
        }

        let mut bois = vec![
            GoodBoy {
                _name: "Pucci".to_owned(),
                weight: (1, 10).into(),
            },
            GoodBoy {
                _name: "Woofer".to_owned(),
                weight: (99, 1).into(),
            },
            GoodBoy {
                _name: "Yapper".to_owned(),
                weight: (10, 1).into(),
            },
            GoodBoy {
                _name: "Chonk".to_owned(),
                weight: RatioExt::Inf,
            },
            GoodBoy {
                _name: "Abs. Unit".to_owned(),
                weight: RatioExt::Nan,
            },
            GoodBoy {
                _name: "Floaty".to_owned(),
                weight: (-5, 1).into(),
            },
        ];

        bois.sort_by(|a, b| a.weight.total_cmp(&b.weight));
        println!("{bois:?}");
        assert_str_eq!(
            format!(
                "{:?}",
                bois.into_iter().map(|b| b.weight).collect::<Vec<_>>()
            ),
            format!(
                "{:?}",
                vec![
                    RatioExt::Nan,
                    (-5, 1).into(),
                    (1, 10).into(),
                    (10, 1).into(),
                    (99, 1).into(),
                    RatioExt::Inf
                ]
            )
        );
    }
}
