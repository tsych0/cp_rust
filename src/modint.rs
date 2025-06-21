use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mint<const MOD: u32> {
    val: u32,
}

impl<const MOD: u32> Mint<MOD> {
    pub fn new(mut val: i64) -> Self {
        val %= MOD as i64;
        if val < 0 {
            val += MOD as i64;
        }
        Self { val: val as u32 }
    }

    pub fn raw(val: u32) -> Self {
        Self { val }
    }

    pub fn get(&self) -> u32 {
        self.val
    }

    pub fn pow(&self, mut exp: u64) -> Self {
        let mut result = Self::new(1);
        let mut base = *self;

        while exp > 0 {
            if exp & 1 == 1 {
                result *= base;
            }
            base *= base;
            exp >>= 1;
        }
        result
    }

    pub fn inv(&self) -> Self {
        self.pow((MOD - 2) as u64)
    }
}

impl<const MOD: u32> Add for Mint<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let sum = self.val + rhs.val;
        Self::raw(if sum >= MOD { sum - MOD } else { sum })
    }
}

impl<const MOD: u32> Sub for Mint<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::raw(if self.val >= rhs.val {
            self.val - rhs.val
        } else {
            self.val + MOD - rhs.val
        })
    }
}

impl<const MOD: u32> Mul for Mint<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::raw(((self.val as u64 * rhs.val as u64) % MOD as u64) as u32)
    }
}

impl<const MOD: u32> Div for Mint<MOD> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

// Implement assignment operators
impl<const MOD: u32> AddAssign for Mint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: u32> SubAssign for Mint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: u32> MulAssign for Mint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: u32> DivAssign for Mint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

pub type ModInt = Mint<1000000007>;
