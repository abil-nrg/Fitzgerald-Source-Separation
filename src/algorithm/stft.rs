use num::Complex;

use super::fft::{fft, ifft};

pub fn stft(
    signal: &[f32],
    window_size: usize,
    hop_size: usize,
    window_fn: fn(usize) -> Vec<f64>,
) -> Vec<Vec<Complex<f64>>> {
    let window = window_fn(window_size);
    let fft_size = window_size.next_power_of_two();
    let mut frames = Vec::new();

    let mut start = 0;
    while start + window_size <= signal.len() {
        let mut frame = vec![Complex::new(0.0, 0.0); fft_size];
        for i in 0..window_size {
            frame[i] = Complex::new(f64::from(signal[start + i]) * window[i], 0.0);
        }

        frames.push(fft(&frame));
        start += hop_size;
    }

    frames
}

pub fn istft(
    frames: &[Vec<Complex<f64>>],
    window_size: usize,
    hop_size: usize,
    output_length: usize,
    window_fn: fn(usize) -> Vec<f64>,
) -> Vec<f32> {
    let window = window_fn(window_size);
    let total_len = (frames.len() - 1) * hop_size + window_size;
    let mut output = vec![0.0f64; total_len];
    let mut window_sum = vec![0.0f64; total_len];

    for (idx, frame) in frames.iter().enumerate() {
        let time_domain = ifft(frame);
        let start = idx * hop_size;

        for i in 0..window_size {
            output[start + i] += time_domain[i].re * window[i];
            window_sum[start + i] += window[i] * window[i];
        }
    }

    for i in 0..output.len() {
        if window_sum[i] > 1e-8 {
            output[i] /= window_sum[i];
        }
    }

    output
        .iter()
        .take(output_length)
        .map(|&s| s as f32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-4;

    #[test]
    fn test_stft_frame_count() {
        let signal: Vec<f32> = (0..1024).map(|i| (i as f32 * 0.1).sin()).collect();
        let window_size = 256;
        let hop_size = 128;
        let frames = stft(&signal, window_size, hop_size, super::super::hann_window);
        let expected = (signal.len() - window_size) / hop_size + 1;
        assert_eq!(frames.len(), expected);
    }

    #[test]
    fn test_stft_frame_length_is_power_of_two() {
        let signal: Vec<f32> = vec![0.0; 512];
        let frames = stft(&signal, 300, 150, super::super::hann_window);
        assert_eq!(frames[0].len(), 512);
    }

    #[test]
    fn test_roundtrip() {
        let n = 2048;
        let signal: Vec<f32> = (0..n)
            .map(|i| {
                let t = i as f32 / 44100.0;
                (2.0 * std::f32::consts::PI * 440.0 * t).sin()
            })
            .collect();

        let window_size = 512;
        let hop_size = 128;

        let frames = stft(&signal, window_size, hop_size, super::super::hann_window);
        let reconstructed = istft(
            &frames,
            window_size,
            hop_size,
            signal.len(),
            super::super::hann_window,
        );

        assert_eq!(reconstructed.len(), signal.len());

        let margin = window_size;
        for i in margin..signal.len() - margin {
            assert!((reconstructed[i] - signal[i]).abs() < EPSILON);
        }
    }

    #[test]
    fn test_silence_roundtrip() {
        let signal = vec![0.0f32; 1024];
        let frames = stft(&signal, 256, 128, super::super::hann_window);
        let reconstructed = istft(&frames, 256, 128, signal.len(), super::super::hann_window);
        for s in &reconstructed {
            assert!(s.abs() < EPSILON);
        }
    }
}
