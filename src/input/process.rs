use crate::{
    input::{
        dc::DcEstimator,
        lookahead::{LookaheadBuffer, LookaheadParams},
        ms::lr_to_ms,
        rms::RmsWindow,
    },
    params::{ChannelMode, PluginParams},
    shared::frame::Frame,
};

pub struct InputProcessor {
    dc_l: DcEstimator,
    dc_r: DcEstimator,
    rms: RmsWindow,
    lookahead: LookaheadBuffer,
}

impl InputProcessor {
    pub fn new(sample_rate: f32, rms_window: usize, max_lookahead_samples: usize) -> Self {
        Self {
            dc_l: DcEstimator::new(0.001),
            dc_r: DcEstimator::new(0.001),
            rms: RmsWindow::new(rms_window),
            lookahead: LookaheadBuffer::new(max_lookahead_samples, sample_rate),
        }
    }

    pub fn process_sample(&mut self, left: f32, right: f32, params: PluginParams) -> Frame {
        let (l, r) = self.lookahead.process(
            left,
            right,
            LookaheadParams {
                enabled: params.lookahead_enabled,
                time_ms: params.lookahead_ms,
            },
        );
        let dc_l = self.dc_l.update(l);
        let dc_r = self.dc_r.update(r);
        let (l, r) = if params.dc_remove {
            (l - dc_l, r - dc_r)
        } else {
            (l, r)
        };

        let rms = self.rms.push((l + r) * 0.5);
        let (a, b) = match params.channel_mode {
            ChannelMode::LR => (l, r),
            ChannelMode::MS => lr_to_ms(l, r),
        };

        Frame {
            a: a * params.gain_scale,
            b: b * params.gain_scale,
            rms,
            dc_a: dc_l,
            dc_b: dc_r,
        }
    }
}
