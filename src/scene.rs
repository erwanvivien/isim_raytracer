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

        let height_f = height as f64;
        let width_f = width as f64;
        let height_half = height_f / 2f64;
        let width_half = width_f / 2f64;

        for i in (0..height).map(|i_usize| cam.beta * (i_usize as f64 - height_half) / height_f) {
            let v = cam.forward.rotate_around(&cam.right, i);
            for j in (0..width).map(|j_usize| cam.alpha * (j_usize as f64 - width_half) / width_f) {
                let v = v.rotate_around(&cam.up, j);
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
