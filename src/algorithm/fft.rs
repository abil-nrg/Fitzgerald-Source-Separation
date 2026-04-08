use num::Complex;
use rustfft::FftPlanner;
use std::sync::Arc;

/// Computes the forward FFT using the rustfft library.
pub fn fft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    let mut buffer = input.to_vec();
    
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut buffer);
    
    buffer
}

/// Computes the inverse FFT using the rustfft library.
pub fn ifft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    let mut buffer = input.to_vec();
    
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(n);
    fft.process(&mut buffer);

    // rustfft does not normalize the inverse transform automatically.
    // We must divide by N to complete the roundtrip.
    let n_f64 = n as f64;
    for x in &mut buffer {
        *x /= n_f64;
    }
    
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.000_000_01;
    #[test]
    fn test_fft() {
        let input = [
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];
        let output = fft(&input);
        let expected = [
            Complex::new(10.0, 0.0),
            Complex::new(-2.0, 2.0),
            Complex::new(-2.0, 0.0),
            Complex::new(-2.0, -2.0),
        ];

        for (o, e) in output.iter().zip(expected.iter()) {
            assert!((o - e).norm() < EPSILON);
        }
    }

    #[test]
    fn test_ifft() {
        let input = [
            Complex::new(10.0, 0.0),
            Complex::new(-2.0, 2.0),
            Complex::new(-2.0, 0.0),
            Complex::new(-2.0, -2.0),
        ];
        let output = ifft(&input);
        let expected = [
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];
        for (o, e) in output.iter().zip(expected.iter()) {
            assert!((o - e).norm() < EPSILON);
        }
    }

    #[test]
    #[should_panic(expected = "fft input length must be a power of 2")]
    fn test_fft_non_power_of_two() {
        let input = [
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ];
        fft(&input);
    }

    #[test]
    #[should_panic(expected = "ifft input length must be a power of 2")]
    fn test_ifft_non_power_of_two() {
        let input = [
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ];
        ifft(&input);
    }

    #[test]
    fn test_fft_ifft() {
        let input = [
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ];
        let output = ifft(&fft(&input));
        for (o, e) in output.iter().zip(input.iter()) {
            println!("o: {}, e: {}, diff: {}\n", o, e, (o - e).norm());
            assert!((o - e).norm() < EPSILON);
        }
    }
}
