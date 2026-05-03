pub mod context;
pub mod error;
pub mod framework;
pub mod operators;
pub mod plugin;

pub mod params;
pub mod input {
    pub mod buffer;
    pub mod dc;
    pub mod lookahead;
    pub mod ms;
    pub mod process;
    pub mod rms;
}
pub mod output {
    pub mod polar;
    pub mod renderer;
    pub mod sl_modes;
    pub mod view;
    pub mod xy;
}
pub mod shared {
    pub mod frame;
}

pub use context::PluginContext;
pub use error::{NihError, NihResult};
pub use framework::{Framework, FrameworkBuilder, FrameworkReport};
pub use input::process::InputProcessor;
pub use operators::{
    BassGoBrrrOperator, FrequencyGateOperator, OperatorSuite, OscilloscopeOperator,
    StereoscopeOperator, SubOperator, VstDescriptor,
};
pub use params::{ChannelMode, PluginParams, ViewMode};
pub use plugin::{NihPlugin, PluginMetadata};
pub use shared::frame::{Frame, Point};

#[cfg(test)]
mod osc_tests {
    use crate::{
        ChannelMode, PluginParams, ViewMode, input::process::InputProcessor,
        output::view::frame_to_points,
    };

    #[test]
    fn ms_mode_and_sum_diff_work() {
        let mut p = PluginParams {
            channel_mode: ChannelMode::Ms,
            view_mode: ViewMode::SumDiff,
            ..Default::default()
        };
        let mut proc = InputProcessor::new(48_000.0, 16, 200.0);
        let frame = proc.process_sample(1.0, -1.0, p);
        assert!((frame.a - 0.0).abs() < 1e-6);
        assert!((frame.b - 1.0).abs() < 1e-6);
        let points = frame_to_points(frame, p);
        assert_eq!(points.len(), 1);
        assert!((points[0].x - 1.0).abs() < 1e-6);
        assert!((points[0].y + 1.0).abs() < 1e-6);
        p.view_mode = ViewMode::DualTrace;
        assert_eq!(frame_to_points(frame, p).len(), 2);
    }

    #[test]
    fn lookahead_disabled_has_zero_latency_and_noise_only_modulates_intensity() {
        let mut proc = InputProcessor::new(48_000.0, 4, 100.0);
        let p = PluginParams {
            noise_threshold: 10.0,
            ..Default::default()
        };
        let frame = proc.process_sample(0.3, 0.1, p);
        assert!((frame.a - 0.3).abs() < 1e-6);
        let pts = frame_to_points(frame, p);
        assert_eq!(pts[0].intensity, 0.25);
    }
}
