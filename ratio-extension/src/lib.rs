mod ratio_ext;

use num_bigint::BigInt;
pub use ratio_ext::RatioExt;

pub type BigRationalExt = RatioExt<BigInt>;
pub type Rational32Ext = RatioExt<i32>;
pub type Rational64Ext = RatioExt<i64>;
