#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    XY,
    Polar,
    SumDiff,
    LissSum,
    DiffOnly,
    DualTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelMode {
    LR,
    MS,
}

#[derive(Debug, Clone, Copy)]
pub struct PluginParams {
    pub view_mode: ViewMode,
    pub channel_mode: ChannelMode,
    pub noise_threshold: f32,
    pub dc_remove: bool,
    pub gain_scale: f32,
    pub lookahead_enabled: bool,
    pub lookahead_ms: f32,
}
