use crate::shared::frame::Point;

pub fn xy_to_polar(x: f32, y: f32, intensity: f32) -> Point {
    let r = (x * x + y * y).sqrt();
    let theta = y.atan2(x);
    Point {
        x: r,
        y: theta,
        intensity,
    }
}
