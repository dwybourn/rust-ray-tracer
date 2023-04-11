use crate::geometry::{Point, Vector};

pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Projectile { position, velocity }
    }
}
