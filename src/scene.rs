use crate::camera::Camera;
use crate::color::Color;
use crate::img::Image;
use crate::light::LightTrait;
use crate::object::ObjectTrait;
use crate::{Point, Vector};
use std::io::Write;

pub struct Scene {
    pub cam: Camera,
    pub lights: Vec<Box<dyn LightTrait>>,
    pub objects: Vec<Box<dyn ObjectTrait>>,
}

const MAX_REC: usize = 0;

impl Scene {
    pub fn image(&self, height: usize, width: usize) -> Image {
        let mut img = Image::new(height, width);

        let gx = (self.cam.alpha / 2f64).tan();
        let gy = gx * ((height - 1) as f64 / (width - 1) as f64);

        let qx = self.cam.right * 2.0 * gx / ((width - 1) as f64);
        let qy = self.cam.up * 2.0 * gy / ((height - 1) as f64);

        let p_top_left =
            self.cam.center + self.cam.forward - self.cam.right * gx + self.cam.up * gy;

        let size = height;
        for (_ii, i) in (0..height).map(|i| qy * i as f64).enumerate() {
            // print!("{:.2}%\r", (_ii * 100) as f64 / (size as f64));
            // std::io::stdout().flush().unwrap();

            for (_jj, j) in (0..width).map(|j| qx * j as f64).enumerate() {
                let p_pixel = p_top_left + j - i;
                let v = (p_pixel - self.cam.center).normalize();
                // dbg!((ii, jj));

                let collision = self.cast_ray(self.cam.center, v);
                if collision.is_none() {
                    img.push(Color::WHITE);
                    continue;
                }

                let (p, obj, _) = collision.unwrap();
                let color = self.cast_ray_rebound(p, obj, v, 0);

                match color {
                    Some(c) => img.push(c),
                    None => img.push(Color::BLACK),
                }
            }
        }
        // println!();

        img
    }

    #[allow(dead_code)]
    pub fn with_cam(&mut self, cam: Camera) -> &mut Self {
        self.cam = cam;
        self
    }

    fn cast_ray_rebound(
        &self,
        p: Point,
        obj: &Box<dyn ObjectTrait>,
        v: Vector,
        rec: usize,
    ) -> Option<Color> {
        let mut normal = obj.normal(p);
        if normal * self.cam.forward > 0f64 {
            normal = -normal;
        }

        let reflect = v - normal * (v * normal) * 2f64;
        let (kd, ks, ka) = obj.texture().coefficients(p);

        let mut current_color = Color::BLACK;

        for light in &self.lights {
            let l_vec = light.point() - p;
            let l_dist = l_vec.mag();

            let intensity = light.intensity();

            let i_d = (obj.texture().color(p).v).mul(intensity.normalize())
                * kd
                * (normal * l_vec.normalize());

            let rl = reflect * l_vec.normalize();
            let i_s = intensity * (ks * rl.powf(50f64)).copysign(rl);
            let out = i_d + i_s + obj.texture().color(p).v * ka;

            let intersect = self.cast_ray(p, l_vec);
            if let Some((_i_p, _i_obj, i_dist)) = intersect {
                if i_dist < l_dist {
                    current_color.v = current_color.v + out;
                    // current_color.v = current_color.v + i_s;
                    continue;
                }
            }

            current_color.v = current_color.v + out;
        }

        if rec == MAX_REC {
            return Some(current_color);
        }

        let rebound = self.cast_ray(p, reflect);
        if rebound.is_none() {
            return Some(current_color);
        }

        let (col_p, col_obj, _) = rebound.unwrap();
        let color = self.cast_ray_rebound(col_p, col_obj, reflect, rec + 1);

        if let Some(c) = color {
            let c = c.v * ks / 2f64;
            current_color.v = current_color.v + c;
        }

        current_color.v = current_color.v.clamp(0f64, 255f64);
        Some(current_color)
    }

    fn cast_ray(&self, p: Point, v: Vector) -> Option<(Point, &Box<dyn ObjectTrait>, f64)> {
        let (mut closest_obj, mut point) = (None, None);
        let mut distance = f64::MAX;

        for obj in &self.objects {
            let intersect_points = obj.intersect_points(p, v);
            if intersect_points.is_empty() {
                continue;
            }

            let intersect = intersect_points
                .into_iter()
                .map(|ip| (ip, (ip - p).mag()))
                .filter(|&(_, distance)| distance > 0.000001f64)
                .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
            if intersect.is_none() {
                continue;
            }
            let intersect = intersect.unwrap();

            // Avoid collision with self
            if intersect.1 < distance {
                distance = intersect.1;
                closest_obj = Some(obj);
                point = Some(intersect.0);
            }
        }

        closest_obj?;

        Some((point.unwrap(), closest_obj.unwrap(), distance))
    }
}
