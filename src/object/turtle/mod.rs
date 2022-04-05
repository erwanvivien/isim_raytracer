pub mod grammar;
pub mod state;

use crate::object::turtle::grammar::parse_grammar;
use crate::object::turtle::state::State;
use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Color, Point, UniformTexture, Vector};
use std::cell::RefCell;

use crate::object::sphere::Sphere;

/// Object
pub struct Turtle {
    pub objects: Vec<Box<dyn ObjectTrait>>,
    pub texture: Box<dyn TextureTrait>,
    pub id: &'static str,

    pub latest_hit: RefCell<Option<usize>>,
}

impl Turtle {
    fn generate(s: String, angle: f64) -> Vec<Box<dyn ObjectTrait>> {
        let mut res = Vec::<Box<dyn ObjectTrait>>::new();
        let mut states = Vec::new();

        let mut current = State {
            position: Vector::ZERO,
            head: Vector::new(1f64, 0f64, 0f64),
            up: Vector::new(0f64, 1f64, 0f64),
            left: Vector::new(0f64, 0f64, 1f64),
        };

        for c in s.chars() {
            let last = &mut current;

            match c {
                'F' => {
                    last.move_forward(0.4f64);
                    res.push(Box::new(Sphere {
                        p: last.position,
                        id: "turtle",
                        r: 0.2f64,
                        texture: Box::new(UniformTexture {
                            kd: 1f64,
                            ka: 0f64,
                            ks: 0.1f64,

                            color: Color::WHITE,
                        }),
                    }))
                }
                '+' => last.rotate_up(angle),
                '-' => last.rotate_up(-angle),
                '&' => last.rotate_left(angle),
                '^' => last.rotate_left(-angle),
                '/' => last.rotate_head(angle),
                '\\' => last.rotate_head(-angle),
                '|' => last.rotate_head(180f64),
                '[' => states.push(last.clone()),
                ']' => {
                    current = states.pop().unwrap();
                }
                _ => {}
            };
        }

        res
    }

    pub fn new(path: String, texture: Box<dyn TextureTrait>, id: &'static str) -> Turtle {
        let g = parse_grammar(path).unwrap();
        let s = g.expand();

        let objects = Turtle::generate(s, g.angle);

        Turtle {
            objects,
            id,
            texture,
            latest_hit: RefCell::new(None),
        }
    }
}

impl Intersect for Turtle {
    fn is_intersect(&self, _p: Point, _v: Vector) -> bool {
        false
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        let mut best_dist = f64::MAX;
        let mut best_inter = Vec::new();

        for i in 0..self.objects.len() {
            let hits = self.objects[i].intersect_points(p, v);
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

impl Normal for Turtle {
    fn normal(&self, p: Point) -> Vector {
        if let Some(index) = self.latest_hit.take() {
            self.objects[index].normal(p)
        } else {
            Vector::ZERO
        }
    }
}

impl GetTexture for Turtle {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        &self.texture
    }
}

impl ObjectId for Turtle {
    fn id(&self) -> &'static str {
        self.id
    }
}

impl ObjectTrait for Turtle {}

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
