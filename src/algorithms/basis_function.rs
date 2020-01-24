use std::iter::repeat;
use ndarray::{Array, Array2};

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

/// Calculates the values of basis functions for a fixed u
pub fn calculate_basis_functions(span: usize, u: f64, p: usize, knots: Vec<f64>) -> Vec<f64> {
    let mut basis_functions: Vec<f64> = repeat(0.0).take(p + 1).collect();
    basis_functions[0] = 1.0;

    // left of |j| is u - u_span+1-j
    let mut left: Vec<f64> = repeat(0.0).take(p + 1).collect();
    // right of |j| is u_span+j - u
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

/// Calculates nonzero basis functions and their derivatives
pub fn calculate_basis_functions_and_derivatives(span: usize, u: f64, p: usize, n: usize, knots: Vec<f64>) -> Array2<f64> {
    let mut ders = Array2::zeros((n + 1, p + 1));
    let mut ndu = Array2::zeros((p + 1, p + 1));
    let mut a = Array2::zeros((2, p + 1));

    let mut left: Vec<f64> = repeat(0.0).take(p + 1).collect();
    let mut right: Vec<f64> = repeat(0.0).take(p + 1).collect();
    let mut saved: f64;

    ndu.row_mut(0)[0] = 1.0;
    for j in 1..p + 1 {
        left[j] = u - knots[span + 1 - j];
        right[j] = knots[span + j] - u;
        saved = 0.0;
        for r in 0..j { // lower triangle
            ndu.row_mut(j)[r] = right[r + 1] + left[j - r];
            let temp = ndu.row(r)[j - 1] / ndu.row(j)[r];

            // upper triangle
            ndu.row_mut(r)[j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu.row_mut(j)[j] = saved;
    }
    println!("ndu is: {:#?}", ndu);
    for j in 0..p + 1 { // load the basis functions
        ders.row_mut(0)[j] = ndu.row(j)[p];
    }

    // compute derivatives
    for r in 0..p + 1 {
        let mut s1 = 0;
        let mut s2 = 1;
        a.row_mut(0)[0] = 1.0;

        // loop to compute k-th derivative
        for k in 1..n + 1 {
            let mut d = 0.0;
            let rk = r as i32 - k as i32;
            let pk = p as i32 - k as i32;
            let j1: i32;
            let j2: i32;

            if r >= k {
                a.row_mut(s2)[0] = a.row(s1)[0] / ndu.row(1 + pk as usize)[rk as usize];
                d = a.row(s2)[0] * ndu.row(rk as usize)[pk as usize];
            }

            if rk >= -1 {j1 = 1;}
            else {j1 = -rk;}

            if r as i32 - 1 <= pk {j2 = k as i32 - 1;}
            else {j2 = (p - r) as i32;}

            for j in j1..j2 + 1 {
                a.row_mut(s2)[j as usize] = (a.row(s1)[j as usize] - a.row(s1)[(j - 1) as usize]) / ndu.row((pk + 1) as usize)[(rk + j) as usize];
                d += a.row(s2)[j as usize] * ndu.row((rk + j) as usize)[pk as usize];
            }

            if r <= pk as usize {
                a.row_mut(s2)[k] = -a.row(s1)[k - 1] / ndu.row((pk + 1) as usize)[r];
                d += a.row(s2)[k] * ndu.row(r)[pk as usize];
            }

            println!("d for k: {:?} and r: {:?} is {:?}", k, r, d);
            ders.row_mut(k)[r] = d;
            let j = s1;
            s1 = s2;
            s2 = j;
        }
    }

    // Multiply through
    println!("ders before multiplication is: {:#?}", ders);
    let mut r = p;
    for k in 1..n + 1 {
        for j in 0..p + 1 {
            ders.row_mut(k)[j] *= r as f64;
        }
        r *= p - k;
    }
    
    ders
}

/// Calculates only one basis function
pub fn calculate_one_basis_function(p: usize, m: usize, knots: Vec<f64>, span: usize, u: f64) -> f64 {
    // trivial cases
    if ((span == 0) && (u == knots[0])) || ((span == m - p - 1) && (u == knots[m])) {return 1.0;}

    if (u < knots[span]) || (u >= knots[span + p + 1]) {return 0.0;}

    // initialize table to either ones or zeros
    let mut table: Vec<f64> = (0..p + 1).map(|j| {
        if (u >= knots[span + j]) && (u < knots[span + j + 1]) {1.0}
        else {0.0}
    }).collect();

    for k in 1..p + 1 {
        let mut saved: f64;
        if table[0] == 0.0 {saved = 0.0;}
        else {saved = ((u - knots[span]) * table[0]) / (knots[span + k] - knots[span]);}

        for j in 0..p - k + 1 {
            let knot_left = knots[span + j + 1];
            let knot_right = knots[span + j + k + 1];
            if table[j + 1] == 0.0 {
                table[j] = saved;
                saved = 0.0;
            }
            else {
                let temp = table[j + 1] / (knot_right - knot_left);
                table[j] = saved + (knot_right - u) * temp;
                saved = (u - knot_left) * temp;
            }
        }
    }
    
    table[0]
}

#[cfg(test)]
mod tests {
    use crate::algorithms::{find_span, calculate_basis_functions, calculate_basis_functions_and_derivatives,
                            calculate_one_basis_function,};
    use ndarray::arr2;

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

    #[test]
    fn calculate_basis_functions_and_derivatives_should_work() {
        let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0];
        let u = 5.0 / 2.0;
        let span = 4;

        let actual = calculate_basis_functions_and_derivatives(span, u, 2, 2, knots);
        // output is expected to be of form
        // [[N2,2(u), N3,2(u), N4,2(u)],
        //  [N2,2^1(u), n3,2^1(u), N4,2^1(u)],
        //  [N2,2²(u), n3,2²(u), N4,2²(u)]]
        let expected = arr2(&[[0.125, 0.75, 0.125],
                                [-0.5, 0.0, 0.5],
                                [0.0, 0.0, 1.0]]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn calculate_one_basis_function_should_work() {
        let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0];
        let u = 5.0 / 2.0;

        let actual = calculate_one_basis_function(2, knots.len() - 1, knots.to_vec(), 4, u);
        let expected = 1.0 / 8.0;

        assert_eq!(actual, expected);

        let actual = calculate_one_basis_function(2, knots.len() - 1, knots.to_vec(), 3, u);
        let expected = 6.0 / 8.0;

        assert_eq!(actual, expected);
    }
}