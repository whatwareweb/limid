use std::f32::consts::PI;

pub fn sin_sample(sample_clock: f32, sample_rate: f32, freq: f32) -> f32 {
    (2.0 * (sample_clock * freq * PI / sample_rate)).sin()
}

pub fn sqr_sample(sample_clock: f32, sample_rate: f32, freq: f32) -> f32 {
    (2.0 * (sample_clock * freq * PI / sample_rate)).sin().signum()
}

pub fn tri_sample(sample_clock: f32, sample_rate: f32, freq: f32) -> f32 {
    (2.0 * ((2.0 * (((sample_clock * freq * PI / sample_rate) - PI / 4.0) % PI) - PI) / PI)).abs() - 1.0
}

pub fn saw_sample(sample_clock: f32, sample_rate: f32, freq: f32) -> f32 {
    (2f32 * ((sample_clock * freq * PI / sample_rate) % PI) - PI) / PI
}

pub fn fm(sample_clock: f32, sample_rate: f32, freq: f32) -> f32 {
    sin_sample(sample_clock, sample_rate, freq + 6.0 * sin_sample(sample_clock + 20.0, sample_rate, freq))
}