use crate::geometry::{Vector};

pub struct ControlPoint {
    location: Vector,
    weight: f64
}

impl ControlPoint {
    pub fn new(location: Vector, weight: f64) -> ControlPoint {
        ControlPoint {
            location: ControlPoint::apply_weight(location, weight),
            weight: weight
        }
    }

    fn apply_weight(location: Vector, weight: f64) -> Vector {
        if weight == 0.0 {
            location
        }
        else {
            location * weight
        }
    }

    pub fn location(&self) -> Vector {
        if self.weight == 0.0 {
            self.location
        }
        else {
            self.location * (1.0 / self.weight)
        }
    }


}