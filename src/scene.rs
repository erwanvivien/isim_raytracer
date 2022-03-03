use crate::camera::Camera;
use crate::color::Color;
use crate::img::Image;
use crate::light::LightTrait;
use crate::object::ObjectTrait;
use crate::{Point, Vector};

pub struct Scene {
    pub cam: Camera,
    pub lights: Vec<Box<dyn LightTrait>>,
    pub objects: Vec<Box<dyn ObjectTrait>>,
}

const MAX_REC: usize = 3;

impl Scene {
    pub fn image(&self, height: usize, width: usize) -> Image {
        let mut img = Image::new(height, width);

        let cam = &self.cam;

        let height_half = (height / 2) as i64;
        let width_half = (width / 2) as i64;

        let beta = cam.alpha * (height as f64 / width as f64);

        let step_y = beta / height as f64;
        let step_x = cam.alpha / width as f64;

        for i in (-height_half..height_half).map(|i| step_y * i as f64) {
            let v = cam.forward.rotate_around(&cam.right, i);
            for j in (-width_half..width_half).map(|j| step_x * j as f64) {
                let v = v.rotate_around(&cam.up, j);

                let collision = self.cast_ray(cam.center, v);
                if collision.is_none() {
                    img.push(Color::BLACK);
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

        return img;
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
        if rec == MAX_REC {
            return Some(Color::BLACK);
        }

        let normal = obj.normal(p);
        let reflect = v - normal * (v * normal) * 2f64;
        let (kd, ks, _ka) = obj.texture().coefficients(p);

        let mut current_color = Color::BLACK;

        for light in &self.lights {
            let l_vec = light.point() - p;
            let l_dist = l_vec.mag();

            let intensity = light.intensity();

            let i_d = (obj.texture().color(p).v).mul(intensity.normalize())
                * kd
                * (normal * l_vec.normalize());

            let i_s = intensity * ks * (reflect * l_vec.normalize()).powf(50f64);
            let out = i_d + i_s;

            let intersect = self.cast_ray(p, l_vec);
            if let Some((_i_p, _i_obj, i_dist)) = intersect {
                if i_dist < l_dist {
                    // current_color.v = current_color.v + i_s;
                    continue;
                }
            }

            current_color.v = current_color.v + out;
        }

        let rebound = self.cast_ray(p, reflect);
        if rebound.is_none() {
            return Some(current_color);
        }

        let (col_p, col_obj, col_distance) = rebound.unwrap();
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
            if intersect_points.len() <= 0 {
                continue;
            }

            let intersect = intersect_points
                .into_iter()
                .map(|ip| (ip, (ip - p).mag()))
                .filter(|&(_, distance)| distance > 0.000001f64)
                .min_by(|(_, d1), (_, d2)| d1.partial_cmp(&d2).unwrap());
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

        if closest_obj.is_none() {
            return None;
        }

        return Some((point.unwrap(), closest_obj.unwrap(), distance));
    }
}
