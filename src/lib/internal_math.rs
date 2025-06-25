// remove this after dependencies has been added
use std::{mem::swap, num::Wrapping as W};

/// # Arguments
/// * `m` `1 <= m`
///
/// # Returns
/// x mod m
pub(crate) fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

/// Fast modular by barrett reduction
/// Reference: https://en.wikipedia.org/wiki/Barrett_reduction
/// NOTE: reconsider after Ice Lake
pub(crate) struct Barrett {
    pub(crate) _m: u32,
    pub(crate) im: u64,
}

impl Barrett {
    /// # Arguments
    /// * `m` `1 <= m`
    pub(crate) fn new(m: u32) -> Barrett {
        Barrett {
            _m: m,
            im: (-1i64 as u64 / m as u64).wrapping_add(1),
        }
    }

    /// # Returns
    /// a * b % m
    pub(crate) fn mul(&self, a: u32, b: u32) -> u32 {
        mul_mod(a, b, self._m, self.im)
    }
}

/// Calculates `a * b % m`.
#[allow(clippy::many_single_char_names)]
pub(crate) fn mul_mod(a: u32, b: u32, m: u32, im: u64) -> u32 {
    let mut z = a as u64;
    z *= b as u64;
    let x = (((z as u128) * (im as u128)) >> 64) as u64;
    let mut v = z.wrapping_sub(x.wrapping_mul(m as u64)) as u32;
    if m <= v {
        v = v.wrapping_add(m);
    }
    v
}

/// # Returns
/// `(x ** n) % m`
#[allow(clippy::many_single_char_names)]
pub(crate) fn pow_mod(x: i64, mut n: i64, m: i32) -> i64 {
    if m == 1 {
        return 0;
    }
    let _m = m as u32;
    let mut r: u64 = 1;
    let mut y: u64 = safe_mod(x, m as i64) as u64;
    while n != 0 {
        if (n & 1) > 0 {
            r = (r * y) % (_m as u64);
        }
        y = (y * y) % (_m as u64);
        n >>= 1;
    }
    r as i64
}

/// Fast Primality Testing for Integers That Fit into a Machine Word
pub(crate) fn is_prime(n: i32) -> bool {
    let n = n as i64;
    match n {
        _ if n <= 1 => return false,
        2 | 7 | 61 => return true,
        _ if n % 2 == 0 => return false,
        _ => {}
    }
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    for &a in &[2, 7, 61] {
        let mut t = d;
        let mut y = pow_mod(a, t, n as i32);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = y * y % n;
            t <<= 1;
        }
        if y != n - 1 && t % 2 == 0 {
            return false;
        }
    }
    true
}

/// # Returns
/// (g, x) s.t. g = gcd(a, b), xa = g (mod b), 0 <= x < b/g
pub(crate) fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let a = safe_mod(a, b);
    if a == 0 {
        return (b, 0);
    }

    let mut s = b;
    let mut t = a;
    let mut m0 = 0;
    let mut m1 = 1;

    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u; // |m1 * u| <= |m1| * s <= b

        swap(&mut s, &mut t);
        swap(&mut m0, &mut m1);
    }
    if m0 < 0 {
        m0 += b / s;
    }
    (s, m0)
}

/// @param m must be prime
/// @return primitive root (and minimum is now)
pub(crate) fn primitive_root(m: i32) -> i32 {
    match m {
        2 => return 1,
        167_772_161 => return 3,
        469_762_049 => return 3,
        754_974_721 => return 11,
        998_244_353 => return 3,
        _ => {}
    }

    let mut divs = [0; 20];
    divs[0] = 2;
    let mut cnt = 1;
    let mut x = (m - 1) / 2;
    while x % 2 == 0 {
        x /= 2;
    }
    for i in (3..i32::MAX).step_by(2) {
        if i as i64 * i as i64 > x as i64 {
            break;
        }
        if x % i == 0 {
            divs[cnt] = i;
            cnt += 1;
            while x % i == 0 {
                x /= i;
            }
        }
    }
    if x > 1 {
        divs[cnt] = x;
        cnt += 1;
    }
    let mut g = 2;
    loop {
        if (0..cnt).all(|i| pow_mod(g, ((m - 1) / divs[i]) as i64, m) != 1) {
            break g as i32;
        }
        g += 1;
    }
}

/// # Returns
/// `sum_{i=0}^{n-1} floor((ai + b) / m) (mod 2^64)`
/* const */
#[allow(clippy::many_single_char_names)]
pub(crate) fn floor_sum_unsigned(
    mut n: W<u64>,
    mut m: W<u64>,
    mut a: W<u64>,
    mut b: W<u64>,
) -> W<u64> {
    let mut ans = W(0);
    loop {
        if a >= m {
            if n > W(0) {
                ans += n * (n - W(1)) / W(2) * (a / m);
            }
            a %= m;
        }
        if b >= m {
            ans += n * (b / m);
            b %= m;
        }

        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        // y_max < m * (n + 1)
        // floor(y_max / m) <= n
        n = y_max / m;
        b = y_max % m;
        std::mem::swap(&mut m, &mut a);
    }
    ans
}
