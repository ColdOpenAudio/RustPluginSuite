#[derive(Debug, Clone)]
pub struct RmsWindow {
    squares: Vec<f32>,
    index: usize,
    sum: f32,
}

impl RmsWindow {
    pub fn new(size: usize) -> Self {
        Self {
            squares: vec![0.0; size.max(1)],
            index: 0,
            sum: 0.0,
        }
    }

    pub fn push(&mut self, sample: f32) -> f32 {
        let sq = sample * sample;
        self.sum -= self.squares[self.index];
        self.squares[self.index] = sq;
        self.sum += sq;
        self.index = (self.index + 1) % self.squares.len();
        (self.sum / self.squares.len() as f32).sqrt()
    }
}
