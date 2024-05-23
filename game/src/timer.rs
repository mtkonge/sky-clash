#[derive(Clone, Debug)]
pub struct Timer {
    duration: f64,
    remaining: f64,
}

impl Timer {
    pub fn new(duration: f64) -> Self {
        Self {
            duration,
            remaining: duration,
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.remaining -= delta;
    }

    pub fn reset(&mut self) {
        self.remaining = self.duration;
    }

    pub fn done(&self) -> bool {
        self.remaining <= 0.0
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn time_remaining(&self) -> f64 {
        self.remaining
    }

    pub fn time_passed(&self) -> f64 {
        self.duration - self.remaining
    }
}
