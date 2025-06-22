use std::{
    fmt,
    iter::{Product, Sum},
    ops::*,
};

pub trait Integral:
    'static
    + Send
    + Sync
    + Copy
    + Ord
    + Not<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
    + Product
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Shl<Output = Self>
    + Shr<Output = Self>
    + ShlAssign
    + ShrAssign
    + fmt::Display
    + fmt::Debug
    + fmt::Binary
    + fmt::Octal
    + Zero
    + One
    + BoundedBelow
    + BoundedAbove
{
}

/// Class that has additive identity element
pub trait Zero: Sized + Add<Self, Output = Self> {
    /// The additive identity element
    fn zero() -> Self;
}

/// Class that has multiplicative identity element
pub trait One {
    /// The multiplicative identity element
    fn one() -> Self;
}

pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::MIN
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::MAX
                }
            }

            impl Integral for $ty {}
        )*
    };
}

impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

pub trait Bounded {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

macro_rules! impl_bounded {
    ($($t:ty),*) => {
        $(
            impl Bounded for $t {
                fn min_value() -> Self { <$t>::MIN }
                fn max_value() -> Self { <$t>::MAX }
            }
        )*
    };
}

pub trait Signed: Sized + std::ops::Sub<Output = Self> + std::ops::Neg<Output = Self> {
    fn abs(&self) -> Self;
    fn abs_sub(&self, other: &Self) -> Self;
    fn signum(&self) -> Self;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}

macro_rules! impl_signed {
    ($($t:ty),*) => {
        $(
            impl Signed for $t {
                fn abs(&self) -> Self { <$t>::abs(*self) }
                fn abs_sub(&self, other: &Self) -> Self {
                    if *self <= *other { 0 as $t } else { *self - *other }
                }
                fn signum(&self) -> Self { <$t>::signum(*self) }
                fn is_positive(&self) -> bool { *self > 0 as $t }
                fn is_negative(&self) -> bool { *self < 0 as $t }
            }
        )*
    };
}

// Apply to all types
impl_bounded!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_signed!(i8, i16, i32, i64, i128, isize, f32, f64);
