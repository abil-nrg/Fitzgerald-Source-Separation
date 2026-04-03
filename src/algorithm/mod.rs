use std::iter;

pub mod fft;
pub mod filter;
pub mod stft;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum WindowFunction {
    HannWindow,
    RectangularWindow,
}

impl WindowFunction {
    pub fn fun(&self) -> fn(usize) -> Vec<f64> {
        match self {
            WindowFunction::HannWindow => hann_window,
            WindowFunction::RectangularWindow => rectangular_window,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            WindowFunction::HannWindow => "Hann Window",
            WindowFunction::RectangularWindow => "Rectangular Window",
        }
    }
}

pub fn hann_window(size: usize) -> Vec<f64> {
    (0..size)
        .map(|n| 0.5 * (1.0 - (2.0 * std::f64::consts::PI * n as f64 / size as f64).cos()))
        .collect()
}

pub fn rectangular_window(size: usize) -> Vec<f64> {
    vec![1.0; size]
}
