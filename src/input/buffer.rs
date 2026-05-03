use std::collections::VecDeque;

use crate::shared::frame::Frame;

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    queue: VecDeque<Frame>,
    cap: usize,
}

impl FrameBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            cap: capacity,
        }
    }

    pub fn push(&mut self, frame: Frame) {
        if self.queue.len() == self.cap {
            let _ = self.queue.pop_front();
        }
        self.queue.push_back(frame);
    }

    pub fn pop(&mut self) -> Option<Frame> {
        self.queue.pop_front()
    }
}
