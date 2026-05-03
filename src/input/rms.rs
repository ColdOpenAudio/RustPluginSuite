#[derive(Debug, Clone)]
pub struct RmsWindow {
    buf: Vec<f32>,
    pos: usize,
    sum_squares: f32,
}

impl RmsWindow {
    pub fn new(size: usize) -> Self {
        Self {
            buf: vec![0.0; size.max(1)],
            pos: 0,
            sum_squares: 0.0,
        }
    }

    pub fn push(&mut self, x: f32) -> f32 {
        let old = self.buf[self.pos];
        self.sum_squares -= old * old;
        self.buf[self.pos] = x;
        self.sum_squares += x * x;
        self.pos = (self.pos + 1) % self.buf.len();
        (self.sum_squares / self.buf.len() as f32).sqrt()
    }
}
