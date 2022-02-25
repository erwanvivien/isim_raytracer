//! Camera representation
use crate::point::Point;
use crate::vector::Vector;

/// Struct representing the camera
pub struct Camera {
    center: Point,
    spotted_point: Point,
    up: Vector,
    right: Vector,
    forward: Vector,
    alpha: f64,
    beta: f64,
}

impl Camera {
    pub fn new(center: Point, spotted_point: Point, up: Vector, alpha: f64, beta: f64) -> Camera {
        let up = up.normalize();
        let forward = (spotted_point - center).to_vec().normalize();
        let right = Vector::normal_vec(&up, &forward);

        Camera {
            center,
            spotted_point,
            alpha,
            beta,
            up,
            forward,
            right,
        }
    }
}
