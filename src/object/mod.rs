pub mod sphere;

use crate::point::Point;
use crate::vector::Vector;

pub trait Intersect {
    fn is_intersect(&self, p: Point, v: Vector) -> bool;
}
trait Normal {
    fn normal(&self, p: Point) -> Vector;
}

pub struct Object {}

impl Intersect for Object {
    fn is_intersect(&self, _p: Point, _v: Vector) -> bool {
        unimplemented!()
    }
}

impl Normal for Object {
    fn normal(&self, _p: Point) -> Vector {
        unimplemented!()
    }
}
