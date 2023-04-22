pub mod matrix;

pub mod geometry {
    mod point;
    mod tuple;
    mod vector;

    pub use point::Point;
    pub use tuple::Tuple;
    pub use vector::Vector;
}

pub mod scene {
    mod canvas;
    mod colour;
    mod environment;
    mod projectile;

    pub use canvas::Canvas;
    pub use colour::Colour;
    pub use environment::Environment;
    pub use projectile::Projectile;
}

extern crate nalgebra;
