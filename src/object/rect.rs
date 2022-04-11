use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Point, Vector};

#[derive(Debug)]
pub struct RectangleInner {
    pub p_min: Point,
    pub p_max: Point,

    pub center: Point,
    pub size: Vector,
}

impl RectangleInner {
    pub fn new(p1: Point, p2: Point) -> RectangleInner {
        RectangleInner {
            p_min: p1,
            p_max: p2,
            center: (p1 + p2) / 2f64,
            size: (p2 - p1).map(|x| x.abs()) * 0.5,
        }
    }
}

/// Object
pub struct Rectangle {
    pub rect: RectangleInner,

    pub texture: Box<dyn TextureTrait>,
    pub id: String,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point, texture: Box<dyn TextureTrait>, id: String) -> Rectangle {
        let mut xs = [p1.x, p2.x];
        let mut ys = [p1.y, p2.y];
        let mut zs = [p1.z, p2.z];
        xs.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());
        ys.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());
        zs.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());

        let p1 = Point::new(xs[0], ys[0], zs[0]);
        let p2 = Point::new(xs[1], ys[1], zs[1]);

        Rectangle {
            rect: RectangleInner::new(p1, p2),
            texture,
            id,
        }
    }
}

impl Intersect for RectangleInner {
    fn is_intersect(&self, p: Point, v: Vector) -> bool {
        const NULL_VEC: Vector = Vector::new(0f64, 0f64, 0f64);
        v != NULL_VEC && !self.intersect_points(p, v).is_empty()
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection

        let invdir = 1f64 / v;
        let sign = [
            (invdir.x < 0f64) as usize,
            (invdir.y < 0f64) as usize,
            (invdir.z < 0f64) as usize,
        ];

        let aabb = [self.p_min, self.p_max];
        let mut tmin = (aabb[sign[0]].x - p.x) * invdir.x;
        let mut tmax = (aabb[1 - sign[0]].x - p.x) * invdir.x;
        let tymin = (aabb[sign[1]].y - p.y) * invdir.y;
        let tymax = (aabb[1 - sign[1]].y - p.y) * invdir.y;

        if (tmin > tymax) || (tymin > tmax) {
            return Vec::new();
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (aabb[sign[2]].z - p.z) * invdir.z;
        let tzmax = (aabb[1 - sign[2]].z - p.z) * invdir.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return Vec::new();
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        if tmin > 0f64 && tmax > 0f64 {
            vec![p + v * tmin, p + v * tmax]
        } else if tmax > 0f64 {
            vec![p + v * tmax]
        } else {
            vec![]
        }
    }
}

impl Normal for RectangleInner {
    fn normal(&self, p: Point) -> Vector {
        const K_EPSILON: f64 = 0.0000001f64;

        let pc = p - self.center;

        let signum = pc.map(|x| x.signum());
        let rel = pc.map(|x| x.abs()).div(self.size);
        let res = (rel - 1f64).map(|x| (x.abs() < K_EPSILON) as usize as f64);
        res.mul(signum).normalize()
    }
}

impl Intersect for Rectangle {
    fn is_intersect(&self, p: Point, v: Vector) -> bool {
        self.rect.is_intersect(p, v)
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        self.rect.intersect_points(p, v)
    }
}

impl Normal for Rectangle {
    fn normal(&self, p: Point) -> Vector {
        self.rect.normal(p)
    }
}

impl GetTexture for Rectangle {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        &self.texture
    }
}

impl ObjectId for Rectangle {
    fn id(&self) -> &String {
        &self.id
    }
}

impl ObjectTrait for Rectangle {}

#[cfg(test)]
mod tests {
    use crate::{object::Intersect, point::Point, vector::Vector, Color, UniformTexture};

    use super::Rectangle;

    const UNIFORM_TEXTURE: UniformTexture = UniformTexture {
        kd: 1f64,
        ka: 1f64,
        ks: 1f64,

        color: Color::BLACK,
    };

    fn rect() -> Rectangle {
        return Rectangle::new(
            Point::new(0f64, 0f64, 0f64),
            Point::new(10f64, 10f64, 10f64),
            Box::new(UNIFORM_TEXTURE),
            String::from("rect"),
        );
    }

    #[test]
    fn intersect() {
        let rect = rect();

        let points =
            rect.intersect_points(Point::new(-5f64, 5f64, 5f64), Vector::new(1f64, 0f64, 0f64));

        dbg!(&points);
        assert_eq!(points.len(), 2)
    }
}
