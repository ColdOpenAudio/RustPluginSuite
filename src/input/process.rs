use crate::{
    input::{dc::DcTracker, lookahead::Lookahead, ms::lr_to_ms, rms::RmsWindow},
    params::{ChannelMode, PluginParams},
    shared::frame::Frame,
};

#[derive(Debug, Clone)]
pub struct InputProcessor {
    sample_rate: f32,
    lookahead: Lookahead,
    rms: RmsWindow,
    dc_a: DcTracker,
    dc_b: DcTracker,
}

impl InputProcessor {
    pub fn new(sample_rate: f32, max_lookahead_ms: f32, rms_window_samples: usize) -> Self {
        let max_delay_samples = ((sample_rate * max_lookahead_ms) / 1_000.0).round() as usize;
        Self {
            sample_rate,
            lookahead: Lookahead::new(max_delay_samples),
            rms: RmsWindow::new(rms_window_samples),
            dc_a: DcTracker::new(0.001),
            dc_b: DcTracker::new(0.001),
        }
    }

    pub fn process_sample(&mut self, left: f32, right: f32, p: &PluginParams) -> Frame {
        let delay_samples =
            ((self.sample_rate * p.lookahead_ms.max(0.0)) / 1_000.0).round() as usize;
        let (l, r) = self
            .lookahead
            .process(left, right, p.lookahead_enabled, delay_samples);
        let (mut a, mut b) = match p.channel_mode {
            ChannelMode::Lr => (l, r),
            ChannelMode::Ms => lr_to_ms(l, r),
        };

        let dc_a = self.dc_a.update(a);
        let dc_b = self.dc_b.update(b);

        if p.dc_remove {
            a = self.dc_a.remove(a);
            b = self.dc_b.remove(b);
        }

        a *= p.gain_scale;
        b *= p.gain_scale;

        let rms = self.rms.push((a * a + b * b).sqrt() * 0.70710677);
        Frame {
            a,
            b,
            rms,
            dc_a,
            dc_b,
        }
    }
}
