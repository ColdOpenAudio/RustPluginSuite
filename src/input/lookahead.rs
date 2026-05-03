#[derive(Debug, Clone)]
pub struct LookaheadBuffer {
    left: Vec<f32>,
    right: Vec<f32>,
    write_idx: usize,
    max_delay_samples: usize,
}

impl LookaheadBuffer {
    pub fn new(max_delay_samples: usize) -> Self {
        let len = max_delay_samples.max(1) + 1;
        Self {
            left: vec![0.0; len],
            right: vec![0.0; len],
            write_idx: 0,
            max_delay_samples,
        }
    }

    pub fn process(&mut self, l: f32, r: f32, enabled: bool, delay_samples: usize) -> (f32, f32) {
        let len = self.left.len();
        let delay = if enabled {
            delay_samples.min(self.max_delay_samples)
        } else {
            0
        };
        let read_idx = (self.write_idx + len - delay % len) % len;
        let out = (self.left[read_idx], self.right[read_idx]);
        self.left[self.write_idx] = l;
        self.right[self.write_idx] = r;
        self.write_idx = (self.write_idx + 1) % len;
        if enabled { out } else { (l, r) }
    }
}
