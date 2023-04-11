use crate::geometry::{Point, Tuple};
use std::fmt::{Display, Formatter};
use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalise(&self) -> Vector {
        *self / self.magnitude()
    }
}

impl Display for Vector {
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

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
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
        0.0
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl BitXor for Vector {
    type Output = f64;

    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_adding_two_vectors_should_equal_a_new_vector() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(0.75, 2.125, -1.55);

        let result = vector1 + vector2;

        assert_eq!(result, Vector::new(1.75, 4.125, 1.45))
    }

    #[test]
    fn test_adding_a_point_to_a_vector() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(0.75, 2.125, -1.55);

        let result = vector + point;

        assert_eq!(result, Point::new(1.75, 4.125, 1.45))
    }

    #[test]
    fn test_subtracting_two_vectors() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(0.75, 2.125, -1.55);

        let result = vector1 - vector2;

        assert_eq!(result, Vector::new(0.25, -0.125, 4.55));
    }

    #[test]
    fn test_negating_vector() {
        let vector = Vector::new(1.0, 2.0, 3.0);

        let result = -vector;

        assert_eq!(result, Vector::new(-1.0, -2.0, -3.0))
    }

    #[test]
    fn test_multiplying_a_vector_by_a_scalar() {
        let vector = Vector::new(1.0, 2.0, 3.0);

        let result = vector * 3.0;

        assert_eq!(result, Vector::new(3.0, 6.0, 9.0))
    }

    #[test]
    fn test_dividing_a_vector_by_a_scalar() {
        let vector = Vector::new(1.0, 2.0, 3.0);

        let result = vector / 4.0;

        assert_eq!(result, Vector::new(0.25, 0.5, 0.75))
    }

    #[test]
    fn test_magnitude_of_a_vector() {
        let vector = Vector::new(3.0, 4.0, 5.0);

        let result = vector.magnitude();

        assert_eq!(result, (50 as f64).sqrt())
    }

    #[test]
    fn test_normalize_vector() {
        let vector = Vector::new(1.0, 2.0, 3.0);

        let magnitude = vector.magnitude();
        let result = vector.normalise();
        let magnitude_after_norm = result.magnitude();

        assert_eq!(
            result,
            Vector::new(1.0 / magnitude, 2.0 / magnitude, 3.0 / magnitude)
        );

        assert_eq!(magnitude_after_norm, 1.0);
    }

    #[test]
    fn test_vector_dot_product() {
        let vector1 = Vector::new(1.5, 2.0, 3.0);
        let vector2 = Vector::new(1.5, 2.0, 3.0);

        let result = vector1 ^ vector2;

        assert_eq!(result, 15.25)
    }

    #[test]
    fn test_vector_cross_product() {
        let vector1 = Vector::new(1.5, 2.0, 3.0);
        let vector2 = Vector::new(1.0, 2.5, -1.5);

        let result = vector1 * vector2;

        assert_eq!(result, Vector::new(-10.5, 5.25, 1.75))
    }
}
