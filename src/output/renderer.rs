use crate::{
    params::ViewMode,
    shared::frame::{Frame, Point},
};

use super::view::map_view;

#[derive(Debug, Clone)]
pub struct Renderer {
    pub noise_threshold: f32,
    pub attenuation: f32,
}

impl Renderer {
    pub fn intensity_for(&self, frame: &Frame) -> f32 {
        if frame.rms < self.noise_threshold {
            self.attenuation
        } else {
            1.0
        }
    }

    pub fn render(&self, frame: Frame, mode: ViewMode) -> Vec<Point> {
        map_view(frame, mode, self.intensity_for(&frame))
    }
}
