use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Point, Vector};

/// Object
pub struct Rectangle {
    pub p_min: Point,
    pub p_max: Point,

    pub center: Point,
    pub size: Vector,

    pub texture: Box<dyn TextureTrait>,
    pub id: &'static str,
}

impl Rectangle {
    pub fn new(
        p1: Point,
        p2: Point,
        texture: Box<dyn TextureTrait>,
        id: &'static str,
    ) -> Rectangle {
        let mut xs = [p1.x, p2.x];
        let mut ys = [p1.y, p2.y];
        let mut zs = [p1.z, p2.z];
        xs.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());
        ys.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());
        zs.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());

        let p1 = Point::new(xs[0], ys[0], zs[0]);
        let p2 = Point::new(xs[1], ys[1], zs[1]);

        Rectangle {
            p_min: p1,
            p_max: p2,

            texture,
            id,
            center: (p1 + p2) / 2f64,
            size: (p2 - p1).map(|x| x.abs()) * 0.5,
        }
    }
}

impl Intersect for Rectangle {
    fn is_intersect(&self, _p: Point, v: Vector) -> bool {
        const NULL_VEC: Vector = Vector::new(0f64, 0f64, 0f64);
        v != NULL_VEC // && ...
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        // // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection

        let invdir = 1f64 / v;
        let sign = [
            (invdir.x < 0f64) as usize,
            (invdir.y < 0f64) as usize,
            (invdir.z < 0f64) as usize,
        ];

        let aabb = [self.p_min, self.p_max];

        let mut ray_min = (aabb[sign[0]].x - p.x) * invdir.x;
        let mut ray_max = (aabb[1 - sign[0]].x - p.x) * invdir.x;

        let y_min = (aabb[sign[1]].y - p.y) * invdir.y;
        let y_max = (aabb[1 - sign[1]].y - p.y) * invdir.y;

        if (ray_min > y_max) || (y_min > ray_max) {
            return Vec::new();
        }

        if y_min > ray_min {
            ray_min = y_min;
        }

        if y_max < ray_max {
            ray_max = y_max;
        }

        let z_min = (aabb[sign[2]].z - p.z) * invdir.z;
        let z_max = (aabb[1 - sign[2]].z - p.z) * invdir.z;

        if (ray_min > z_max) || (z_min > ray_max) {
            return Vec::new();
        }

        if z_max < ray_max {
            ray_max = z_max;
        }

        let min = p + v * ray_min;
        let max = p + v * ray_max;

        vec![min, max]
    }
}

impl Normal for Rectangle {
    fn normal(&self, p: Point) -> Vector {
        const K_EPSILON: f64 = 0.0001;

        // let pc = p - self.center;
        //
        // // step(edge,x) : x < edge ? 0 : 1
        // let signum = pc.map(|x| x.signum());
        //
        // let pc_abs = pc.map(|x| x.abs());
        // let pc_removed_size = pc_abs - self.size;
        //
        // let tmp = pc_removed_size.map(|x| (x.abs() <= K_EPSILON) as u8 as f64);
        //
        // let res = signum.mul(tmp);
        // dbg!(&res);
        // res

        let pc = p - self.center;

        let signum = pc.map(|x| x.signum());
        let rel = pc.map(|x| x.abs()).div(self.size);
        let res = (rel - 1f64).map(|x| (x.abs() < 0.0000001f64) as usize as f64);

        let res = res.mul(signum).normalize();
        if res.x.abs() != 1f64 {
            dbg!(&res);
        }
        res
    }
}

impl GetTexture for Rectangle {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        &self.texture
    }
}

impl ObjectId for Rectangle {
    fn id(&self) -> &'static str {
        self.id
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
            Point::new(1f64, 1f64, 1f64),
            Point::new(10f64, 10f64, 10f64),
            Box::new(UNIFORM_TEXTURE),
            "rect",
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
