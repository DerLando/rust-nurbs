

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

#[cfg(test)]
mod tests {
    use crate::algorithms::find_span;

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
}