use std::iter::repeat;

/// computes the value of a bernstein polynomial of form
/// B_i,n(u) = n! / i!(n-i)! * u^i(1-u)^n-1
pub fn compute_bernstein(i: usize, n: usize, u: f64) -> f64 {
    // compute value table
    // initialize to all 0s last item is 1.0
    let mut table: Vec<f64> = repeat(0.0).take(n + 1 as usize).collect();
    
    table[n - i] = 1.0;
    let u1 = 1.0 - u;

    let mut k = 1;
    while k <= n {
        let mut j = n;
        while j >= k {
            table[j] = u1 * table[j] + u * table[j - 1];

            j -= 1;
        }
        
        k += 1;
    }

    println!("table for i:{:?} and n:{:?} is: {:#?}", i, n, table);
    table[n]
}

/// Computes the n+1 nth-degree Bernstein polynomials
/// which are nonzero at a fixed u parameter
pub fn all_bernstein(n: usize, u: f64) -> Vec<f64> {
    let mut table: Vec<f64> = repeat(1.0).take(n + 1).collect();
    let u1 = 1.0 - u;

    for j in 1..n {
        let mut saved = 0.0;

        for k in 0..j {
            let temp = table[k];
            table[k] = saved + u1 * temp;
            saved = u * temp;
        }

        table[j] = saved;
    }

    table
}

#[cfg(test)]
mod tests {

    use super::{compute_bernstein, all_bernstein};

    #[test]
    fn compute_bernstein_should_work() {
        let u = 0.2;
        
        assert_eq!(compute_bernstein(0, 1, u), 1.0 - u);
        assert_eq!(compute_bernstein(1, 1, u), u);
        assert_eq!(compute_bernstein(0, 2, u), (1.0 - u).powi(2));
        assert_eq!(compute_bernstein(1, 2, u), 2.0 * u * (1.0 - u));
        assert_eq!(compute_bernstein(2, 2, u), u.powi(2));
    }

    #[test]
    fn all_bernstein_should_work() {
        let u = 0.4;

        assert_eq!(all_bernstein(2, u), vec![1.0 - u, u, 1.0]);
    }
}