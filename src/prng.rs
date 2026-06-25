pub struct Prng {
    state: u64,
}

impl Prng {
    pub fn new(state: u64) -> Self {
        Self { state }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = (self.state).wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state ^ (self.state >> 22)) >> (22 + (self.state >> 61))
    }
}
