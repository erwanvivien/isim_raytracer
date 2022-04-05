use crate::object::sphere::Sphere;
use crate::Vector;

#[derive(Clone)]
pub struct State {
    pub position: Vector,
    pub head: Vector,
    pub up: Vector,
    pub left: Vector,
}

impl State {
    pub fn move_forward(&mut self, dist: f64) {
        self.position = self.position + self.head * dist;
    }

    pub fn rotate_head(&mut self, angle: f64) {
        let angle = angle.to_radians();
        let left = self.left * angle.cos() + self.up * angle.sin();
        let up = self.left * -angle.sin() + self.up * angle.cos();

        self.left = left.normalize();
        self.up = up.normalize();
    }

    pub fn rotate_left(&mut self, angle: f64) {
        let angle = angle.to_radians();
        let head = self.head * angle.cos() + self.up * angle.sin();
        let up = self.head * -angle.sin() + self.up * angle.cos();

        self.head = head.normalize();
        self.up = up.normalize();
    }

    pub fn rotate_up(&mut self, angle: f64) {
        let angle = angle.to_radians();
        let head = self.head * angle.cos() + self.left * -angle.sin();
        let left = self.head * angle.sin() + self.left * angle.cos();

        self.head = head.normalize();
        self.left = left.normalize();
    }
}
