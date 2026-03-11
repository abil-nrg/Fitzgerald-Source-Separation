use num::Complex;

// Cooley-Tukey decimation-in-time FFT
// adapted from the implementation on Wikipedia: https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
pub fn fft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    if input.len() == 1 {
        vec![input[0]]
    } else {
        let even = input
            .iter()
            .step_by(2)
            .cloned()
            .collect::<Vec<Complex<f64>>>();
        let odd = input
            .iter()
            .skip(1)
            .step_by(2)
            .cloned()
            .collect::<Vec<Complex<f64>>>();

        let even = fft(&even);
        let odd = fft(&odd);

        let n = input.len();
        let mut output = vec![Complex::new(0.0, 0.0); n];

        for k in 0..n / 2 {
            let t = Complex::new(0.0, -2.0 * std::f64::consts::PI * k as f64 / n as f64).exp();
            output[k] = even[k] + t * odd[k];
            output[k + n / 2] = even[k] - t * odd[k];
        }

        output
    }
}

// implementation based on https://www.embedded.com/dsp-tricks-computing-inverse-ffts-using-the-forward-fft/
pub fn ifft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    let conjugated: Vec<Complex<f64>> = input.iter().map(|c| c.conj()).collect();
    let mut output = fft(&conjugated);
    for x in output.iter_mut() {
        *x = x.conj() / n as f64;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.00000001;
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
