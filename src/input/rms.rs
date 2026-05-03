#[derive(Debug, Clone)]
pub struct RmsWindow {
    squares: Vec<f32>,
    sum: f32,
    idx: usize,
}

impl RmsWindow {
    pub fn new(size: usize) -> Self {
        Self {
            squares: vec![0.0; size.max(1)],
            sum: 0.0,
            idx: 0,
        }
    }
    pub fn push(&mut self, x: f32) -> f32 {
        let s = x * x;
        self.sum -= self.squares[self.idx];
        self.squares[self.idx] = s;
        self.sum += s;
        self.idx = (self.idx + 1) % self.squares.len();
        (self.sum / self.squares.len() as f32).sqrt()
    }
}
