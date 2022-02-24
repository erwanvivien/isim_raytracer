mod sphere;

use crate::point::Point;
use crate::vector::Vector;

trait Intersect {
    fn is_intersect(&self, p: Point, v: Vector) -> bool;
}
trait Normal {
    fn normal(&self) -> Vector;
}

pub struct Object {}

impl Intersect for Object {
    fn is_intersect(&self, p: Point, v: Vector) -> bool {
        unimplemented!()
    }
}

impl Normal for Object {
    fn normal(&self) -> Vector {
        unimplemented!()
    }
}
