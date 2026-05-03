#[inline]
pub fn lr_to_ms(left: f32, right: f32) -> (f32, f32) {
    ((left + right) * 0.5, (left - right) * 0.5)
}
