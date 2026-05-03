use crate::shared::frame::{Frame, Point};

pub fn map_xy(frame: Frame, intensity: f32) -> Point {
    Point {
        x: frame.a,
        y: frame.b,
        intensity,
    }
}
