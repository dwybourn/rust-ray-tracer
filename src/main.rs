use ray_tracer::types::{Point, Tuple};

fn test<T: Tuple>(test: T) {
    println!("{:?}", test.to_string())
}

fn main() {
    let tuple = Point::new(1.0, 2.0, 3.0);
    test(tuple)
}
