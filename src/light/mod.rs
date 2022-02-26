//! Lighting container
use crate::{Point, Vector};

pub mod point;

pub trait LightTrait {
    fn intensity(&self) -> Vector;
    fn point(&self) -> Point;
}

/// Abstract struct for Lighting
pub struct Light {}

impl LightTrait for Light {
    fn intensity(&self) -> Vector {
        unimplemented!()
    }

    fn point(&self) -> Point {
        unimplemented!()
    }
}
