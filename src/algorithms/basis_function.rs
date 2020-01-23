use std::iter::repeat;

/// Finds the knot span (the index of the basis function)
pub fn find_span(n: usize, p: usize, u: f64, knots: Vec<f64>) -> usize {
    if u == knots[n + 1] {return n;}; // Special case
    let mut low = p;
    let mut high = n + 1;
    let mut mid = (low + high) / 2;

    while (u < knots[mid]) || (u >= knots[mid + 1]) {
        if u < knots[mid] {high = mid}
        else {low = mid}
        mid = (low + high) / 2;
    }

    mid
}

pub fn calculate_basis_functions(span: usize, u: f64, p: usize, knots: Vec<f64>) -> Vec<f64> {
    let mut basis_functions: Vec<f64> = repeat(0.0).take(p + 1).collect();
    basis_functions[0] = 1.0;

    let mut left: Vec<f64> = repeat(0.0).take(p + 1).collect();
    let mut right: Vec<f64> = repeat(0.0).take(p + 1).collect();
    let mut saved: f64;

    for j in 1..p + 1 {
        left[j] = u - knots[span + 1 - j];
        right[j] = knots[span + j] - u;
        saved = 0.0;
        for r in 0..j {
            let temp = basis_functions[r] / (right[r + 1] + left[j - r]);
            basis_functions[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        basis_functions[j] = saved
    }

    basis_functions
}

#[cfg(test)]
mod tests {
    use crate::algorithms::{find_span, calculate_basis_functions};

    #[test]
    fn find_span_should_work() {
        let n = 9;
        let p = 2;
        let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0];
        let u = 5.0 / 2.0;

        let expected = 4;

        let actual = find_span(n, p, u, knots);

        assert_eq!(actual, expected);
    }

    #[test]
    fn calculate_basis_functions_should_work() {
        let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0];
        let u = 5.0 / 2.0;
        let span = 4;

        let actual = calculate_basis_functions(span, u, 2, knots.to_vec());
        let expected = vec![0.125, 0.75, 0.125];

        assert_eq!(actual, expected);

        let actual = calculate_basis_functions(span, u, 1, knots.to_vec());
        let expected = vec![0.5, 0.5];

        assert_eq!(actual, expected);

        let actual = calculate_basis_functions(span, u, 0, knots.to_vec());
        let expected = vec![1.0];

        assert_eq!(actual, expected);
    }
}