pub struct Sieve {
    spf: Vec<usize>, // smallest prime factor
}

impl Sieve {
    pub fn new(n: usize) -> Self {
        let mut spf: Vec<usize> = (0..n).collect();

        for i in 2.. {
            if i * i >= n {
                break;
            }
            if spf[i] == i {
                for j in (i * i..n).step_by(i) {
                    if spf[j] == j {
                        spf[j] = i;
                    }
                }
            }
        }

        Self { spf }
    }

    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        let mut factors = Vec::new();
        while x != 1 {
            let d = self.spf[x];
            factors.push(d);
            while x % d == 0 {
                x /= d;
            }
        }
        factors
    }

    pub fn is_prime(&self, x: usize) -> bool {
        x > 1 && self.spf[x] == x
    }
}
