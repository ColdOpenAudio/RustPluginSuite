pub mod context;
pub mod error;
pub mod framework;
pub mod input;
pub mod operators;
pub mod output;
pub mod params;
pub mod plugin;
pub mod shared;

pub use context::PluginContext;
pub use error::{NihError, NihResult};
pub use framework::{Framework, FrameworkBuilder, FrameworkReport};
pub use plugin::{NihPlugin, PluginMetadata};

pub use operators::{
    BassGoBrrrOperator, FrequencyGateOperator, OperatorSuite, OscilloscopeOperator,
    StereoscopeOperator, SubOperator, VstDescriptor,
};

#[cfg(test)]
mod tests {
    use crate::{
        input::process::InputProcessor,
        output::renderer::Renderer,
        params::{ChannelMode, PluginParams, ViewMode},
    };

    #[test]
    fn processes_ms_and_maps_sum_diff() {
        let mut p = PluginParams {
            channel_mode: ChannelMode::Ms,
            ..Default::default()
        };
        let mut input = InputProcessor::new(48_000.0, 50.0, 64);
        let frame = input.process_sample(1.0, -1.0, &p);
        assert!((frame.a - 0.0).abs() < 1e-6);
        assert!((frame.b - 1.0).abs() < 1e-6);

        let renderer = Renderer {
            noise_threshold: 0.0,
            attenuation: 0.25,
        };
        let pts = renderer.render(frame, ViewMode::SumDiff);
        assert_eq!(pts.len(), 1);
        assert!((pts[0].x - 1.0).abs() < 1e-6);
        assert!((pts[0].y + 1.0).abs() < 1e-6);

        p.view_mode = ViewMode::Polar;
        let polar = renderer.render(frame, p.view_mode);
        assert_eq!(polar.len(), 1);
        assert!(polar[0].x >= 0.0);
    }

    #[test]
    fn lookahead_disabled_has_zero_latency() {
        let p = PluginParams::default();
        let mut input = InputProcessor::new(48_000.0, 100.0, 64);
        let frame = input.process_sample(0.25, -0.5, &p);
        assert!((frame.a - 0.25).abs() < 1e-6);
        assert!((frame.b + 0.5).abs() < 1e-6);
    }
}
