use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::light::LightTrait;
use crate::object::ObjectTrait;
use crate::{Point, Vector};

pub struct Scene {
    pub cam: Camera,
    pub lights: Vec<Box<dyn LightTrait>>,
    pub objects: Vec<Box<dyn ObjectTrait>>,
}

fn cast_ray_rebound(p: Point, obj: &Box<dyn ObjectTrait>, v: Vector, scene: &Scene) -> Color {
    let normal = obj.normal(p);

    for lights in &scene.lights {
        let l_vec = lights.point() - p;
        let intensity = lights.intensity();

        let (kd, ks, ka) = obj.texture().coefficients(p);

        let out = (obj.texture().color(p).to_vec()).mul(intensity) * kd * (normal * l_vec);

        return out.into();
    }

    Color::BLACK
}

fn cast_ray_cam(v: Vector, scene: &Scene) -> Color {
    let (mut closest_obj, mut point) = (None, None);
    let mut distance = f64::MAX;

    for obj in &scene.objects {
        let intersect_points = obj.intersect_points(scene.cam.center, v);
        if intersect_points.len() <= 0 {
            continue;
        }

        let last = intersect_points.last().unwrap().clone();
        let mut intersect = intersect_points
            .into_iter()
            .map(|p| p.mag())
            .collect::<Vec<_>>();
        intersect.sort_by_key(|&p| p as u128);

        let d = *intersect.last().unwrap();
        if d < distance {
            distance = d;
            closest_obj = Some(obj);
            point = Some(last);
        }
    }

    if closest_obj.is_none() {
        return Color::BLACK;
    }

    return cast_ray_rebound(point.unwrap(), closest_obj.unwrap(), v, scene);
}

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

                let color = cast_ray_cam(v, self);

                img.push(color);
            }
        }

        return img;
    }
}
