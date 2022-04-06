use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Point, Vector};

/// Object
pub struct Plane {
    pub p: Point,
    pub normal: Vector,

    pub texture: Box<dyn TextureTrait>,
    pub id: String,
}

impl Intersect for Plane {
    fn is_intersect(&self, _p: Point, v: Vector) -> bool {
        const NULL_VEC: Vector = Vector::new(0f64, 0f64, 0f64);

        v != NULL_VEC && (v * self.normal) != 0f64
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        // from https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        // test http://www.ambrsoft.com/TrigoCalc/Plan3D/PlaneLineIntersection_.htm

        if !self.is_intersect(p, v) {
            return Vec::new();
        }

        let t = ((self.p - p) * self.normal) / (v * self.normal);
        if t <= 0f64 {
            return Vec::new();
        }
        return vec![p + v * t];
    }
}

impl Normal for Plane {
    fn normal(&self, _p: Point) -> Vector {
        self.normal
    }
}

impl GetTexture for Plane {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        &self.texture
    }
}

impl ObjectId for Plane {
    fn id(&self) -> &String {
        &self.id
    }
}

impl ObjectTrait for Plane {}

#[cfg(test)]
mod tests {
    use crate::object::plane::Plane;
    use crate::object::Intersect;
    use crate::{Color, Point, UniformTexture, Vector};

    const UNIFORM_TEXTURE: UniformTexture = UniformTexture {
        kd: 1f64,
        ka: 1f64,
        ks: 1f64,

        color: Color::BLACK,
    };

    fn plane() -> Plane {
        Plane {
            texture: Box::new(UNIFORM_TEXTURE),
            normal: Vector::new(1f64, 0f64, 0f64),
            p: Point::new(0f64, 0f64, 0f64),
            id: "plane",
        }
    }

    #[test]
    fn intersect() {
        let plane = Plane {
            texture: Box::new(UNIFORM_TEXTURE),
            normal: Vector::new(2f64, 3f64, 6f64),
            p: Point::new(1f64, 5f64, 3f64),
            id: "plane",
        };

        let (p, v) = (Point::new(0f64, 0f64, 0f64), Vector::new(1f64, 0f64, 0f64));

        let is_intersect = plane.is_intersect(p, v);
        assert_eq!(true, is_intersect);

        let intersect_points = plane.intersect_points(p, v);
        assert_ne!(intersect_points.len(), 0);

        assert_eq!(intersect_points[0], Vector::new(17.5f64, 0f64, 0f64))
    }

    #[test]
    fn a() {
        let plane = Plane {
            normal: Vector::new(0f64, 1f64, 0f64),
            p: Point::new(0f64, -2f64, 0f64),
            id: "plane1",
            texture: Box::new(UNIFORM_TEXTURE),
        };

        let (p, v) = (
            Point::new(0f64, 0f64, 0f64),
            Vector {
                x: 0.4919729194802923,
                y: 0.5141027441932218,
                z: 0.7026101443268782,
            },
        );

        let is_intersect = plane.intersect_points(p, v);
        dbg!(&is_intersect);

        assert_eq!(is_intersect.len(), 0)
    }

    #[test]
    fn no_intersect1() {
        let plane = plane();

        let (p, v) = (Point::new(0f64, 0f64, 0f64), Vector::new(0f64, 1f64, 0f64));

        let is_intersect = plane.is_intersect(p, v);
        assert_eq!(false, is_intersect);

        let intersect_points = plane.intersect_points(p, v);
        assert_eq!(intersect_points.len(), 0)
    }

    #[test]
    fn no_intersect2() {
        let plane = plane();

        let (p, v) = (Point::new(0f64, 0f64, 0f64), Vector::new(0f64, 0f64, 0f64));

        let is_intersect = plane.is_intersect(p, v);
        assert_eq!(false, is_intersect);

        let intersect_points = plane.intersect_points(p, v);
        assert_eq!(intersect_points.len(), 0)
    }
}
