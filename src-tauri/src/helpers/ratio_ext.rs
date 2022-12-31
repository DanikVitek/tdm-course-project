use std::{
    cmp::Ordering,
    fmt,
    hint::unreachable_unchecked,
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Sub, Neg},
};

use derive_more::IsVariant;
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::{One, Zero};

#[derive(Debug, Clone, IsVariant)]
pub enum RatioExt<T> {
    Inf,
    Finite(Ratio<T>),
    MinusInf,
    Nan,
}

impl<T: Clone + Integer> Default for RatioExt<T> {
    fn default() -> Self {
        Self::Finite(Ratio::<T>::default())
    }
}

impl<T: Clone + Integer> PartialEq for RatioExt<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RatioExt::Inf, RatioExt::Inf) | (RatioExt::MinusInf, RatioExt::MinusInf) => true,
            (RatioExt::Finite(lhs), RatioExt::Finite(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl<T: Clone + Integer> PartialOrd for RatioExt<T> {
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

impl<T, U> From<T> for RatioExt<U>
where
    Ratio<U>: From<T>,
{
    fn from(value: T) -> Self {
        Self::Finite(value.into())
    }
}

impl<T: Clone + Integer> Sum for RatioExt<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |sum, el| sum + el)
    }
}

impl<T: Clone + Integer> Product for RatioExt<T> {
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
        if self.is_nan() || rhs.is_nan() {
            return Self::Nan;
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
                Self::Finite(rhs) => Self::Finite(lhs + rhs),
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

impl<T> Sub for RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return Self::Nan;
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
                Self::Finite(rhs) => Self::Finite(lhs - rhs),
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

impl<T> Mul for RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return Self::Nan;
        }
        match self {
            Self::Inf => match rhs {
                Self::Inf => Self::Inf,
                Self::Finite(rhs) => match rhs {
                    _ if rhs < Ratio::zero() => Self::MinusInf,
                    _ if rhs.is_zero() => Self::zero(),
                    _ => Self::Inf,
                },
                Self::MinusInf => Self::MinusInf,
                _ => unsafe { unreachable_unchecked() },
            },
            Self::Finite(lhs) => match rhs {
                Self::Inf => match lhs {
                    _ if lhs < Ratio::zero() => Self::MinusInf,
                    _ if lhs.is_zero() => Self::zero(),
                    _ => Self::Inf,
                },
                Self::Finite(rhs) => Self::Finite(lhs * rhs),
                Self::MinusInf => Self::MinusInf,
                _ => unsafe { unreachable_unchecked() },
            },
            Self::MinusInf => match rhs {
                Self::Inf => Self::MinusInf,
                Self::MinusInf => Self::Inf,
                Self::Finite(rhs) => match rhs {
                    _ if rhs < Ratio::zero() => Self::Inf,
                    _ if rhs.is_zero() => Self::zero(),
                    _ => Self::MinusInf,
                },
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T> Div for RatioExt<T>
where
    T: Clone + Integer,
{
    type Output = RatioExt<T>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return Self::Nan;
        }
        match self {
            Self::Inf => match rhs {
                Self::Finite(rhs) => match rhs {
                    _ if rhs < Ratio::zero() => Self::MinusInf,
                    _ => Self::Inf,
                },
                Self::Inf | Self::MinusInf => Self::Nan,
                _ => unsafe { unreachable_unchecked() },
            },
            Self::Finite(lhs) => match rhs {
                Self::Finite(rhs) => match rhs {
                    _ if rhs.is_zero() => match lhs {
                        _ if lhs.is_zero() => Self::Nan,
                        _ if lhs < Ratio::zero() => Self::MinusInf,
                        _ => Self::Inf,
                    },
                    _ => Self::Finite(lhs / rhs),
                },
                Self::Inf | Self::MinusInf => Self::zero(),
                _ => unsafe { unreachable_unchecked() },
            },
            Self::MinusInf => match rhs {
                Self::Finite(rhs) => match rhs {
                    _ if rhs < Ratio::zero() => Self::Inf,
                    _ => Self::MinusInf,
                },
                Self::Inf | Self::MinusInf => Self::Nan,
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T: Clone + Integer + Neg<Output = T>> Neg for RatioExt<T> {
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

impl<T: fmt::Display + Integer + Clone> fmt::Display for RatioExt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Finite(ratio) => write!(f, "{ratio}"),
            Self::Inf => write!(f, "Inf"),
            Self::MinusInf => write!(f, "-Inf"),
            Self::Nan => write!(f, "NaN"),
        }
    }
}