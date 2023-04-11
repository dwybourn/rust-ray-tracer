use crate::geometry::Vector;

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Environment { gravity, wind }
    }
}
