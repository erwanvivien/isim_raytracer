use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Point, Vector};

/// Object
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,

    pub edge1: Vector,
    pub edge2: Vector,

    pub normal: Vector,

    pub texture: Box<dyn TextureTrait>,
    pub id: &'static str,
}

impl Triangle {
    pub fn new(
        p1: Point,
        p2: Point,
        p3: Point,
        texture: Box<dyn TextureTrait>,
        id: &'static str,
    ) -> Triangle {
        let edge1 = p2 - p1;
        let edge2 = p3 - p1;

        Triangle {
            p1,
            p2,
            p3,
            edge1,
            edge2,
            normal: edge1.cross_product(&edge2).normalize(),
            texture,
            id,
        }
    }

    pub fn intersection(&self, p: Point, v: Vector) -> (bool, f64) {
        let h = v.cross_product(&self.edge2);
        let angle = self.edge1 * h;

        if angle.abs() < 0.000001f64 {
            return (false, 0f64);
        }

        let f = 1f64 / angle;
        let s = p - self.p1;
        let u = (s * h) * f;

        if u < 0f64 || u > 1f64 {
            return (false, 01f64);
        }

        let q = s.cross_product(&self.edge1);
        let v = f * (v * q);

        if v < 0f64 || u + v > 1f64 {
            return (false, 02f64);
        }

        let t = f * (self.edge2 * q);
        if t >= 0.00001f64 {
            return (true, t);
        }

        (false, 0f64)
    }
}

impl Intersect for Triangle {
    fn is_intersect(&self, p: Point, v: Vector) -> bool {
        self.intersection(p, v).0
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        let (intersect, t) = self.intersection(p, v);
        if !intersect {
            return Vec::new();
        }

        return vec![p + v * t];
    }
}

impl Normal for Triangle {
    fn normal(&self, _p: Point) -> Vector {
        self.normal
    }
}

impl GetTexture for Triangle {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        &self.texture
    }
}

impl ObjectId for Triangle {
    fn id(&self) -> &'static str {
        self.id
    }
}

impl ObjectTrait for Triangle {}

#[cfg(test)]
mod tests {
    use crate::object::Intersect;
    use crate::{Color, Point, Triangle, UniformTexture, Vector};

    const UNIFORM_TEXTURE: UniformTexture = UniformTexture {
        kd: 1f64,
        ka: 1f64,
        ks: 1f64,

        color: Color::BLACK,
    };

    fn triangle() -> Triangle {
        Triangle::new(
            Point::new(0f64, 0f64, -5f64),
            Point::new(0f64, 0f64, 5f64),
            Point::new(0f64, 4f64, 0f64),
            Box::new(UniformTexture {
                kd: 1f64,
                ka: 1f64,
                ks: 0f64,
                color: Color::WHITE,
            }),
            "Triangle",
        )
    }

    #[test]
    fn intersect_1() {
        let triangle = triangle();
        let start = Point::new(-3f64, 0f64, 0f64);
        let ray = Vector::new(1f64, 0f64, 0f64);

        let res = triangle.intersection(start, ray);
        assert_eq!(res.0, true);
        assert_eq!(res.1, 3f64)
    }

    #[test]
    fn intersect_point_1() {
        let triangle = triangle();
        let start = Point::new(-3f64, 0f64, 0f64);
        let ray = Vector::new(1f64, 0f64, 0f64);

        let res = triangle.intersect_points(start, ray);
        assert_eq!(res.len(), 1);
        assert_eq!(res.get(1).unwrap(), 3f64)
    }
}
