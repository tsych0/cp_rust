use super::modint::*;

pub struct Combinatorics<const MOD: u32> {
    fact: Vec<Mint<MOD>>,
    inv_fact: Vec<Mint<MOD>>,
}

impl<const MOD: u32> Combinatorics<MOD> {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![Mint::new(1); n + 1];
        for i in 1..=n {
            fact[i] = fact[i - 1] * Mint::new(i as i64);
        }

        let mut inv_fact = vec![Mint::new(1); n + 1];
        inv_fact[n] = fact[n].inv();
        for i in (0..n).rev() {
            inv_fact[i] = inv_fact[i + 1] * Mint::new((i + 1) as i64);
        }

        Self { fact, inv_fact }
    }

    pub fn ncr(&self, n: usize, r: usize) -> Mint<MOD> {
        if r > n {
            return Mint::new(0);
        }
        self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]
    }

    pub fn npr(&self, n: usize, r: usize) -> Mint<MOD> {
        if r > n {
            return Mint::new(0);
        }
        self.fact[n] * self.inv_fact[n - r]
    }
}
