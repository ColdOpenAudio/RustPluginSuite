use crate::shared::frame::Frame;

pub fn sum_diff(frame: Frame) -> (f32, f32) {
    (frame.a + frame.b, frame.a - frame.b)
}
