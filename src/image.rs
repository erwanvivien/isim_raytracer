//! Basic image representation with `Color` list
use crate::color::Color;
use std::fs;
use std::io::Write;

/// Contains list of `color::Color`
pub struct Image {
    height: usize,
    width: usize,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(height: usize, width: usize) -> Self {
        Image {
            height,
            width,
            pixels: Vec::with_capacity(height * width),
        }
    }

    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    pub fn push(&mut self, c: Color) {
        self.pixels.push(c);
    }

    #[allow(dead_code)]
    pub fn save(&self, filename: &str) -> bool {
        if self.pixels.len() != (self.height * self.width) {
            return false;
        }

        let header = format!(
            "P6 {width} {height} 255\n",
            width = self.width,
            height = self.height
        );

        let mut content = header;

        for chunks in self.pixels.chunks(23) {
            let mut s = String::with_capacity(69);
            for c in chunks {
                s.push(char::from(c.r));
                s.push(char::from(c.g));
                s.push(char::from(c.b));
            }

            content.push_str(&*s);
            content.push('\n');
        }

        let buffer = fs::File::create(filename);
        if buffer.is_err() {
            return false;
        }

        let content = content.as_bytes();
        let mut buffer = buffer.unwrap();

        let mut pos = 0;
        while pos < content.len() {
            let bytes_written = buffer.write(&content[pos..]);
            if bytes_written.is_err() {
                return false;
            }

            pos += bytes_written.unwrap();
        }

        true
    }

    #[allow(dead_code)]
    pub fn save_png(&self, filename: &str) -> bool {
        let tmp = &self
            .pixels
            .iter()
            .map(|c| [c.r, c.g, c.b])
            .flatten()
            .collect::<Vec<u8>>();

        image::save_buffer(
            filename,
            &tmp[..],
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
        true
    }
}

mod tests {
    use crate::image::Image;
    use crate::{Camera, Color, Point, PointLight, Scene, Sphere, UniformTexture, Vector};
    use image::GenericImageView;

    #[test]
    fn new() {
        let image = Image::new(16, 16);
        assert_eq!(image.pixels.capacity(), image.height * image.width)
    }

    fn check(img: &Image, path: &str) -> bool {
        // let _ = img.save_png("images/test_80x80_v1.png");
        let buffer = image::io::Reader::open(path).unwrap().decode().unwrap();

        for (_nb, (x, y, color)) in buffer.pixels().enumerate() {
            let x = x as usize;
            let y = y as usize;
            let pixel = img.pixels()[y * 80 + x];

            if [pixel.r, pixel.g, pixel.b, 255] != color.0 {
                return false;
            }
        }

        return true;
    }

    #[test]
    fn save_black_white() {
        const CAMERA_CENTER: Point = Point::new(0f64, 0f64, 0f64);
        const SPOTTED_POINT: Point = Point::new(1f64, 0f64, 0f64);
        const UP: Vector = Vector::new(0f64, 1f64, 0f64);

        const OBJ1_POINT: Point = Point::new(5f64, 0f64, 0f64);
        const OBJ2_POINT: Point = Point::new(5f64, 0f64, 3f64);

        const LIGHT_CENTER: Point = Point::new(0f64, 2f64, 0f64);

        let cam = Camera::new(
            CAMERA_CENTER,
            SPOTTED_POINT,
            UP,
            std::f64::consts::FRAC_PI_2,
            std::f64::consts::FRAC_PI_2,
        );

        let obj1 = Sphere {
            p: OBJ1_POINT,
            r: 2f64,
            texture: Box::new(UniformTexture {
                kd: 1f64,
                ka: 1f64,
                ks: 1f64,
                color: Color::BLACK,
            }),
        };
        let obj2 = Sphere {
            p: OBJ2_POINT,
            r: 1f64,
            texture: Box::new(UniformTexture {
                kd: 1f64,
                ka: 1f64,
                ks: 1f64,
                color: Color::BLACK,
            }),
        };

        let light = PointLight {
            point: LIGHT_CENTER,
        };

        let scene = Scene {
            cam,
            lights: vec![Box::new(light)],
            objects: vec![Box::new(obj1), Box::new(obj2)],
        };

        let img = scene.image(80, 80);

        assert_eq!(true, check(&img, "images/80x80_v1_black_white.png"));
    }

    #[test]
    fn save_color() {
        const CAMERA_CENTER: Point = Point::new(0f64, 0f64, 0f64);
        const SPOTTED_POINT: Point = Point::new(1f64, 0f64, 0f64);
        const UP: Vector = Vector::new(0f64, 1f64, 0f64);

        const OBJ1_POINT: Point = Point::new(5f64, 0f64, 0f64);
        const OBJ2_POINT: Point = Point::new(5f64, 0f64, 3f64);

        const LIGHT_CENTER: Point = Point::new(0f64, 2f64, 0f64);

        let cam = Camera::new(
            CAMERA_CENTER,
            SPOTTED_POINT,
            UP,
            std::f64::consts::FRAC_PI_2,
            std::f64::consts::FRAC_PI_2,
        );

        let obj1 = Sphere {
            p: OBJ1_POINT,
            r: 2f64,
            texture: Box::new(UniformTexture {
                kd: 1f64,
                ka: 1f64,
                ks: 1f64,
                color: Color {
                    r: 66,
                    g: 135,
                    b: 245,
                },
            }),
        };
        let obj2 = Sphere {
            p: OBJ2_POINT,
            r: 1f64,
            texture: Box::new(UniformTexture {
                kd: 1f64,
                ka: 1f64,
                ks: 1f64,

                color: Color {
                    r: 227,
                    g: 66,
                    b: 245,
                },
            }),
        };

        let light = PointLight {
            point: LIGHT_CENTER,
        };

        let scene = Scene {
            cam,
            lights: vec![Box::new(light)],
            objects: vec![Box::new(obj1), Box::new(obj2)],
        };

        let img = scene.image(80, 80);
        assert_eq!(true, check(&img, "images/80x80_v2_color.png"));
    }
}
