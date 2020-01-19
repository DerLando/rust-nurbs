use crate::{
    algorithms::{horner},
    geometry::Vector,
    utils::Matrix,
};

pub fn horner_surface(points: Box<dyn Matrix<Vector>>, n: usize, m: usize, u: f64, v: f64) -> Vector {
    let mut b: Vec<Vector> = Vec::new();
    for i in 0..n + 1 {
        b.push(horner(&points.row(i), m, v));
    }
    horner(&b, n, u)
}

#[cfg(test)]
mod tests {
    use crate::algorithms::horner_surface;
    use crate::geometry::Vector;

    // #[test]
    // fn horner_surface_should_work() {
    //     let v0 = Vector::zero();
    //     let v1 = Vector::new(1.0, 0.0, 0.0);
    //     let v2 = Vector::new(2.0, 0.0, 0.0);
    //     let v3 = Vector::new(0.0, 1.0, 0.0);
    //     let v4 = Vector::new(0.0, 2.0, 0.0);

    //     let nested = vec![vec![v0, v1, v2], vec![v0, v3, v4]];

    //     assert_eq!(horner_surface(&nested, 1, 1, 0.0, 0.0), v0);
    //     assert_eq!(horner_surface(&nested, 1, 1, 1.0, 1.0), Vector::new(2.0, 2.0, 0.0));
    // }
}