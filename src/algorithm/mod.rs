use core::f64;
use std::iter;

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
    pub fn fun(&self) -> fn(usize) -> Vec<f64> {
        match self {
            Window::Hann => hann_window,
            Window::Rectangular => rectangular_window,
            Window::Triangular => triangular_window,
            Window::Blackman => blackman_window,
            Window::Hamming => hamming_window,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Window::Hann => "Hann Window",
            Window::Rectangular => "Rectangular Window",
            Window::Triangular => "Triangular Window",
            Window::Blackman => "Blackman Window",
            Window::Hamming => "Hamming Window",
        }
    }
}

// implementations based on https://en.wikipedia.org/wiki/Window_function#Examples_of_window_functions

pub fn hann_window(size: usize) -> Vec<f64> {
    (0..size)
        .map(|n| 0.5 - 0.5 * (2.0 * f64::consts::PI * n as f64 / size as f64).cos())
        .collect()
}

pub fn hamming_window(size: usize) -> Vec<f64> {
    (0..size)
        .map(|n| 0.54 - 0.46 * (2.0 * f64::consts::PI * n as f64 / size as f64).cos())
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
