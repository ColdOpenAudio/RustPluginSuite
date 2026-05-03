use crate::shared::frame::Point;
#[derive(Default, Debug)]
pub struct Renderer {
    pub last_points: Vec<Point>,
}
impl Renderer {
    pub fn render(&mut self, points: Vec<Point>) {
        self.last_points = points;
    }
}
