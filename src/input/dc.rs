#[derive(Debug, Clone)]
pub struct DcTracker {
    alpha: f32,
    mean: f32,
}

impl DcTracker {
    pub fn new(alpha: f32) -> Self {
        Self { alpha, mean: 0.0 }
    }

    pub fn update(&mut self, x: f32) -> f32 {
        self.mean += self.alpha * (x - self.mean);
        self.mean
    }

    pub fn remove(&self, x: f32) -> f32 {
        x - self.mean
    }
}
