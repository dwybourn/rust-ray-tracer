use nalgebra::{
    DMatrix, Matrix2x4, Point3, Rotation, Rotation2, Rotation3, Transform3, Translation2,
    Translation3, Vector3,
};
use ray_tracer::geometry::{Point, Tuple, Vector};
use ray_tracer::matrix::shearing;
use ray_tracer::scene::{Canvas, Colour, Environment, Projectile};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

fn tick(environment: &Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;
    return Projectile { position, velocity };
}

fn projectile() {
    Matrix2x4::<f64>::zeros();
    let b = DMatrix::<f64>::zeros(5, 3);
    let c = b.determinant();
    println!("{}", c);
    let mut projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.8, 0.0).normalise() * 11.25,
    );
    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));
    let height = 550;
    let width = 900;
    let mut canvas = Canvas::new(width, height);
    while projectile.position.y() >= 0.0 {
        let x = projectile.position.x().round() as usize;
        let y = height - projectile.position.y().round() as usize;
        if y > height || x > width {
            continue;
        };
        canvas[(y, x)] = Colour::new(1.0, 0.8, 0.6);
        projectile = tick(&environment, projectile);
        println!("{}", projectile.position.to_string())
    }
    canvas.write_to_file("image.ppm").expect("Oh no");
}

fn clock() {
    let mut canvas = Canvas::new(500, 500);
    let translation = Translation3::new(250.0, 250.0, 0.0);
    let centre: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    let zero: Point3<f64> = Translation3::new(0.0, 1.0, 0.0).transform_point(&centre);
    let mut points: Vec<Point3<f64>> = vec![];
    let radius = 200.0;

    for i in 0..12 {
        let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), TAU * (i as f64 / 12.0));
        let rotated = rotation.transform_point(&zero);
        let scaled = Point3::new(rotated.x * radius, rotated.y * radius, rotated.z);
        let translated = translation.transform_point(&scaled);
        points.push(translated);
    }

    for point in points.iter() {
        println!("{}", point);
        canvas.pixels[(point.y as usize, point.x as usize)] = Colour::new(255.0, 255.0, 255.0);
    }

    canvas
        .write_to_file("image.ppm")
        .expect("TODO: panic message");
}

fn main() {}
