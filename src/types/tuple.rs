use std::fmt::Display;
use std::ops::Neg;

pub trait Tuple: Display + Neg {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}
