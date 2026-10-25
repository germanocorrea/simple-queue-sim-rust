pub struct RandomGenerator {
    last_random: u128,
    randoms_used: u128,
}

impl RandomGenerator {
    pub fn new(seed: u128) -> RandomGenerator {
        return RandomGenerator {
            last_random: seed,
            randoms_used: 0,
        };
    }

    pub fn generate(&mut self) -> f64 {
        self.randoms_used += 1;
        let m = 2_u128.pow(27);
        let a = 545643;
        let c = 76785897;
        self.last_random = ((a * self.last_random) + c) % m;
        return self.last_random as f64 / m as f64;
    }
}
