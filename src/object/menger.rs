use crate::object::rect::RectangleInner;
use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Point, Vector};
use std::cell::RefCell;

#[derive(Debug)]
pub struct MengerRec {
    pub rect: RectangleInner,
    pub sub_menger: Vec<MengerRec>,
    pub latest_hit: RefCell<Option<usize>>,
}

impl MengerRec {
    pub fn new(rec_count: usize, p1: Point, p2: Point) -> MengerRec {
        if rec_count == 0 {
            return MengerRec {
                sub_menger: Vec::new(),
                rect: RectangleInner::new(p1, p2),
                latest_hit: RefCell::new(None),
            };
        }

        let diff = (p2 - p1) / 3f64;

        let sub = (0..27)
            .filter_map(|i| {
                let k = (i % 3) as f64;
                let j = ((i % 9) / 3) as f64;
                let i = (i / 9) as f64;

                if (i == 1f64 && j == 1f64) || (j == 1f64 && k == 1f64) | (k == 1f64 && i == 1f64) {
                    return None;
                }

                let p_min = p1 + Point::new(i * diff.x, j * diff.y, k * diff.z);

                Some(MengerRec::new(rec_count - 1, p_min, p_min + diff))
            })
            .collect::<Vec<_>>();

        assert_eq!(sub.len(), 20);

        MengerRec {
            rect: RectangleInner::new(p1, p2),
            sub_menger: sub,
            latest_hit: RefCell::new(None),
        }
    }
}

/// Object
pub struct Menger {
    pub menger: MengerRec,
    pub texture: Box<dyn TextureTrait>,
    pub id: String,
}

impl Menger {
    pub fn new(
        rec_count: usize,
        p1: Point,
        p2: Point,
        texture: Box<dyn TextureTrait>,
        id: String,
    ) -> Menger {
        let mut xs = [p1.x, p2.x];
        let mut ys = [p1.y, p2.y];
        let mut zs = [p1.z, p2.z];
        xs.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());
        ys.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());
        zs.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());

        let p1 = Point::new(xs[0], ys[0], zs[0]);
        let p2 = Point::new(xs[1], ys[1], zs[1]);

        Menger {
            menger: MengerRec::new(rec_count, p1, p2),
            id,
            texture,
        }
    }
}

impl Intersect for MengerRec {
    fn is_intersect(&self, _p: Point, v: Vector) -> bool {
        const NULL_VEC: Vector = Vector::new(0f64, 0f64, 0f64);
        v != NULL_VEC // && (v * self.normal) != 0f64
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        let inter = self.rect.intersect_points(p, v);
        if inter.is_empty() || self.sub_menger.is_empty() {
            return inter;
        }

        let mut best_dist = f64::MAX;
        let mut best_inter = Vec::new();

        for i in 0..self.sub_menger.len() {
            let hits = self.sub_menger[i].intersect_points(p, v);
            if hits.is_empty() {
                continue;
            }

            let dist = (hits[0] - p).powf(2f64).sum();
            if dist < best_dist {
                self.latest_hit.replace(Some(i));
                best_dist = dist;
                best_inter = hits;
            }
        }

        best_inter
    }
}

impl Normal for MengerRec {
    fn normal(&self, p: Point) -> Vector {
        if let Some(index) = self.latest_hit.take() {
            self.sub_menger[index].normal(p)
        } else {
            self.rect.normal(p)
        }
    }
}

impl Intersect for Menger {
    fn is_intersect(&self, p: Point, v: Vector) -> bool {
        self.menger.is_intersect(p, v)
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        self.menger.intersect_points(p, v)
    }
}

impl Normal for Menger {
    fn normal(&self, p: Point) -> Vector {
        self.menger.normal(p)
    }
}

impl GetTexture for Menger {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        &self.texture
    }
}

impl ObjectId for Menger {
    fn id(&self) -> &String {
        &self.id
    }
}

impl ObjectTrait for Menger {}

#[cfg(test)]
mod tests {
    use crate::object::Intersect;
    use crate::{Color, Point, UniformTexture, Vector};

    const UNIFORM_TEXTURE: UniformTexture = UniformTexture {
        kd: 1f64,
        ka: 1f64,
        ks: 1f64,

        color: Color::BLACK,
    };
}
