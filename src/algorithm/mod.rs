use core::f64;

pub mod fft;
pub mod filter;
pub mod stft;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Window {
    Hann,
    Rectangular,
    Triangular,
    Blackman,
    Hamming,
}

impl Window {
    pub const fn fun(self) -> fn(usize) -> Vec<f64> {
        match self {
            Self::Hann => hann_window,
            Self::Rectangular => rectangular_window,
            Self::Triangular => triangular_window,
            Self::Blackman => blackman_window,
            Self::Hamming => hamming_window,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Hann => "Hann Window",
            Self::Rectangular => "Rectangular Window",
            Self::Triangular => "Triangular Window",
            Self::Blackman => "Blackman Window",
            Self::Hamming => "Hamming Window",
        }
    }
}

// implementations based on https://en.wikipedia.org/wiki/Window_function#Examples_of_window_functions

pub fn hann_window(size: usize) -> Vec<f64> {
    (0..size)
        .map(|n| 0.5_f64.mul_add(-(2.0 * f64::consts::PI * n as f64 / size as f64).cos(), 0.5))
        .collect()
}

pub fn hamming_window(size: usize) -> Vec<f64> {
    (0..size)
        .map(|n| {
            0.46_f64.mul_add(
                -(2.0 * f64::consts::PI * n as f64 / size as f64).cos(),
                0.54,
            )
        })
        .collect()
}

pub fn rectangular_window(size: usize) -> Vec<f64> {
    vec![1.0; size]
}

pub fn triangular_window(size: usize) -> Vec<f64> {
    let f_size = size as f64;
    (0..size)
        .map(|n| 1.0 - ((n as f64 - f_size / 2.0) / (f_size / 2.0)).abs())
        .collect()
}

pub fn blackman_window(size: usize) -> Vec<f64> {
    let alpha = 0.16;
    let a0 = (1.0 - alpha) / 2.0;
    let a1 = 0.5;
    let a2 = alpha / 2.0;

    (0..size)
        .map(|n| {
            a0 - a1 * (2.0 * f64::consts::PI * n as f64 / size as f64).cos()
                + a2 * (4.0 * f64::consts::PI * n as f64 / size as f64).cos()
        })
        .collect()
}
