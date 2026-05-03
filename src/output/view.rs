use crate::{
    params::ViewMode,
    shared::frame::{Frame, Point},
};

use super::{polar::xy_to_polar, sl_modes::sum_diff, xy::map_xy};

pub fn map_view(frame: Frame, mode: ViewMode, intensity: f32) -> Vec<Point> {
    match mode {
        ViewMode::Xy => vec![map_xy(frame, intensity)],
        ViewMode::Polar => vec![xy_to_polar(frame.a, frame.b, intensity)],
        ViewMode::SumDiff => {
            let (x, y) = sum_diff(frame);
            vec![Point { x, y, intensity }]
        }
        ViewMode::LissSum => vec![Point {
            x: frame.a,
            y: frame.a + frame.b,
            intensity,
        }],
        ViewMode::DiffOnly => vec![Point {
            x: frame.a - frame.b,
            y: 0.0,
            intensity,
        }],
        ViewMode::DualTrace => vec![
            Point {
                x: frame.a,
                y: 0.0,
                intensity,
            },
            Point {
                x: frame.b,
                y: 0.0,
                intensity,
            },
        ],
    }
}
