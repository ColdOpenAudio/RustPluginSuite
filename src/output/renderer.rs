use crate::{
    output::{polar::polar_from_xy, sl_modes::map_mode, view::attenuate_for_noise},
    params::ViewMode,
    shared::frame::{Frame, Point},
};

pub fn render_points(
    frame: &Frame,
    mode: ViewMode,
    threshold: f32,
    attenuation: f32,
) -> Vec<Point> {
    let intensity = attenuate_for_noise(frame.rms, threshold, 1.0, attenuation);
    let points = map_mode(frame, mode, intensity);

    if mode == ViewMode::Polar {
        points.into_iter().map(polar_from_xy).collect()
    } else {
        points
    }
}
