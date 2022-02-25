use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::light::Lighting;
use crate::object::{Intersect, Normal};

pub struct Scene<L: Lighting, O: Intersect + Normal> {
    pub cam: Camera,
    pub lights: Vec<L>,
    pub objects: Vec<O>,
}

impl<L: Lighting, O: Intersect + Normal> Scene<L, O> {
    pub fn image(&self, height: usize, width: usize) -> Image {
        let mut img = Image::new(height, width);

        let cam = &self.cam;

        let beta_half = cam.beta / 2f64;
        let alpha_half = cam.alpha / 2f64;

        for i in (0..height).map(|v| (v as f64) - beta_half) {
            for j in (0..width).map(|v| (v as f64) - alpha_half) {
                let v = cam
                    .forward
                    .rotate_around(&cam.up, i)
                    .rotate_around(&cam.right, j);

                let mut color = Color::WHITE;

                for obj in &self.objects {
                    let intersect = obj.is_intersect(self.cam.center, v);
                    if intersect {
                        color = Color::BLACK
                    }
                }

                img.push(color);
            }
        }

        return img;
    }
}
