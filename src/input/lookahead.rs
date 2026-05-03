#[derive(Debug, Clone)]
pub struct Lookahead {
    left: Vec<f32>,
    right: Vec<f32>,
    idx: usize,
}

impl Lookahead {
    pub fn new(max_delay_samples: usize) -> Self {
        let len = max_delay_samples.max(1) + 1;
        Self {
            left: vec![0.0; len],
            right: vec![0.0; len],
            idx: 0,
        }
    }

    pub fn process(
        &mut self,
        left_in: f32,
        right_in: f32,
        enabled: bool,
        delay_samples: usize,
    ) -> (f32, f32) {
        if !enabled || delay_samples == 0 {
            return (left_in, right_in);
        }
        let delay = delay_samples.min(self.left.len() - 1);
        self.left[self.idx] = left_in;
        self.right[self.idx] = right_in;
        let read = (self.idx + self.left.len() - delay) % self.left.len();
        let out = (self.left[read], self.right[read]);
        self.idx = (self.idx + 1) % self.left.len();
        out
    }
}
