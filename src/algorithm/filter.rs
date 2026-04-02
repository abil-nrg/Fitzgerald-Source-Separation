pub fn median_filter(input: &[f64], window_size: usize) -> Vec<f64> {
    let mut output = Vec::with_capacity(input.len());
    let half_window = window_size / 2;

    for i in 0..input.len() {
        let start = i.saturating_sub(half_window);
        let end = usize::min(i + half_window + 1, input.len());
        let mut window: Vec<f64> = input[start..end].to_vec();

        window.resize(window_size, 0.0);
        window.sort_by(|a, b| a.partial_cmp(b).unwrap());
        output.push(window[window.len() / 2]);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_median_filter() {
        // example from https://en.wikipedia.org/wiki/Median_filter
        let input = vec![2.0, 3.0, 80.0, 6.0, 2.0, 3.0];
        let expected = vec![2.0, 3.0, 6.0, 6.0, 3.0, 2.0];
        assert_eq!(median_filter(&input, 3), expected);
    }
}
