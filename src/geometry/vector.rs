use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector{x: x, y: y, z: z}
    }

    pub const fn zero() -> Vector {
        Vector{x: 0.0, y: 0.0, z: 0.0}
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Add<f64> for Vector {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Vector::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Add<Vector> for f64 {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}