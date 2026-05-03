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
        }
    }

    pub fn process_sample(&mut self, l: f32, r: f32, p: PluginParams) -> Frame {
        let delay_samples =
            ((self.sample_rate * p.lookahead_ms.max(0.0)) / 1000.0).round() as usize;
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
}
