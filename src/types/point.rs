use crate::types::{Tuple, Vector};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {}, y: {}, z: {}, w: {}",
            self.x,
            self.y,
            self.z,
            self.w()
        )
    }
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z, w: 1.0 }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        self.w
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Vector;

    #[test]
    fn test_adding_a_vector_to_a_point() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(0.75, 2.125, -1.55);

        let result = point + vector;

        assert_eq!(result, Point::new(1.75, 4.125, 1.45))
    }

    #[test]
    fn test_subtracting_a_point_from_another_point() {
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(0.75, 2.125, -1.55);

        let result = point1 - point2;

        assert_eq!(result, Vector::new(0.25, -0.125, 4.55))
    }

    #[test]
    fn test_subtracting_a_vector_from_a_point() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(0.75, 2.125, -1.55);

        let result = point - vector;

        assert_eq!(result, Point::new(0.25, -0.125, 4.55))
    }

    #[test]
    fn test_negating_point() {
        let point = Point::new(1.0, 2.0, 3.0);

        let result = -point;

        assert_eq!(
            result,
            Point {
                x: -1.0,
                y: -2.0,
                z: -3.0,
                w: -1.0
            }
        )
    }
}
