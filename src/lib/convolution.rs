//! Functions that calculate $(+, \times)$ convolution.
//!
//! Given two non-empty sequences $a_0, a_1, \ldots, a_{N - 1}$ and $b_0, b_1, \ldots, b_{M - 1}$, they calculate the sequence $c$ of length $N + M - 1$ defined by
//!
//! \\[
//!   c_i = \sum_ {j = 0}^i a_j b_{i - j}
//! \\]
macro_rules! modulus {
    ($($name:ident),*) => {
        $(
            #[derive(Copy, Clone, Eq, PartialEq)]
            enum $name {}

            impl Modulus for $name {
                const VALUE: u32 = $name as _;
                const HINT_VALUE_IS_PRIME: bool = true;

                fn butterfly_cache() -> &'static ::std::thread::LocalKey<::std::cell::RefCell<::std::option::Option<$crate::modint::ButterflyCache<Self>>>> {
                    thread_local! {
                        static BUTTERFLY_CACHE: ::std::cell::RefCell<::std::option::Option<$crate::modint::ButterflyCache<$name>>> = ::std::default::Default::default();
                    }
                    &BUTTERFLY_CACHE
                }
            }
        )*
    };
}

use crate::{
    internal_bit, internal_math,
    modint::{ButterflyCache, Modulus, RemEuclidU32, StaticModInt},
};
use std::{
    cmp,
    convert::{TryFrom, TryInto as _},
    fmt,
};

/// Calculates the $(+, \times)$ convolution is $\mathbb{Z}/p\mathbb{Z}$.
pub fn convolution<M>(a: &[StaticModInt<M>], b: &[StaticModInt<M>]) -> Vec<StaticModInt<M>>
where
    M: Modulus,
{
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let (n, m) = (a.len(), b.len());

    if cmp::min(n, m) <= 60 {
        let (n, m, a, b) = if n < m { (m, n, b, a) } else { (n, m, a, b) };
        let mut ans = vec![StaticModInt::new(0); n + m - 1];
        for i in 0..n {
            for j in 0..m {
                ans[i + j] += a[i] * b[j];
            }
        }
        return ans;
    }

    let (mut a, mut b) = (a.to_owned(), b.to_owned());
    let z = 1 << internal_bit::ceil_pow2((n + m - 1) as _);
    a.resize(z, StaticModInt::raw(0));
    butterfly(&mut a);
    b.resize(z, StaticModInt::raw(0));
    butterfly(&mut b);
    for (a, b) in a.iter_mut().zip(&b) {
        *a *= b;
    }
    butterfly_inv(&mut a);
    a.resize(n + m - 1, StaticModInt::raw(0));
    let iz = StaticModInt::new(z).inv();
    for a in &mut a {
        *a *= iz;
    }
    a
}

/// Calculates the $(+, \times)$ convolution is $\mathbb{Z}/p\mathbb{Z}$.
pub fn convolution_raw<T, M>(a: &[T], b: &[T]) -> Vec<T>
where
    T: RemEuclidU32 + TryFrom<u32> + Clone,
    T::Error: fmt::Debug,
    M: Modulus,
{
    let a = a.iter().cloned().map(Into::into).collect::<Vec<_>>();
    let b = b.iter().cloned().map(Into::into).collect::<Vec<_>>();
    convolution::<M>(&a, &b)
        .into_iter()
        .map(|z| {
            z.val()
                .try_into()
                .expect("the numeric type is smaller than the modulus")
        })
        .collect()
}

/// Calculates the $(+, \times)$ convolution is `i64`.
pub fn convolution_i64(a: &[i64], b: &[i64]) -> Vec<i64> {
    const M1: u64 = 754_974_721; // 2^24
    const M2: u64 = 167_772_161; // 2^25
    const M3: u64 = 469_762_049; // 2^26
    const M2M3: u64 = M2 * M3;
    const M1M3: u64 = M1 * M3;
    const M1M2: u64 = M1 * M2;
    const M1M2M3: u64 = M1M2.wrapping_mul(M3);

    modulus!(M1, M2, M3);

    if a.is_empty() || b.is_empty() {
        return vec![];
    }

    let (_, i1) = internal_math::inv_gcd(M2M3 as _, M1 as _);
    let (_, i2) = internal_math::inv_gcd(M1M3 as _, M2 as _);
    let (_, i3) = internal_math::inv_gcd(M1M2 as _, M3 as _);

    let c1 = convolution_raw::<i64, M1>(a, b);
    let c2 = convolution_raw::<i64, M2>(a, b);
    let c3 = convolution_raw::<i64, M3>(a, b);

    c1.into_iter()
        .zip(c2)
        .zip(c3)
        .map(|((c1, c2), c3)| {
            const OFFSET: &[u64] = &[0, 0, M1M2M3, 2 * M1M2M3, 3 * M1M2M3];

            let mut x = [(c1, i1, M1, M2M3), (c2, i2, M2, M1M3), (c3, i3, M3, M1M2)]
                .iter()
                .map(|&(c, i, m1, m2)| c.wrapping_mul(i).rem_euclid(m1 as _).wrapping_mul(m2 as _))
                .fold(0, i64::wrapping_add);

            let mut diff = c1 - internal_math::safe_mod(x, M1 as _);
            if diff < 0 {
                diff += M1 as i64;
            }
            x = x.wrapping_sub(OFFSET[diff.rem_euclid(5) as usize] as _);
            x
        })
        .collect()
}

fn butterfly<M: Modulus>(a: &mut [StaticModInt<M>]) {
    let n = a.len();
    let h = internal_bit::ceil_pow2(n as u32);

    M::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let ButterflyCache { sum_e, .. } = cache.get_or_insert_with(prepare);
        for ph in 1..=h {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut now = StaticModInt::<M>::new(1);
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p] * now;
                    a[i + offset] = l + r;
                    a[i + offset + p] = l - r;
                }
                now *= sum_e[(!s).trailing_zeros() as usize];
            }
        }
    });
}

fn butterfly_inv<M: Modulus>(a: &mut [StaticModInt<M>]) {
    let n = a.len();
    let h = internal_bit::ceil_pow2(n as u32);

    M::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let ButterflyCache { sum_ie, .. } = cache.get_or_insert_with(prepare);
        for ph in (1..=h).rev() {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut inow = StaticModInt::<M>::new(1);
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p];
                    a[i + offset] = l + r;
                    a[i + offset + p] = StaticModInt::new(M::VALUE + l.val() - r.val()) * inow;
                }
                inow *= sum_ie[(!s).trailing_zeros() as usize];
            }
        }
    });
}

fn prepare<M: Modulus>() -> ButterflyCache<M> {
    let g = StaticModInt::<M>::raw(internal_math::primitive_root(M::VALUE as i32) as u32);
    let mut es = [StaticModInt::<M>::raw(0); 30]; // es[i]^(2^(2+i)) == 1
    let mut ies = [StaticModInt::<M>::raw(0); 30];
    let cnt2 = (M::VALUE - 1).trailing_zeros() as usize;
    let mut e = g.pow(((M::VALUE - 1) >> cnt2).into());
    let mut ie = e.inv();
    for i in (2..=cnt2).rev() {
        es[i - 2] = e;
        ies[i - 2] = ie;
        e *= e;
        ie *= ie;
    }
    let sum_e = es
        .iter()
        .scan(StaticModInt::new(1), |acc, e| {
            *acc *= e;
            Some(*acc)
        })
        .collect();
    let sum_ie = ies
        .iter()
        .scan(StaticModInt::new(1), |acc, ie| {
            *acc *= ie;
            Some(*acc)
        })
        .collect();
    ButterflyCache { sum_e, sum_ie }
}
