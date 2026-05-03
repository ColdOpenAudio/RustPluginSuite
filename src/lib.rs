pub mod context;
pub mod error;
pub mod framework;
pub mod operators;
pub mod plugin;

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

pub mod params;
pub mod shared {
    pub mod frame;
}

pub use context::PluginContext;
pub use error::{NihError, NihResult};
pub use framework::{Framework, FrameworkBuilder, FrameworkReport};
pub use plugin::{NihPlugin, PluginMetadata};

pub use operators::{
    BassGoBrrrOperator, FrequencyGateOperator, OperatorSuite, OscilloscopeOperator,
    StereoscopeOperator, SubOperator, VstDescriptor,
};

#[cfg(test)]
mod phase1_tests {
    use crate::{
        input::process::InputProcessor,
        output::renderer::render_points,
        params::{ChannelMode, PluginParams, ViewMode},
    };

    fn params() -> PluginParams {
        PluginParams {
            view_mode: ViewMode::XY,
            channel_mode: ChannelMode::LR,
            noise_threshold: 0.05,
            dc_remove: false,
            gain_scale: 1.0,
            lookahead_enabled: false,
            lookahead_ms: 0.0,
        }
    }

    #[test]
    fn supports_ms_and_polar_and_sl_modes() {
        let mut p = InputProcessor::new(48_000.0, 64, 4_800);
        let mut cfg = params();
        cfg.channel_mode = ChannelMode::MS;
        let frame = p.process_sample(1.0, -1.0, cfg);
        assert!((frame.a - 0.0).abs() < 1e-6);
        assert!((frame.b - 1.0).abs() < 1e-6);

        let polar = render_points(&frame, ViewMode::Polar, 0.0, 0.5);
        assert_eq!(polar.len(), 1);

        for mode in [
            ViewMode::XY,
            ViewMode::SumDiff,
            ViewMode::LissSum,
            ViewMode::DiffOnly,
            ViewMode::DualTrace,
        ] {
            let pts = render_points(&frame, mode, 0.0, 0.5);
            assert!(!pts.is_empty());
        }
    }

    #[test]
    fn dc_removal_and_lookahead_work() {
        let mut p = InputProcessor::new(48_000.0, 16, 480);
        let mut cfg = params();
        cfg.dc_remove = true;

        let f1 = p.process_sample(0.5, 0.5, cfg);
        assert!(f1.dc_a > 0.0);

        cfg.lookahead_enabled = true;
        cfg.lookahead_ms = 1.0;
        let delayed = p.process_sample(1.0, 1.0, cfg);
        assert!(delayed.a.abs() <= 1.0);
    }
}
