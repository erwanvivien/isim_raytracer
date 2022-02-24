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
    fn equation(&self, a: f64, b: f64, c: f64) -> f64 {
        (b * b) - 4f64 * a * c
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

        dbg!((x1, x2, xc));
        dbg!((y1, y2, yc));
        dbg!((z1, z2, zc));

        let a = (x2 - x1).powf(2f64) + (y2 - y1).powf(2f64) + (z2 - z1).powf(2f64);
        let b = v * (p - self.p).to_vec();
        let b = b * 2f64;

        let c =
            (xc - x1).powf(2f64) + (yc - y1).powf(2f64) + (zc - z1).powf(2f64) - self.r.powf(2f64);

        dbg!((a, b, c));
        dbg!(self.equation(a, b, c));

        self.equation(a, b, c) >= 0f64
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

mod tests {
    use crate::object::sphere::Sphere;
    use crate::object::Intersect;
    use crate::point::Point;
    use crate::texture::uniform::UniformTexture;
    use crate::vector::Vector;

    #[test]
    fn intersert() {
        let sphere = Sphere {
            p: Point::new(4f64, 5f64, 6f64),
            r: 1f64,
            texture: UniformTexture {
                kd: 1f64,
                ka: 1f64,
                ks: 1f64,
            },
        };

        let point = Point::new(0f64, 1f64, 2f64);
        let ray = Vector::new(1f64, 1f64, 1f64);
        let is_intersect = sphere.is_intersect(point, ray);

        dbg!(&is_intersect);
        assert!(is_intersect)
    }
}
