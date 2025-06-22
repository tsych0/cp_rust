use crate::internal_bit::ceil_pow2;
use crate::internal_type_traits::{BoundedAbove, BoundedBelow, One, Zero};
use std::cmp::{max, min};
use std::convert::Infallible;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Add, BitAnd, BitOr, BitXor, Bound, Mul, Not, RangeBounds};

// TODO Should I split monoid-related traits to another module?
pub trait Monoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct Max<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for Max<S>
where
    S: Copy + Ord + BoundedBelow,
{
    type S = S;
    fn identity() -> Self::S {
        S::min_value()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        max(*a, *b)
    }
}

pub struct Min<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for Min<S>
where
    S: Copy + Ord + BoundedAbove,
{
    type S = S;
    fn identity() -> Self::S {
        S::max_value()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        min(*a, *b)
    }
}

pub struct Additive<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for Additive<S>
where
    S: Copy + Add<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
}

pub struct Multiplicative<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for Multiplicative<S>
where
    S: Copy + Mul<Output = S> + One,
{
    type S = S;
    fn identity() -> Self::S {
        S::one()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a * *b
    }
}

pub struct BitwiseOr<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for BitwiseOr<S>
where
    S: Copy + BitOr<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a | *b
    }
}

pub struct BitwiseAnd<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for BitwiseAnd<S>
where
    S: Copy + BitAnd<Output = S> + Not<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        !S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a & *b
    }
}

pub struct BitwiseXor<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for BitwiseXor<S>
where
    S: Copy + BitXor<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
    }
}

impl<M: Monoid> Default for Segtree<M> {
    fn default() -> Self {
        Segtree::new(0)
    }
}
impl<M: Monoid> Segtree<M> {
    pub fn new(n: usize) -> Segtree<M> {
        vec![M::identity(); n].into()
    }
}
impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        let mut d = vec![M::identity(); 2 * size];
        d[size..][..n].clone_from_slice(&v);
        let mut ret = Segtree { n, size, log, d };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<M: Monoid> FromIterator<M::S> for Segtree<M> {
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        v.into()
    }
}
impl<M: Monoid> Segtree<M> {
    pub fn set(&mut self, mut p: usize, x: M::S) {
        assert!(p < self.n);
        p += self.size;
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&self, p: usize) -> M::S {
        assert!(p < self.n);
        self.d[p + self.size].clone()
    }

    pub fn get_slice(&self) -> &[M::S] {
        &self.d[self.size..][..self.n]
    }

    pub fn prod<R>(&self, range: R) -> M::S
    where
        R: RangeBounds<usize>,
    {
        // Trivial optimization
        if range.start_bound() == Bound::Unbounded && range.end_bound() == Bound::Unbounded {
            return self.all_prod();
        }

        let mut r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.n,
        };
        let mut l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            // TODO: There are another way of optimizing [0..r)
            Bound::Unbounded => 0,
        };

        assert!(l <= r && r <= self.n);
        let mut sml = M::identity();
        let mut smr = M::identity();
        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        M::binary_operation(&sml, &smr)
    }

    pub fn all_prod(&self) -> M::S {
        self.d[1].clone()
    }

    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(&M::identity()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = M::identity();
        while {
            // do
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::binary_operation(&sm, &self.d[l])) {
                while l < self.size {
                    l *= 2;
                    let res = M::binary_operation(&sm, &self.d[l]);
                    if f(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = M::binary_operation(&sm, &self.d[l]);
            l += 1;
            // while
            {
                let l = l as isize;
                (l & -l) != l
            }
        } {}
        self.n
    }

    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(&M::identity()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = M::identity();
        while {
            // do
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(&M::binary_operation(&self.d[r], &sm)) {
                while r < self.size {
                    r = 2 * r + 1;
                    let res = M::binary_operation(&self.d[r], &sm);
                    if f(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::binary_operation(&self.d[r], &sm);
            // while
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }

    fn update(&mut self, k: usize) {
        self.d[k] = M::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
    }
}

#[derive(Clone)]
pub struct Segtree<M>
where
    M: Monoid,
{
    n: usize,
    size: usize,
    log: usize,
    d: Vec<M::S>,
}
