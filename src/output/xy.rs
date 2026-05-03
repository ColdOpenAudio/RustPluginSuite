use crate::shared::frame::{Frame, Point};
pub fn map_xy(f: Frame, intensity: f32) -> Point {
    Point {
        x: f.a,
        y: f.b,
        intensity,
    }
}
