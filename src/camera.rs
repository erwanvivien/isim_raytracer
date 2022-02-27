//! Camera representation
use crate::point::Point;
use crate::vector::Vector;

/// Struct representing the camera
pub struct Camera {
    pub center: Point,
    pub spotted_point: Point,
    pub up: Vector,
    pub right: Vector,
    pub forward: Vector,
    pub alpha: f64,
}

impl Camera {
    pub fn new(center: Point, spotted_point: Point, up: Vector, alpha: f64) -> Camera {
        let up = up.normalize();
        let forward = (spotted_point - center).normalize();
        let right = up.cross_product(&forward);

        let perpendicular = up * forward;
        dbg!(&center);

        if up * forward != 0f64 {
            panic!(
                "Up & forward not perpendicular: up * forward = {}\n{:?}\n{:?}",
                perpendicular, &up, &forward
            )
        }

        Camera {
            center,
            spotted_point,
            alpha,
            up,
            forward,
            right,
        }
    }
}
