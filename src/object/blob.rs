use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Point, Vector};

/// Object
pub struct Blob {
    pub points: Vec<Point>,
    pub d: f64,
    pub e: f64,

    pub id: &'static str,
}

impl Blob {
    pub fn triangles(&self) {
        let mut i = 0f64;
        while i < self.e - d {
            let mut j = 0f64;
            while j < self.e - d {
                let mut k = 0f64;
                while k < self.e - d {
                    let (p0, p1, p2, p3, p4, p5, p6, p7) = (
                        Point::new(i, j, k),
                        Point::new(i, j, k),
                        Point::new(i, j, k),
                        Point::new(i, j, k),
                        Point::new(i, j, k),
                        Point::new(i, j, k),
                        Point::new(i, j, k),
                        Point::new(i, j, k),
                    );
                    k += self.d;
                }
                j += self.d;
            }
            i += self.d;
        }
    }
}
