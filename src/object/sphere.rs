use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::point::Point;
use crate::texture::{LightCoefficients, TextureTrait};
use crate::vector::Vector;

pub struct Sphere {
    pub p: Point,
    pub r: f64,
    pub texture: Box<dyn TextureTrait>,
    pub id: &'static str,
}

impl Sphere {
    fn intersect_coeff(&self, p: Point, v: Vector) -> (f64, f64, f64) {
        // taken from
        // http://ambrsoft.com/TrigoCalc/Sphere/SpherLineIntersection_.htm
        let cp = self.p - p;

        let a = v.powf(2f64).sum();
        let b = -2f64 * v.mul(cp).sum();
        let c = cp.powf(2f64).sum() - self.r.powf(2f64);

        (a, b, c)
    }
}

impl Intersect for Sphere {
    fn is_intersect(&self, p: Point, v: Vector) -> bool {
        let (a, b, c) = self.intersect_coeff(p, v);
        (b * b) - 4f64 * a * c >= 0f64
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        let (a, b, c) = self.intersect_coeff(p, v);

        let delta = b.powf(2f64) - 4f64 * a * c;

        if delta < 0f64 {
            return Vec::new();
        }

        if delta <= f64::EPSILON {
            let t = -b / (2f64 * a);
            let p = p + (v * t);
            return vec![p];
        }

        let t_sqrt = delta.sqrt();

        let t1 = (-b + t_sqrt) / (2f64 * a);
        let t2 = (-b - t_sqrt) / (2f64 * a);

        let p1 = p + (v * t1);
        let p2 = p + (v * t2);

        return vec![p1, p2];
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

impl LightCoefficients for Sphere {
    fn coefficients(&self, point: Point) -> (f64, f64, f64) {
        self.texture.coefficients(point)
    }
}

impl GetTexture for Sphere {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        &self.texture
    }
}

impl ObjectId for Sphere {
    fn id(&self) -> &'static str {
        self.id
    }
}

impl ObjectTrait for Sphere {}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::object::sphere::Sphere;
    use crate::object::Intersect;
    use crate::point::Point;
    use crate::texture::uniform::UniformTexture;
    use crate::vector::Vector;

    const UNIFORM_TEXTURE: UniformTexture = UniformTexture {
        kd: 1f64,
        ka: 1f64,
        ks: 1f64,

        color: Color::BLACK,
    };

    fn sphere() -> Sphere {
        Sphere {
            p: Point::new(4f64, 5f64, 6f64),
            r: 1f64,
            texture: Box::new(UNIFORM_TEXTURE),
            id: "First",
        }
    }

    #[test]
    fn intersect0() {
        let point = Point::new(0f64, 1f64, 2f64);
        let ray = Vector::new(1f64, 1f64, 1f64);
        let is_intersect = sphere().is_intersect(point, ray);

        assert_eq!(true, is_intersect)
    }

    #[test]
    fn intersect1() {
        let point = Point::new(0f64, 1f64, 1f64);
        let ray = Vector::new(1f64, 1f64, 1f64);
        let is_intersect = sphere().is_intersect(point, ray);

        assert_eq!(true, is_intersect)
    }

    #[test]
    fn intersect2() {
        let point = Point::new(0f64, 1f64, 0f64);
        let ray = Vector::new(1f64, 1f64, 1.5f64);
        let is_intersect = sphere().is_intersect(point, ray);

        assert_eq!(true, is_intersect)
    }

    #[test]
    fn intersect_points0() {
        let point = Point::new(4f64, 1f64, 7.1f64);
        let ray = Vector::new(0f64, 1f64, 0f64);
        let intersect_points = sphere().intersect_points(point, ray);

        assert_eq!(intersect_points.len(), 0);
    }

    #[test]
    fn intersect_points1() {
        let point = Point::new(4f64, 1f64, 5f64);
        let ray = Vector::new(0f64, 1f64, 0f64);
        let intersect_points = sphere().intersect_points(point, ray);

        assert_eq!(intersect_points.len(), 1);
        let first = intersect_points[0];

        let first_point = &*format!("({:.2} , {:.2} , {:.2} )", first.x, first.y, first.z);
        assert_eq!(first_point, "(4.00 , 5.00 , 5.00 )");
    }

    #[test]
    fn intersect_points2() {
        let point = Point::new(0f64, 1f64, 2f64);
        let ray = Vector::new(1f64, 1f64, 1f64);
        let intersect_points = sphere().intersect_points(point, ray);

        assert_eq!(intersect_points.len(), 2);
        let first = intersect_points[0];
        let second = intersect_points[1];

        let (first, second) = if first.x > second.x {
            (second, first)
        } else {
            (first, second)
        };

        let first_point = format!("({:.2} , {:.2} , {:.2} )", first.x, first.y, first.z);
        let second_point = format!("({:.2} , {:.2} , {:.2} )", second.x, second.y, second.z);

        assert_eq!(&*first_point, "(3.42 , 4.42 , 5.42 )");
        assert_eq!(&*second_point, "(4.58 , 5.58 , 6.58 )");
    }

    #[test]
    fn no_intersect0() {
        let point = Point::new(0f64, 1f64, -0.5f64);
        let ray = Vector::new(1f64, 1f64, 1f64);
        let is_intersect = sphere().is_intersect(point, ray);

        assert_eq!(false, is_intersect)
    }

    #[test]
    fn no_intersect1() {
        let point = Point::new(0f64, 1f64, -0.3f64);
        let ray = Vector::new(1f64, 1f64, 1f64);
        let is_intersect = sphere().is_intersect(point, ray);

        assert_eq!(false, is_intersect)
    }

    #[test]
    fn no_intersect2() {
        let point = Point::new(0f64, 1f64, -1f64);
        let ray = Vector::new(1f64, 1f64, 1f64);
        let is_intersect = sphere().is_intersect(point, ray);

        assert_eq!(false, is_intersect)
    }

    #[test]
    fn no_intersect3() {
        let sphere = Sphere {
            p: Point::new(5f64, 0f64, 5f64),
            r: 1f64,
            texture: Box::new(UNIFORM_TEXTURE),
            id: "First",
        };

        let point = Point::new(0f64, 0f64, 0f64);
        let ray = Vector::new(1f64, 1f64, 1f64);
        let is_intersect = sphere.is_intersect(point, ray);

        assert_eq!(false, is_intersect)
    }
}
