use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::light::LightTrait;
use crate::object::ObjectTrait;
use crate::texture::TextureTrait;

pub struct Scene {
    pub cam: Camera,
    pub lights: Vec<Box<dyn LightTrait>>,
    pub objects: Vec<Box<dyn ObjectTrait>>,
}

impl Scene {
    pub fn image(&self, height: usize, width: usize) -> Image {
        let mut img = Image::new(height, width);

        let cam = &self.cam;

        let height_f = height as f64;
        let width_f = width as f64;
        let height_half = height_f / 2f64;
        let width_half = width_f / 2f64;

        for i in (0..height).map(|i_usize| cam.beta * (i_usize as f64 - height_half) / height_f) {
            let v = cam.forward.rotate_around(&cam.right, i);
            for j in (0..width).map(|j_usize| cam.alpha * (j_usize as f64 - width_half) / width_f) {
                let v = v.rotate_around(&cam.up, j);
                let mut color = Color::WHITE;
                let mut distance = f64::MAX;

                for obj in &self.objects {
                    let mut intersect = obj
                        .intersect_points(self.cam.center, v)
                        .into_iter()
                        .map(|p| p.to_vec().mag())
                        .collect::<Vec<_>>();
                    intersect.sort_by_key(|&p| p as u128);

                    if &intersect.len() > &0 {
                        let d = *intersect.last().unwrap();

                        if d < distance {
                            color = (*obj.texture()).color();
                            distance = d;
                        }
                    }
                }

                img.push(color);
            }
        }

        return img;
    }
}
