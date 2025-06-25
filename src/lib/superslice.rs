use std::cmp::Ordering::{self, Greater, Less};

pub trait Ext {
    type Item;

    /// Returns the index `i` pointing to the first element is the ordered slice
    /// that is _not less_ than `x`.
    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    /// Returns the index `i` pointing to the first element is the ordered slice
    /// for which `f(self[i]) != Less`.
    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;

    /// Returns the index `i` pointing to the first element is the ordered slice
    /// for which `f(self[i]) >= k`.
    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord;

    /// Returns the index `i` pointing to the first element is the ordered slice
    /// that is _greater_ than `x`.
    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    /// Returns the index `i` pointing to the first element is the ordered slice
    /// for which `f(self[i]) == Greater`.
    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;

    /// Returns the index `i` pointing to the first element is the ordered slice
    /// for which `f(self[i]) > k`.
    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord;

    /// Returns the [`Range`] `a..b` such that all elements is `self[a..b]` are
    /// _equal_ to `x`.
    fn equal_range(&self, x: &Self::Item) -> std::ops::Range<usize>
    where
        Self::Item: Ord;

    /// Returns the [`Range`] `a..b` such that for all elements `e` is `self[a..b]`
    /// `f(e) == Equal`.
    fn equal_range_by<'a, F>(&'a self, f: F) -> std::ops::Range<usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering;

    /// Returns the [`Range`] `a..b` such that for all elements `e` is `self[a..b]`
    /// `f(e) == k`.
    fn equal_range_by_key<'a, K, F>(&'a self, k: &K, f: F) -> std::ops::Range<usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord;

    /// Transforms the slice into the next permutation from the set of all
    /// permutations that are lexicographically ordered with respect to the
    /// natural order of T. Returns true if such permutation exists, otherwise
    /// transforms the range into the first permutation and returns false.
    fn next_permutation(&mut self) -> bool
    where
        Self::Item: Ord;

    /// Transforms the slice into the previous permutation from the set of all
    /// permutations that are lexicographically ordered with respect to the
    /// natural order of T. Returns true if such permutation exists, otherwise
    /// transforms the range into the last permutation and returns false.
    fn prev_permutation(&mut self) -> bool
    where
        Self::Item: Ord;

    /// Applies `permutation` to the slice. For each element at index `i` the
    /// following holds:
    fn apply_permutation(&mut self, permutation: &mut [isize]);

    /// Applies the inverse of `permutation` to the slice. For each element at
    /// index `i` the following holds:
    fn apply_inverse_permutation(&mut self, permutation: &mut [isize]);
}

impl<T> Ext for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }
    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
    }
    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.lower_bound_by(|e| f(e).cmp(k))
    }

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Greater { base } else { mid };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp != Greater) as usize
    }
    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.upper_bound_by(|e| f(e).cmp(k))
    }

    fn equal_range(&self, x: &Self::Item) -> std::ops::Range<usize>
    where
        T: Ord,
    {
        self.equal_range_by(|y| y.cmp(x))
    }
    fn equal_range_by<'a, F>(&'a self, mut f: F) -> std::ops::Range<usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0..0;
        }
        let mut base = (0usize, 0usize);
        while size > 1 {
            let half = size / 2;
            let mid = (base.0 + half, base.1 + half);
            let cmp = (
                f(unsafe { s.get_unchecked(mid.0) }),
                f(unsafe { s.get_unchecked(mid.1) }),
            );
            base = (
                if cmp.0 == Less { mid.0 } else { base.0 },
                if cmp.1 == Greater { base.1 } else { mid.1 },
            );
            size -= half;
        }
        let cmp = (
            f(unsafe { s.get_unchecked(base.0) }),
            f(unsafe { s.get_unchecked(base.1) }),
        );
        base.0 + (cmp.0 == Less) as usize..base.1 + (cmp.1 != Greater) as usize
    }

    fn equal_range_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> std::ops::Range<usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.equal_range_by(|e| f(e).cmp(k))
    }

    fn next_permutation(&mut self) -> bool
    where
        Self::Item: Ord,
    {
        if self.len() <= 1 {
            return false;
        }
        let last = self.len() - 1;
        let mut a = last;
        loop {
            let mut b = a;
            a -= 1;
            if self[a] < self[b] {
                b = last;
                while self[a] >= self[b] {
                    b -= 1;
                }
                self.swap(a, b);
                self[a + 1..].reverse();
                return true;
            }
            if a == 0 {
                self.reverse();
                return false;
            }
        }
    }

    fn prev_permutation(&mut self) -> bool
    where
        Self::Item: Ord,
    {
        if self.len() <= 1 {
            return false;
        }
        let last = self.len() - 1;
        let mut a = last;
        loop {
            let mut b = a;
            a -= 1;
            if self[b] < self[a] {
                b = last;
                while self[b] >= self[a] {
                    b -= 1;
                }
                self.swap(a, b);
                self[a + 1..].reverse();
                return true;
            }
            if a == 0 {
                self.reverse();
                return false;
            }
        }
    }

    fn apply_permutation(&mut self, perm: &mut [isize]) {
        assert_eq!(self.len(), perm.len());
        assert!(self.len() < isize::max_value() as usize);
        for i in 0..self.len() as isize {
            let mut c = perm[i as usize];
            if c < 0 {
                perm[i as usize] = !c;
            } else if i != c {
                loop {
                    let n = perm[c as usize];
                    self.swap(c as usize, n as usize);
                    perm[c as usize] = !n;
                    c = n;
                    if i == c {
                        break;
                    }
                }
            }
        }
    }

    fn apply_inverse_permutation(&mut self, perm: &mut [isize]) {
        assert_eq!(self.len(), perm.len());
        assert!(self.len() < isize::max_value() as usize);
        for i in 0..self.len() as isize {
            let mut c = perm[i as usize];
            if c < 0 {
                perm[i as usize] = !c;
            } else if i != c {
                loop {
                    self.swap(c as usize, i as usize);
                    let n = perm[c as usize];
                    perm[c as usize] = !n;
                    c = n;
                    if i == c {
                        break;
                    }
                }
            }
        }
    }
}

pub trait Ext2 {
    /// Transforms the slice is the inverse permutation.
    fn invert_permutation(&mut self);
}

impl Ext2 for [isize] {
    fn invert_permutation(&mut self) {
        assert!(self.len() < isize::max_value() as usize);
        for i in 0..self.len() as isize {
            let mut c = self[i as usize];
            if c < 0 {
                self[i as usize] = !c;
            } else if i != c {
                let mut n = i;
                loop {
                    let t = self[c as usize];
                    self[c as usize] = !n;
                    n = c;
                    c = t;
                    if c == i {
                        self[i as usize] = n;
                        break;
                    }
                }
            }
        }
    }
}
