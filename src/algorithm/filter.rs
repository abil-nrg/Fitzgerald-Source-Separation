pub fn median_filter(input: &[f64], window_size: usize) -> Vec<f64> {
    let mut output = vec![0.0; input.len()];
    let half_window = window_size / 2;

    for i in 0..input.len() {
        let start = i.saturating_sub(half_window);
        let end = (i + half_window + 1).min(input.len());
        let mut window: Vec<f64> = input[start..end].to_vec();
        window.sort_by(|a, b| a.partial_cmp(b).unwrap());
        output[i] = window[window.len() / 2];
    }

    output
}
