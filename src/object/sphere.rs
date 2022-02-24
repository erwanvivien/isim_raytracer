use crate::object::{Intersect, Normal};
use crate::point::Point;
use crate::texture::uniform::UniformTexture;
use crate::texture::Lighting;
use crate::vector::Vector;

pub struct Sphere {
    p: Point,
    r: f64,
    texture: UniformTexture,
}

impl Sphere {
    fn equation(&self, a: f64, b: f64, c: f64) -> bool {
        (b * b - 4f64 * a * c) >= 0f64
    }
}

impl Intersect for Sphere {
    fn is_intersect(&self, p: Point, v: Vector) -> bool {
        // a = (x2 - x1)2 + (y2 - y1)2 + (z2 - z1)2
        // b = - 2[(x2 - x1)(xc - x1) + (y2 - y1)(yc - y1) + (z2 - z1)(zc - z1)]
        // c = (xc - x1)2 + (yc - y1)2 + (zc - z1)2 - r2
        let (x1, x2, xc) = (p.x, v.x, self.p.x);
        let (y1, y2, yc) = (p.y, v.y, self.p.y);
        let (z1, z2, zc) = (p.z, v.z, self.p.z);

        let a = (x2 - x1).powf(2f64) + (y2 - y1).powf(2f64) + (z2 - z1).powf(2f64);
        let b = (x2 - x1) * (xc - x1) + (y2 - y1) * (yc - y1) + (z2 - z1) * (zc - z1);
        let b = 2f64 * b;
        let c =
            (xc - x1).powf(2f64) + (yc - y1).powf(2f64) + (zc - z1).powf(2f64) - self.r.powf(2f64);

        self.equation(a, b, c)
    }
}

impl Normal for Sphere {
    fn normal(&self, p: Point) -> Vector {
        let tmp = p - self.p;
        Vector {
            x: tmp.x,
            y: tmp.y,
            z: tmp.z,
        }
        .normalize()
    }
}

impl Lighting for Sphere {
    fn coefficients(&self, point: Point) -> (f64, f64, f64) {
        self.texture.coefficients(point)
    }
}
