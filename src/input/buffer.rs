use crate::shared::frame::Frame;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct FrameBuffer {
    q: VecDeque<Frame>,
    cap: usize,
}
impl FrameBuffer {
    pub fn new(cap: usize) -> Self {
        Self {
            q: VecDeque::with_capacity(cap),
            cap: cap.max(1),
        }
    }
    pub fn push(&mut self, f: Frame) {
        if self.q.len() == self.cap {
            self.q.pop_front();
        }
        self.q.push_back(f);
    }
    pub fn pop(&mut self) -> Option<Frame> {
        self.q.pop_front()
    }
}
