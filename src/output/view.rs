use super::{polar::map_polar, sl_modes::map_view};
use crate::{
    params::{PluginParams, ViewMode},
    shared::frame::{Frame, Point},
};

pub fn frame_to_points(frame: Frame, p: PluginParams) -> Vec<Point> {
    let intensity = if frame.rms < p.noise_threshold {
        0.25
    } else {
        1.0
    };
    let pts = map_view(frame, p.view_mode, intensity);
    if matches!(p.view_mode, ViewMode::Polar) {
        pts.into_iter().map(map_polar).collect()
    } else {
        pts
    }
}
