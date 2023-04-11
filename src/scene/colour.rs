use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Colour {
    red: f64,
    green: f64,
    blue: f64,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Colour { red, green, blue }
    }

    pub fn black() -> Self {
        Colour::new(0.0, 0.0, 0.0)
    }

    pub fn red(&self) -> f64 {
        self.red
    }

    pub fn green(&self) -> f64 {
        self.green
    }

    pub fn blue(&self) -> f64 {
        self.blue
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Colour {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Colour {
    type Output = Colour;

    fn sub(self, rhs: Self) -> Self::Output {
        Colour {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Colour::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl Mul for Colour {
    type Output = Colour;

    fn mul(self, rhs: Self) -> Self::Output {
        Colour::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_two_colours() {
        let colour1 = Colour::new(1.0, 2.0, 3.0);
        let colour2 = Colour::new(0.75, 2.125, -1.55);

        let result = colour1 + colour2;

        assert_eq!(result, Colour::new(1.75, 4.125, 1.45))
    }

    #[test]
    fn test_subtracting_colours() {
        let colour1 = Colour::new(1.0, 2.0, 3.0);
        let colour2 = Colour::new(0.75, 2.125, -1.55);

        let result = colour1 - colour2;

        assert_eq!(result, Colour::new(0.25, -0.125, 4.55))
    }

    #[test]
    fn test_multiplying_a_colour_by_a_scalar() {
        let colour = Colour::new(1.0, 2.0, 3.0);

        let result = colour * 3.0;

        assert_eq!(result, Colour::new(3.0, 6.0, 9.0))
    }

    #[test]
    fn test_multiplying_colours() {
        let colour1 = Colour::new(1.0, 2.0, 3.0);
        let colour2 = Colour::new(0.75, 2.125, -1.55);

        let result = colour1 * colour2;

        assert_eq!(result, Colour::new(0.75, 4.25, -4.65))
    }
}
