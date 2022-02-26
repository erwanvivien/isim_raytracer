//! Object representation
pub mod sphere;

use crate::point::Point;
use crate::texture::TextureTrait;
use crate::vector::Vector;

/// Defines functions to asserts collision of ray to object
pub trait Intersect {
    /// Returns true if `intersect_points` contains points
    fn is_intersect(&self, p: Point, v: Vector) -> bool;
    /// Returns collision points
    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point>;
}

pub trait Normal {
    /// Returns the normal at `p` point
    fn normal(&self, p: Point) -> Vector;
}

pub trait GetTexture {
    fn texture(&self) -> &Box<dyn TextureTrait>;
}

/// SuperTrait for objects
pub trait ObjectTrait: Intersect + Normal + GetTexture {}

/// Object
pub struct Object {}

impl Intersect for Object {
    fn is_intersect(&self, _p: Point, _v: Vector) -> bool {
        unimplemented!()
    }
    fn intersect_points(&self, _p: Point, _v: Vector) -> Vec<Point> {
        unimplemented!()
    }
}

impl Normal for Object {
    fn normal(&self, _p: Point) -> Vector {
        unimplemented!()
    }
}

impl GetTexture for Object {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        unimplemented!()
    }
}

impl ObjectTrait for Object {}
