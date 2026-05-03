#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    Xy,
    Polar,
    SumDiff,
    LissSum,
    DiffOnly,
    DualTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChannelMode {
    #[default]
    Lr,
    Ms,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PluginParams {
    pub view_mode: ViewMode,
    pub channel_mode: ChannelMode,
    pub noise_threshold: f32,
    pub dc_remove: bool,
    pub gain_scale: f32,
    pub lookahead_enabled: bool,
    pub lookahead_ms: f32,
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            view_mode: ViewMode::Xy,
            channel_mode: ChannelMode::Lr,
            noise_threshold: 0.02,
            dc_remove: false,
            gain_scale: 1.0,
            lookahead_enabled: false,
            lookahead_ms: 0.0,
        }
    }
}
