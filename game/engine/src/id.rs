use std::ops::AddAssign;

pub type Id = u64;

pub struct IdAccumulator(Vec<Id>);

impl IdAccumulator {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn finish(self) -> Vec<Id> {
        self.0
    }
}

impl AddAssign<Id> for IdAccumulator {
    fn add_assign(&mut self, rhs: Id) {
        self.0.push(rhs);
    }
}
