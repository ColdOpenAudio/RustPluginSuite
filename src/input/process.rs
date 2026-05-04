use crate::{
    input::{dc::DcTracker, lookahead::LookaheadBuffer, ms::lr_to_ms, rms::RmsWindow},
    params::{ChannelMode, PluginParams},
    shared::frame::Frame,
};

#[derive(Debug)]
pub struct InputProcessor {
    left_dc: DcTracker,
    right_dc: DcTracker,
    rms: RmsWindow,
    lookahead: LookaheadBuffer,
    sample_rate: f32,
    cached_lookahead_ms: f32,
    cached_lookahead_enabled: bool,
    cached_delay_samples: usize,
}

impl InputProcessor {
    pub fn new(sample_rate: f32, rms_window: usize, max_lookahead_ms: f32) -> Self {
        let max_delay_samples = ((sample_rate * max_lookahead_ms) / 1000.0).round() as usize;
        Self {
            left_dc: DcTracker::new(0.001),
            right_dc: DcTracker::new(0.001),
            rms: RmsWindow::new(rms_window),
            lookahead: LookaheadBuffer::new(max_delay_samples),
            sample_rate,
            cached_lookahead_ms: f32::NAN,
            cached_lookahead_enabled: false,
            cached_delay_samples: 0,
        }
    }

    pub fn process_sample(&mut self, l: f32, r: f32, p: PluginParams) -> Frame {
        let delay_samples = self.lookahead_delay_samples(p.lookahead_ms, p.lookahead_enabled);
        let (l, r) = self
            .lookahead
            .process(l, r, p.lookahead_enabled, delay_samples);
        let dc_l = self.left_dc.update(l);
        let dc_r = self.right_dc.update(r);
        let (l, r) = if p.dc_remove {
            (self.left_dc.remove(l), self.right_dc.remove(r))
        } else {
            (l, r)
        };
        let rms = self.rms.push((l + r) * 0.5);
        let (a, b) = match p.channel_mode {
            ChannelMode::Lr => (l, r),
            ChannelMode::Ms => lr_to_ms(l, r),
        };
        Frame {
            a: a * p.gain_scale,
            b: b * p.gain_scale,
            rms,
            dc_a: dc_l,
            dc_b: dc_r,
        }
    }

    fn lookahead_delay_samples(&mut self, lookahead_ms: f32, lookahead_enabled: bool) -> usize {
        if !lookahead_enabled {
            self.cached_lookahead_enabled = false;
            self.cached_lookahead_ms = lookahead_ms;
            self.cached_delay_samples = 0;
            return 0;
        }

        if self.cached_lookahead_enabled && self.cached_lookahead_ms == lookahead_ms {
            return self.cached_delay_samples;
        }

        let delay_samples = ((self.sample_rate * lookahead_ms.max(0.0)) / 1000.0).round() as usize;
        self.cached_lookahead_enabled = true;
        self.cached_lookahead_ms = lookahead_ms;
        self.cached_delay_samples = delay_samples;
        delay_samples
    }
}

#[cfg(test)]
mod tests {
    use super::InputProcessor;
    use crate::params::PluginParams;

    #[test]
    fn lookahead_delay_is_zero_when_disabled() {
        let mut p = InputProcessor::new(48_000.0, 64, 50.0);
        assert_eq!(p.lookahead_delay_samples(25.0, false), 0);
    }

    #[test]
    fn lookahead_delay_matches_sample_rate_conversion() {
        let mut p = InputProcessor::new(48_000.0, 64, 50.0);
        assert_eq!(p.lookahead_delay_samples(10.0, true), 480);
    }

    #[test]
    fn processing_handles_repeated_params_without_changing_output_contract() {
        let mut proc = InputProcessor::new(48_000.0, 8, 50.0);
        let params = PluginParams {
            lookahead_enabled: true,
            lookahead_ms: 1.0,
            ..Default::default()
        };
        let a = proc.process_sample(0.5, -0.5, params);
        let b = proc.process_sample(0.25, -0.25, params);
        assert!(a.rms.is_finite());
        assert!(b.rms.is_finite());
    }
}
