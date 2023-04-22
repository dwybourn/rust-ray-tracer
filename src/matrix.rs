use nalgebra::Matrix4;

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4<f64> {
    return Matrix4::new(
        1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
}
