use crate::shared::frame::Point;
pub fn map_polar(p: Point) -> Point {
    let r = (p.x * p.x + p.y * p.y).sqrt();
    let t = p.y.atan2(p.x);
    Point {
        x: r * t.cos(),
        y: r * t.sin(),
        intensity: p.intensity,
    }
}
