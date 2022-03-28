/*
bool intersect(const Ray &r) const
{
    float tmin, tmax, tymin, tymax, tzmin, tzmax;

    tmin = (bounds[r.sign[0]].x - r.orig.x) * r.invdir.x;
    tmax = (bounds[1-r.sign[0]].x - r.orig.x) * r.invdir.x;
    tymin = (bounds[r.sign[1]].y - r.orig.y) * r.invdir.y;
    tymax = (bounds[1-r.sign[1]].y - r.orig.y) * r.invdir.y;

    if ((tmin > tymax) || (tymin > tmax))
        return false;
    if (tymin > tmin)
        tmin = tymin;
    if (tymax < tmax)
        tmax = tymax;

    tzmin = (bounds[r.sign[2]].z - r.orig.z) * r.invdir.z;
    tzmax = (bounds[1-r.sign[2]].z - r.orig.z) * r.invdir.z;

    if ((tmin > tzmax) || (tzmin > tmax))
        return false;
    if (tzmin > tmin)
        tmin = tzmin;
    if (tzmax < tmax)
        tmax = tzmax;

    return true;
}
*/

use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Point, Vector};

/// Object
pub struct Rect {
    pub p: Point,
    pub normal: Vector,

    pub texture: Box<dyn TextureTrait>,
    pub id: &'static str,
}

impl Intersect for Rect {
    fn is_intersect(&self, _p: Point, v: Vector) -> bool {
        const NULL_VEC: Vector = Vector::new(0f64, 0f64, 0f64);

        v != NULL_VEC // && ...
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        float tmin, tmax, tymin, tymax, tzmin, tzmax;

        let tmin = (bounds[r.sign[0]].x - r.orig.x) * r.invdir.x;
        let tmax = (bounds[1-r.sign[0]].x - r.orig.x) * r.invdir.x;
        let tymin = (bounds[r.sign[1]].y - r.orig.y) * r.invdir.y;
        let tymax = (bounds[1-r.sign[1]].y - r.orig.y) * r.invdir.y;

        if (tmin > tymax) || (tymin > tmax) {
            return Vec::new();
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (bounds[r.sign[2]].z - r.orig.z) * r.invdir.z;
        let tzmax = (bounds[1-r.sign[2]].z - r.orig.z) * r.invdir.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        return true;
    }
}

impl Normal for Rect {
    fn normal(&self, _p: Point) -> Vector {
        self.normal
    }
}

impl GetTexture for Rect {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        &self.texture
    }
}

impl ObjectId for Rect {
    fn id(&self) -> &'static str {
        self.id
    }
}

impl ObjectTrait for Rect {}

#[cfg(test)]
mod tests {
    use crate::{Color, UniformTexture};

    const UNIFORM_TEXTURE: UniformTexture = UniformTexture {
        kd: 1f64,
        ka: 1f64,
        ks: 1f64,

        color: Color::BLACK,
    };
}
