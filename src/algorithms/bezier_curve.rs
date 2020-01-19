use crate::geometry::Vector;

/// Computes point on a power-basis curve
/// NOTE: degree of power-basis curves is bezier-curve-degree - 1 !
pub fn horner(points: &Vec<Vector>, n: usize, u: f64) -> Vector {
    let mut c = points[n];
    let mut i = (n - 1) as i64;
    while i >= 0 {
        c = c * u + points[i as usize];

        i -= 1;
    }

    c
}

/// Computes point on a bezier curve using
/// repeated linear onterpolation of the vector of control points
/// at the parameter u.
pub fn de_casteljeau(points: &Vec<Vector>, n: usize, u: f64) -> Vector {
    let mut pt_copy = points.to_vec();
    for k in 1..n + 1 {
        for i in 0..n - k {
            pt_copy[i] = (1.0 - u) * pt_copy[i] + u * pt_copy[i + 1];
        }
    }

    pt_copy[0]
}

#[cfg(test)]
mod tests {
    use crate::algorithms::{horner, de_casteljeau};
    use crate::geometry::{Vector, Bezier};

    #[test]
    fn horner_should_worik() {
        let v0 = Vector::zero();
        let v1 = Vector::new(3.0, 3.0, 0.0);
        let curve = Bezier::new(vec![v0, v1]);

        let mid_point = horner(&curve.control_points, curve.degree() - 1, 0.5);

        assert_eq!(mid_point, Vector::new(1.5, 1.5, 0.0));
    }

    #[test]
    fn de_casteljeau_should_work() {
        let v0 = Vector::zero();
        let v1 = Vector::new(3.0, 3.0, 0.0);
        let curve = Bezier::new(vec![v0, v1]);

        let mid_point = de_casteljeau(&curve.control_points, curve.degree(), 0.5);

        assert_eq!(mid_point, Vector::new(1.5, 1.5, 0.0));
    }
}