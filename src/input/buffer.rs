use std::collections::VecDeque;

use crate::shared::frame::Frame;

#[derive(Debug)]
pub struct FrameRingBuffer {
    buf: VecDeque<Frame>,
    capacity: usize,
}

impl FrameRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(capacity.max(1)),
            capacity: capacity.max(1),
        }
    }

    pub fn push(&mut self, frame: Frame) {
        if self.buf.len() == self.capacity {
            let _ = self.buf.pop_front();
        }
        self.buf.push_back(frame);
    }

    pub fn pop(&mut self) -> Option<Frame> {
        self.buf.pop_front()
    }
}
