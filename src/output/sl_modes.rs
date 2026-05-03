use crate::{
    params::ViewMode,
    shared::frame::{Frame, Point},
};

pub fn map_mode(frame: &Frame, mode: ViewMode, intensity: f32) -> Vec<Point> {
    let sum = frame.a + frame.b;
    let diff = frame.a - frame.b;
    match mode {
        ViewMode::XY => vec![Point {
            x: frame.a,
            y: frame.b,
            intensity,
        }],
        ViewMode::Polar => vec![Point {
            x: frame.a,
            y: frame.b,
            intensity,
        }],
        ViewMode::SumDiff => vec![Point {
            x: sum,
            y: diff,
            intensity,
        }],
        ViewMode::LissSum => vec![Point {
            x: frame.a,
            y: sum,
            intensity,
        }],
        ViewMode::DiffOnly => vec![Point {
            x: diff,
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
