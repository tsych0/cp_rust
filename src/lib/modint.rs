use crate::internal_math;
use std::{
    cell::RefCell,
    convert::{Infallible, TryInto as _},
    fmt,
    hash::{Hash, Hasher},
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
    sync::atomic::{self, AtomicU32, AtomicU64},
    thread::LocalKey,
};

pub type ModInt1000000007 = StaticModInt<Mod1000000007>;
pub type ModInt998244353 = StaticModInt<Mod998244353>;
pub type ModInt = DynamicModInt<DefaultId>;

/// Represents $\mathbb{Z}/m\mathbb{Z}$ where $m$ is a constant value.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct StaticModInt<M> {
    val: u32,
    phantom: PhantomData<fn() -> M>,
}

impl<M: Modulus> StaticModInt<M> {
    /// Returns the modulus, which is [`<M as Modulus>::VALUE`].
    #[inline(always)]
    pub fn modulus() -> u32 {
        M::VALUE
    }

    /// Creates a new `StaticModInt`.
    #[inline]
    pub fn new<T: RemEuclidU32>(val: T) -> Self {
        Self::raw(val.rem_euclid_u32(M::VALUE))
    }

    /// Constructs a `StaticModInt` from a `val < Self::modulus()` without checking it.
    #[inline]
    pub fn raw(val: u32) -> Self {
        Self {
            val,
            phantom: PhantomData,
        }
    }

    /// Retruns the representative.
    #[inline]
    pub fn val(self) -> u32 {
        self.val
    }

    /// Returns `self` to the power of `n`.
    #[inline]
    pub fn pow(self, n: u64) -> Self {
        <Self as ModIntBase>::pow(self, n)
    }

    /// Retruns the multiplicative inverse of `self`.
    #[inline]
    pub fn inv(self) -> Self {
        if M::HINT_VALUE_IS_PRIME {
            if self.val() == 0 {
                panic!("attempt to divide by zero");
            }
            debug_assert!(
                internal_math::is_prime(M::VALUE.try_into().unwrap()),
                "{} is not a prime number",
                M::VALUE,
            );
            self.pow((M::VALUE - 2).into())
        } else {
            Self::inv_for_non_prime_modulus(self)
        }
    }
}

/// These methods are implemented for the struct.
/// You don't need to `use` `ModIntBase` to call methods of `StaticModInt`.
impl<M: Modulus> ModIntBase for StaticModInt<M> {
    #[inline(always)]
    fn modulus() -> u32 {
        Self::modulus()
    }

    #[inline]
    fn raw(val: u32) -> Self {
        Self::raw(val)
    }

    #[inline]
    fn val(self) -> u32 {
        self.val()
    }

    #[inline]
    fn inv(self) -> Self {
        self.inv()
    }
}

/// Represents a modulus.
pub trait Modulus: 'static + Copy + Eq {
    const VALUE: u32;
    const HINT_VALUE_IS_PRIME: bool;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>>;
}

/// Represents $1000000007$.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Mod1000000007 {}

impl Modulus for Mod1000000007 {
    const VALUE: u32 = 1_000_000_007;
    const HINT_VALUE_IS_PRIME: bool = true;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod1000000007>>> = RefCell::default();
        }
        &BUTTERFLY_CACHE
    }
}

/// Represents $998244353$.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Mod998244353 {}

impl Modulus for Mod998244353 {
    const VALUE: u32 = 998_244_353;
    const HINT_VALUE_IS_PRIME: bool = true;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod998244353>>> = RefCell::default();
        }
        &BUTTERFLY_CACHE
    }
}

/// Cache for butterfly operations.
pub struct ButterflyCache<M> {
    pub(crate) sum_e: Vec<StaticModInt<M>>,
    pub(crate) sum_ie: Vec<StaticModInt<M>>,
}

/// Represents $\mathbb{Z}/m\mathbb{Z}$ where $m$ is a dynamic value.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DynamicModInt<I> {
    val: u32,
    phantom: PhantomData<fn() -> I>,
}

impl<I: Id> DynamicModInt<I> {
    /// Returns the modulus.
    #[inline]
    pub fn modulus() -> u32 {
        I::companion_barrett().umod()
    }

    /// Sets a modulus.
    #[inline]
    pub fn set_modulus(modulus: u32) {
        if modulus == 0 {
            panic!("the modulus must not be 0");
        }
        I::companion_barrett().update(modulus);
    }

    /// Creates a new `DynamicModInt`.
    #[inline]
    pub fn new<T: RemEuclidU32>(val: T) -> Self {
        <Self as ModIntBase>::new(val)
    }

    /// Constructs a `DynamicModInt` from a `val < Self::modulus()` without checking it.
    #[inline]
    pub fn raw(val: u32) -> Self {
        Self {
            val,
            phantom: PhantomData,
        }
    }

    /// Retruns the representative.
    #[inline]
    pub fn val(self) -> u32 {
        self.val
    }

    /// Returns `self` to the power of `n`.
    #[inline]
    pub fn pow(self, n: u64) -> Self {
        <Self as ModIntBase>::pow(self, n)
    }

    /// Retruns the multiplicative inverse of `self`.
    #[inline]
    pub fn inv(self) -> Self {
        Self::inv_for_non_prime_modulus(self)
    }
}

/// These methods are implemented for the struct.
/// You don't need to `use` `ModIntBase` to call methods of `DynamicModInt`.
impl<I: Id> ModIntBase for DynamicModInt<I> {
    #[inline]
    fn modulus() -> u32 {
        Self::modulus()
    }

    #[inline]
    fn raw(val: u32) -> Self {
        Self::raw(val)
    }

    #[inline]
    fn val(self) -> u32 {
        self.val()
    }

    #[inline]
    fn inv(self) -> Self {
        self.inv()
    }
}

pub trait Id: 'static + Copy + Eq {
    fn companion_barrett() -> &'static Barrett;
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DefaultId {}

impl Id for DefaultId {
    fn companion_barrett() -> &'static Barrett {
        static BARRETT: Barrett = Barrett::default();
        &BARRETT
    }
}

/// Pair of $m$ and $\lceil 2^{64}/m \rceil$.
pub struct Barrett {
    m: AtomicU32,
    im: AtomicU64,
}

impl Barrett {
    /// Creates a new `Barrett`.
    #[inline]
    pub const fn new(m: u32) -> Self {
        Self {
            m: AtomicU32::new(m),
            im: AtomicU64::new((-1i64 as u64 / m as u64).wrapping_add(1)),
        }
    }

    #[inline]
    const fn default() -> Self {
        Self::new(998_244_353)
    }

    #[inline]
    fn update(&self, m: u32) {
        let im = (-1i64 as u64 / m as u64).wrapping_add(1);
        self.m.store(m, atomic::Ordering::SeqCst);
        self.im.store(im, atomic::Ordering::SeqCst);
    }

    #[inline]
    fn umod(&self) -> u32 {
        self.m.load(atomic::Ordering::SeqCst)
    }

    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        let m = self.m.load(atomic::Ordering::SeqCst);
        let im = self.im.load(atomic::Ordering::SeqCst);
        internal_math::mul_mod(a, b, m, im)
    }
}

impl Default for Barrett {
    #[inline]
    fn default() -> Self {
        Self::default()
    }
}

/// A trait for [`StaticModInt`] and [`DynamicModInt`].
pub trait ModIntBase:
    Default
    + FromStr
    + From<i8>
    + From<i16>
    + From<i32>
    + From<i64>
    + From<i128>
    + From<isize>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<u128>
    + From<usize>
    + Copy
    + Eq
    + Hash
    + fmt::Display
    + fmt::Debug
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    /// Returns the modulus.
    fn modulus() -> u32;

    /// Constructs a `Self` from a `val < Self::modulus()` without checking it.
    fn raw(val: u32) -> Self;

    /// Retruns the representative.
    fn val(self) -> u32;

    /// Retruns the multiplicative inverse of `self`.
    fn inv(self) -> Self;

    /// Creates a new `Self`.
    #[inline]
    fn new<T: RemEuclidU32>(val: T) -> Self {
        Self::raw(val.rem_euclid_u32(Self::modulus()))
    }

    /// Returns `self` to the power of `n`.
    #[inline]
    fn pow(self, mut n: u64) -> Self {
        let mut x = self;
        let mut r = Self::raw(u32::from(Self::modulus() > 1));
        while n > 0 {
            if n & 1 == 1 {
                r *= x;
            }
            x *= x;
            n >>= 1;
        }
        r
    }
}

/// A trait for `{StaticModInt, DynamicModInt, ModIntBase}::new`.
pub trait RemEuclidU32 {
    /// Calculates `self` $\bmod$ `modulus` losslessly.
    fn rem_euclid_u32(self, modulus: u32) -> u32;
}

macro_rules! impl_rem_euclid_u32_for_small_signed {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (self as i64).rem_euclid(i64::from(modulus)) as _
                }
            }
        )*
    }
}

impl_rem_euclid_u32_for_small_signed!(i8, i16, i32, i64, isize);

impl RemEuclidU32 for i128 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        self.rem_euclid(i128::from(modulus)) as _
    }
}

macro_rules! impl_rem_euclid_u32_for_small_unsigned {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    self as u32 % modulus
                }
            }
        )*
    }
}

macro_rules! impl_rem_euclid_u32_for_large_unsigned {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (self % (modulus as $ty)) as _
                }
            }
        )*
    }
}

impl_rem_euclid_u32_for_small_unsigned!(u8, u16, u32);
impl_rem_euclid_u32_for_large_unsigned!(u64, u128);

#[cfg(target_pointer_width = "32")]
impl_rem_euclid_u32_for_small_unsigned!(usize);

#[cfg(target_pointer_width = "64")]
impl_rem_euclid_u32_for_large_unsigned!(usize);

trait InternalImplementations: ModIntBase {
    #[inline]
    fn inv_for_non_prime_modulus(this: Self) -> Self {
        let (gcd, x) = internal_math::inv_gcd(this.val().into(), Self::modulus().into());
        if gcd != 1 {
            panic!("the multiplicative inverse does not exist");
        }
        Self::new(x)
    }

    #[inline]
    fn default_impl() -> Self {
        Self::raw(0)
    }

    #[inline]
    fn from_str_impl(s: &str) -> Result<Self, Infallible> {
        Ok(s.parse::<i64>()
            .map(Self::new)
            .unwrap_or_else(|_| todo!("parsing as an arbitrary precision integer?")))
    }

    #[inline]
    fn hash_impl(this: &Self, state: &mut impl Hasher) {
        this.val().hash(state)
    }

    #[inline]
    fn display_impl(this: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&this.val(), f)
    }

    #[inline]
    fn debug_impl(this: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&this.val(), f)
    }

    #[inline]
    fn neg_impl(this: Self) -> Self {
        Self::sub_impl(Self::raw(0), this)
    }

    #[inline]
    fn add_impl(lhs: Self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        let mut val = lhs.val() + rhs.val();
        if val >= modulus {
            val -= modulus;
        }
        Self::raw(val)
    }

    #[inline]
    fn sub_impl(lhs: Self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        let mut val = lhs.val().wrapping_sub(rhs.val());
        if val >= modulus {
            val = val.wrapping_add(modulus)
        }
        Self::raw(val)
    }

    fn mul_impl(lhs: Self, rhs: Self) -> Self;

    #[inline]
    fn div_impl(lhs: Self, rhs: Self) -> Self {
        Self::mul_impl(lhs, rhs.inv())
    }
}

impl<M: Modulus> InternalImplementations for StaticModInt<M> {
    #[inline]
    fn mul_impl(lhs: Self, rhs: Self) -> Self {
        Self::raw((u64::from(lhs.val()) * u64::from(rhs.val()) % u64::from(M::VALUE)) as u32)
    }
}

impl<I: Id> InternalImplementations for DynamicModInt<I> {
    #[inline]
    fn mul_impl(lhs: Self, rhs: Self) -> Self {
        Self::raw(I::companion_barrett().mul(lhs.val, rhs.val))
    }
}

macro_rules! impl_basic_traits {
    () => {};
    (impl <$generic_param:ident : $generic_param_bound:tt> _ for $self:ty; $($rest:tt)*) => {
        impl <$generic_param: $generic_param_bound> Default for $self {
            #[inline]
            fn default() -> Self {
                Self::default_impl()
            }
        }

        impl <$generic_param: $generic_param_bound> FromStr for $self {
            type Err = Infallible;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Infallible> {
                Self::from_str_impl(s)
            }
        }

        impl<$generic_param: $generic_param_bound, V: RemEuclidU32> From<V> for $self {
            #[inline]
            fn from(from: V) -> Self {
                Self::new(from)
            }
        }

        impl<$generic_param: $generic_param_bound> Hash for $self {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                Self::hash_impl(self, state)
            }
        }

        impl<$generic_param: $generic_param_bound> fmt::Display for $self {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Self::display_impl(self, f)
            }
        }

        impl<$generic_param: $generic_param_bound> fmt::Debug for $self {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Self::debug_impl(self, f)
            }
        }

        impl<$generic_param: $generic_param_bound> Neg for $self {
            type Output = $self;

            #[inline]
            fn neg(self) -> $self {
                Self::neg_impl(self)
            }
        }

        impl<$generic_param: $generic_param_bound> Neg for &'_ $self {
            type Output = $self;

            #[inline]
            fn neg(self) -> $self {
                <$self>::neg_impl(*self)
            }
        }

        impl_basic_traits!($($rest)*);
    };
}

impl_basic_traits! {
    impl <M: Modulus> _ for StaticModInt<M> ;
    impl <I: Id     > _ for DynamicModInt<I>;
}

macro_rules! impl_bin_ops {
    () => {};
    (for<$($generic_param:ident : $generic_param_bound:tt),*> <$lhs_ty:ty> ~ <$rhs_ty:ty> -> $output:ty { { $lhs_body:expr } ~ { $rhs_body:expr } } $($rest:tt)*) => {
        impl <$($generic_param: $generic_param_bound),*> Add<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs_ty) -> $output {
                <$output>::add_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$($generic_param: $generic_param_bound),*> Sub<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs_ty) -> $output {
                <$output>::sub_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$($generic_param: $generic_param_bound),*> Mul<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs_ty) -> $output {
                <$output>::mul_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$($generic_param: $generic_param_bound),*> Div<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn div(self, rhs: $rhs_ty) -> $output {
                <$output>::div_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl_bin_ops!($($rest)*);
    };
}

macro_rules! impl_assign_ops {
    () => {};
    (for<$($generic_param:ident : $generic_param_bound:tt),*> <$lhs_ty:ty> ~= <$rhs_ty:ty> { _ ~= { $rhs_body:expr } } $($rest:tt)*) => {
        impl <$($generic_param: $generic_param_bound),*> AddAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs_ty) {
                *self = *self + apply($rhs_body, rhs);
            }
        }

        impl <$($generic_param: $generic_param_bound),*> SubAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs_ty) {
                *self = *self - apply($rhs_body, rhs);
            }
        }

        impl <$($generic_param: $generic_param_bound),*> MulAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn mul_assign(&mut self, rhs: $rhs_ty) {
                *self = *self * apply($rhs_body, rhs);
            }
        }

        impl <$($generic_param: $generic_param_bound),*> DivAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn div_assign(&mut self, rhs: $rhs_ty) {
                *self = *self / apply($rhs_body, rhs);
            }
        }

        impl_assign_ops!($($rest)*);
    };
}

#[inline]
fn apply<F: FnOnce(X) -> O, X, O>(f: F, x: X) -> O {
    f(x)
}

impl_bin_ops! {
    for<M: Modulus> <StaticModInt<M>     > ~ <StaticModInt<M>     > -> StaticModInt<M>  { { |x| x  } ~ { |x| x  } }
    for<M: Modulus> <StaticModInt<M>     > ~ <&'_ StaticModInt<M> > -> StaticModInt<M>  { { |x| x  } ~ { |&x| x } }
    for<M: Modulus> <&'_ StaticModInt<M> > ~ <StaticModInt<M>     > -> StaticModInt<M>  { { |&x| x } ~ { |x| x  } }
    for<M: Modulus> <&'_ StaticModInt<M> > ~ <&'_ StaticModInt<M> > -> StaticModInt<M>  { { |&x| x } ~ { |&x| x } }
    for<I: Id     > <DynamicModInt<I>    > ~ <DynamicModInt<I>    > -> DynamicModInt<I> { { |x| x  } ~ { |x| x  } }
    for<I: Id     > <DynamicModInt<I>    > ~ <&'_ DynamicModInt<I>> -> DynamicModInt<I> { { |x| x  } ~ { |&x| x } }
    for<I: Id     > <&'_ DynamicModInt<I>> ~ <DynamicModInt<I>    > -> DynamicModInt<I> { { |&x| x } ~ { |x| x  } }
    for<I: Id     > <&'_ DynamicModInt<I>> ~ <&'_ DynamicModInt<I>> -> DynamicModInt<I> { { |&x| x } ~ { |&x| x } }

    for<M: Modulus, T: RemEuclidU32> <StaticModInt<M>     > ~ <T> -> StaticModInt<M>  { { |x| x  } ~ { StaticModInt::<M>::new } }
    for<I: Id     , T: RemEuclidU32> <DynamicModInt<I>    > ~ <T> -> DynamicModInt<I> { { |x| x  } ~ { DynamicModInt::<I>::new } }
}

impl_assign_ops! {
    for<M: Modulus> <StaticModInt<M> > ~= <StaticModInt<M>     > { _ ~= { |x| x  } }
    for<M: Modulus> <StaticModInt<M> > ~= <&'_ StaticModInt<M> > { _ ~= { |&x| x } }
    for<I: Id     > <DynamicModInt<I>> ~= <DynamicModInt<I>    > { _ ~= { |x| x  } }
    for<I: Id     > <DynamicModInt<I>> ~= <&'_ DynamicModInt<I>> { _ ~= { |&x| x } }

    for<M: Modulus, T: RemEuclidU32> <StaticModInt<M> > ~= <T> { _ ~= { StaticModInt::<M>::new } }
    for<I: Id,      T: RemEuclidU32> <DynamicModInt<I>> ~= <T> { _ ~= { DynamicModInt::<I>::new } }
}

macro_rules! impl_folding {
    () => {};
    (impl<$generic_param:ident : $generic_param_bound:tt> $trait:ident<_> for $self:ty { fn $method:ident(_) -> _ { _($unit:expr, $op:expr) } } $($rest:tt)*) => {
        impl<$generic_param: $generic_param_bound> $trait<Self> for $self {
            #[inline]
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = Self>, {
                iter.fold($unit, $op)
            }
        }

        impl<'a, $generic_param: $generic_param_bound> $trait<&'a Self> for $self {
            #[inline]
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = &'a Self>, {
                iter.fold($unit, $op)
            }
        }

        impl_folding!($($rest)*);
    };
}

impl_folding! {
    impl<M: Modulus> Sum<_>     for StaticModInt<M>  { fn sum(_)     -> _ { _(Self::raw(0), Add::add) } }
    impl<M: Modulus> Product<_> for StaticModInt<M>  { fn product(_) -> _ { _(Self::raw(u32::from(Self::modulus() > 1)), Mul::mul) } }
    impl<I: Id     > Sum<_>     for DynamicModInt<I> { fn sum(_)     -> _ { _(Self::raw(0), Add::add) } }
    impl<I: Id     > Product<_> for DynamicModInt<I> { fn product(_) -> _ { _(Self::raw(u32::from(Self::modulus() > 1)), Mul::mul) } }
}
