#[derive(Debug, Clone, Copy)]
pub struct LookaheadParams {
    pub enabled: bool,
    pub time_ms: f32,
}

#[derive(Debug, Clone)]
pub struct LookaheadBuffer {
    left: Vec<f32>,
    right: Vec<f32>,
    write_idx: usize,
    sample_rate: f32,
}

impl LookaheadBuffer {
    pub fn new(max_delay_samples: usize, sample_rate: f32) -> Self {
        Self {
            left: vec![0.0; max_delay_samples.max(1)],
            right: vec![0.0; max_delay_samples.max(1)],
            write_idx: 0,
            sample_rate,
        }
    }

    pub fn process(&mut self, left: f32, right: f32, params: LookaheadParams) -> (f32, f32) {
        if !params.enabled {
            return (left, right);
        }

        let delay_samples = ((params.time_ms * 0.001) * self.sample_rate)
            .round()
            .clamp(0.0, (self.left.len() - 1) as f32) as usize;
        let read_idx = (self.write_idx + self.left.len() - delay_samples) % self.left.len();
        let out = (self.left[read_idx], self.right[read_idx]);

        self.left[self.write_idx] = left;
        self.right[self.write_idx] = right;
        self.write_idx = (self.write_idx + 1) % self.left.len();

        out
    }
}
