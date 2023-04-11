use ray_tracer::geometry::{Point, Tuple, Vector};
use ray_tracer::scene::{Canvas, Colour, Environment, Projectile};

fn tick(environment: &Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;
    return Projectile { position, velocity };
}

fn main() {
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
        canvas[y][x] = Colour::new(1.0, 0.8, 0.6);
        projectile = tick(&environment, projectile);
        println!("{}", projectile.position.to_string())
    }
    canvas.write_to_file("image.ppm").expect("Oh no");
}
