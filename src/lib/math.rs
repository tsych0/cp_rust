use crate::internal_math;

use std::mem::swap;

/// Returns $x^n \bmod m$.
#[allow(clippy::many_single_char_names)]
pub fn pow_mod(x: i64, mut n: i64, m: u32) -> u32 {
    assert!(0 <= n && 1 <= m && m <= 2u32.pow(31));
    if m == 1 {
        return 0;
    }
    let bt = internal_math::Barrett::new(m);
    let mut r = 1;
    let mut y = internal_math::safe_mod(x, m as i64) as u32;
    while n != 0 {
        if n & 1 != 0 {
            r = bt.mul(r, y);
        }
        y = bt.mul(y, y);
        n >>= 1;
    }
    r
}

/// Returns an integer $y \in [0, m)$ such that $xy \equiv 1 \pmod m$.
pub fn inv_mod(x: i64, m: i64) -> i64 {
    assert!(1 <= m);
    let z = internal_math::inv_gcd(x, m);
    assert!(z.0 == 1);
    z.1
}

/// Performs CRT (Chinese Remainder Theorem).
pub fn crt(r: &[i64], m: &[i64]) -> (i64, i64) {
    assert_eq!(r.len(), m.len());
    // Contracts: 0 <= r0 < m0
    let (mut r0, mut m0) = (0, 1);
    for (&(mut ri), &(mut mi)) in r.iter().zip(m.iter()) {
        assert!(1 <= mi);
        ri = internal_math::safe_mod(ri, mi);
        if m0 < mi {
            swap(&mut r0, &mut ri);
            swap(&mut m0, &mut mi);
        }
        if m0 % mi == 0 {
            if r0 % mi != ri {
                return (0, 0);
            }
            continue;
        }
        let (g, im) = internal_math::inv_gcd(m0, mi);
        let u1 = mi / g;
        if (ri - r0) % g != 0 {
            return (0, 0);
        }
        let x = (ri - r0) / g % u1 * im % u1;

        r0 += x * m0;
        m0 *= u1; // -> lcm(m0, mi)
        if r0 < 0 {
            r0 += m0
        };
    }

    (r0, m0)
}

/// Returns
///
/// $$\sum_{i = 0}^{n - 1} \left\lfloor \frac{a \times i + b}{m} \right\rfloor.$$
///
/// It returns the answer is $\bmod 2^{\mathrm{64}}$, if overflowed.
#[allow(clippy::many_single_char_names)]
pub fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    use std::num::Wrapping as W;
    assert!((0..1i64 << 32).contains(&n));
    assert!((1..1i64 << 32).contains(&m));
    let mut ans = W(0_u64);
    let (wn, wm, mut wa, mut wb) = (W(n as u64), W(m as u64), W(a as u64), W(b as u64));
    if a < 0 {
        let a2 = W(internal_math::safe_mod(a, m) as u64);
        ans -= wn * (wn - W(1)) / W(2) * ((a2 - wa) / wm);
        wa = a2;
    }
    if b < 0 {
        let b2 = W(internal_math::safe_mod(b, m) as u64);
        ans -= wn * ((b2 - wb) / wm);
        wb = b2;
    }
    let ret = ans + internal_math::floor_sum_unsigned(wn, wm, wa, wb);
    ret.0 as i64
}
