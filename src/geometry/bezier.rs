use crate::{
    geometry::Vector,
    algorithms::{all_bernstein, compute_bernstein},
    };

pub struct Bezier {
    pub control_points: Vec<Vector>,
}

impl Bezier {
    pub fn new(control_points: Vec<Vector>) -> Bezier {
        Bezier {
            control_points: control_points
        }
    }

    /// Gets the degree of a bezier curve.
    /// This is defined as the length of the contro points vector.
    pub fn degree(&self) -> usize {
        self.control_points.len()
    }

    pub fn point_at(&self, u: f64) -> Vector {
        let degree = self.degree();
        let bernstein = all_bernstein(degree, u);
        let mut c = Vector::zero();

        // WARNING: Book describes loop from 0 to degree + 1
        for k in 0..degree {
            // TODO: one-line by implementing operators for f64 and Vector
            c.x = c.x + bernstein[k] * self.control_points[k].x;
            c.y = c.y + bernstein[k] * self.control_points[k].y;
            c.z = c.z + bernstein[k] * self.control_points[k].z;
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Vector, Bezier};

    #[test]
    fn point_at_should_work() {
        let v0 = Vector::zero();
        let v1 = Vector::new(2.0, 2.0, 0.0);
        let bezier = Bezier::new(vec![v0, v1]);

        assert_eq!(bezier.point_at(0.5), Vector::new(1.0, 1.0, 0.0));
    }
}