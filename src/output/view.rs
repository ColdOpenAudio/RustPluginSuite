pub fn attenuate_for_noise(rms: f32, threshold: f32, intensity: f32, attenuation: f32) -> f32 {
    if rms < threshold {
        intensity * attenuation
    } else {
        intensity
    }
}
