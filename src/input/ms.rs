pub fn lr_to_ms(l: f32, r: f32) -> (f32, f32) {
    ((l + r) * 0.5, (l - r) * 0.5)
}
