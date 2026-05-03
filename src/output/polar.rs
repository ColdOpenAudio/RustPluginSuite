use crate::shared::frame::Point;

pub fn polar_from_xy(p: Point) -> Point {
    let r = (p.x * p.x + p.y * p.y).sqrt();
    let theta = p.y.atan2(p.x);
    Point {
        x: r,
        y: theta,
        intensity: p.intensity,
    }
}
