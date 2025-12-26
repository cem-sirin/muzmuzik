pub struct SineWave;

impl SineWave {
    pub fn sample(phase: f32) -> f32 {
        (phase * 2.0 * std::f32::consts::PI).sin()
    }
}
