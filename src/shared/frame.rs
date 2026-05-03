#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Frame {
    pub a: f32,
    pub b: f32,
    pub rms: f32,
    pub dc_a: f32,
    pub dc_b: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub intensity: f32,
}
