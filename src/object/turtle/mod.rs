pub mod grammar;
pub mod state;

use crate::object::rect::RectangleInner;
use crate::object::turtle::grammar::parse_grammar;
use crate::object::turtle::state::State;
use crate::object::{GetTexture, Intersect, Normal, ObjectId, ObjectTrait};
use crate::texture::TextureTrait;
use crate::{Color, Point, UniformTexture, Vector};
use std::cell::RefCell;

use crate::object::sphere::Sphere;
use crate::object::triangle::Triangle;

/// Object
pub struct Turtle {
    pub objects: Vec<Box<dyn ObjectTrait>>,
    pub texture: Box<dyn TextureTrait>,
    pub id: String,

    pub rect: RectangleInner,

    pub latest_hit: RefCell<Option<usize>>,
}

impl Turtle {
    fn generate(s: String, angle: f64, ka: f64) -> (Vec<Box<dyn ObjectTrait>>, Vector, Vector) {
        let mut res = Vec::<Box<dyn ObjectTrait>>::new();
        let mut states = Vec::new();
        let mut polygon_edges = Vec::new();

        let mut current = State {
            position: Vector::ZERO,
            head: Vector::new(1f64, 0f64, 0f64),
            up: Vector::new(0f64, 1f64, 0f64),
            left: Vector::new(0f64, 0f64, 1f64),
            color_index: 0,
            radius: 0.2f64,
        };

        let mut min = Vector::ZERO;
        let mut max = Vector::ZERO;

        for c in s.chars() {
            match c {
                // Forward
                'F' => {
                    current.move_forward(0.4f64);
                    res.push(Box::new(Sphere {
                        p: current.position,
                        id: format!("Turtle {}", res.len()),
                        r: current.radius,
                        texture: Box::new(UniformTexture {
                            kd: 1f64,
                            ka,
                            ks: 0.1f64,

                            color: Color::new(103, 78, 31),
                        }),
                    }));

                    min = min.min_against(&(current.position - current.radius));
                    max = max.max_against(&(current.position + current.radius));
                }
                // Turn left
                '+' => current.rotate_up(angle),
                // Turn right
                '-' => current.rotate_up(-angle),
                // Turn up
                '&' => current.rotate_left(angle),
                // Turn down
                '^' => current.rotate_left(-angle),
                // Rotate clockwise
                '/' => current.rotate_head(angle),
                // Rotate anti-clockwise
                '\\' => current.rotate_head(-angle),
                // U-Turn
                '|' => current.rotate_up(180f64),
                // Place polygon egde
                'f' => {
                    min = min.min_against(&current.position);
                    max = max.max_against(&current.position);

                    polygon_edges.push(current.position.clone());
                    current.move_forward(0.4f64);
                }
                // Decrement diameter
                '!' => current.radius /= 1.3f64,
                // Increment index color table
                '\'' => {}
                // Starts a polygon
                '{' => polygon_edges.clear(),
                // Closes a polygon
                '}' => {
                    let start = polygon_edges[0];

                    for (idx, win) in polygon_edges.windows(2).skip(1).enumerate() {
                        let (first, second) = (win[0], win[1]);
                        let triangle = Triangle::new(
                            start,
                            first,
                            second,
                            Box::new(UniformTexture {
                                kd: 1f64,
                                ks: 0.2f64,
                                ka: 0.2f64,
                                color: Color::GREEN,
                            }),
                            format!("Triangle {}", idx),
                        );

                        res.push(Box::new(triangle));
                    }
                }
                // Saves states
                '[' => states.push(current.clone()),
                // Restores state
                ']' => current = states.pop().unwrap(),
                _ => {}
            };
        }

        dbg!(res.len());
        (res, min, max)
    }

    pub fn new(path: String, texture: Box<dyn TextureTrait>, id: String) -> Turtle {
        let g = parse_grammar(path).unwrap();
        let s = g.expand();

        dbg!(&texture.coefficients(Point::ZERO).2);
        let (objects, min, max) = Turtle::generate(s, g.angle, texture.coefficients(Point::ZERO).2);

        Turtle {
            objects,
            id,
            texture,
            rect: RectangleInner::new(min, max),
            latest_hit: RefCell::new(None),
        }
    }
}

impl Intersect for Turtle {
    fn is_intersect(&self, _p: Point, _v: Vector) -> bool {
        false
    }

    fn intersect_points(&self, p: Point, v: Vector) -> Vec<Point> {
        if !self.rect.is_intersect(p, v) {
            return Vec::new();
        }

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
        let old = self.latest_hit.take();
        if let Some(index) = old {
            self.latest_hit.replace(old);
            self.objects[index].normal(p)
        } else {
            Vector::ZERO
        }
    }
}

impl GetTexture for Turtle {
    fn texture(&self) -> &Box<dyn TextureTrait> {
        let old = self.latest_hit.take();
        if let Some(index) = old {
            self.latest_hit.replace(old);
            self.objects[index].texture()
        } else {
            &self.texture
        }
    }
}

impl ObjectId for Turtle {
    fn id(&self) -> &String {
        let old = self.latest_hit.take();
        if let Some(index) = old {
            self.latest_hit.replace(old);
            self.objects[index].id()
        } else {
            &self.id
        }
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
