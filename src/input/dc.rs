#[derive(Debug, Clone)]
pub struct DcEstimator {
    alpha: f32,
    dc: f32,
}

impl DcEstimator {
    pub fn new(alpha: f32) -> Self {
        Self { alpha, dc: 0.0 }
    }

    pub fn update(&mut self, sample: f32) -> f32 {
        self.dc += self.alpha * (sample - self.dc);
        self.dc
    }

    pub fn remove(&self, sample: f32) -> f32 {
        sample - self.dc
    }

    pub fn value(&self) -> f32 {
        self.dc
    }
}
