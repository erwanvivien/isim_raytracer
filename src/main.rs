use image::math::Rect;
use object::rect::Rectangle;
use rand::{RngCore, SeedableRng};

use crate::camera::Camera;
use crate::color::Color;
use crate::light::point::PointLight;
use crate::object::plane::Plane;
use crate::object::sphere::Sphere;
use crate::object::triangle::Triangle;
use crate::object::ObjectTrait;
use crate::point::Point;
use crate::scene::Scene;
use crate::texture::uniform::UniformTexture;
use crate::vector::Vector;

mod camera;
mod color;
mod img;
mod light;
mod object;
mod point;
mod scene;
mod texture;
mod vector;

const OFFSET: f64 = 0f64;

const CAMERA_CENTER: Point = Point::new(-3f64, -1f64, -5f64 + OFFSET);
const SPOTTED_POINT: Point = Point::new(0f64, -1f64, 0f64 + OFFSET);
const UP: Vector = Vector::new(0f64, 1f64, 0f64);

const OBJ1_POINT: Point = Point::new(10f64, 5f64, 0f64);
const OBJ2_POINT: Point = Point::new(10f64, 0f64, 0f64);

const LIGHT_CENTER: Point = Point::new(-3f64, -1f64, -5f64 + OFFSET);

fn main() {
    let obj1_color = Color::new(66, 135, 245);
    let obj2_color = Color::new(227, 66, 245);
    let pla1_color = Color::new(212, 104, 104);

    let obj1 = Sphere {
        p: OBJ1_POINT,
        r: 1f64,
        texture: Box::new(UniformTexture {
            kd: 1f64,
            ka: 1f64,
            ks: 0.4f64,
            color: obj1_color,
        }),
        id: "obj1",
    };
    let obj2 = Sphere {
        p: OBJ2_POINT,
        r: 2f64,
        texture: Box::new(UniformTexture {
            kd: 1f64,
            ka: 1f64,
            ks: 0.4f64,

            color: obj2_color,
        }),
        id: "obj2",
    };

    let triangle = Triangle::new(
        Point::new(0f64, 0f64, -5f64),
        Point::new(0f64, 0f64, 5f64),
        Point::new(0f64, 10f64, 0f64),
        Box::new(UniformTexture {
            kd: 1f64,
            ka: 1f64,
            ks: 0.4f64,
            color: Color::RED,
        }),
        "Triangle",
    );

    let plane1 = Plane {
        normal: Vector::new(0f64, 1f64, 0f64),
        p: Point::new(0f64, -2f64, 0f64),
        id: "plane1",
        texture: Box::new(UniformTexture {
            kd: 1f64,
            ka: 0f64,
            ks: 0.1f64,

            color: pla1_color,
        }),
    };

    let rect = Rectangle::new(
        Point::new(1f64, 1f64, 1f64 + OFFSET),
        Point::new(0f64, 0f64, 0f64 + OFFSET),
        Box::new(UniformTexture {
            kd: 1f64,
            ka: 0f64,
            ks: 0.1f64,

            color: Color::RED,
        }),
        "rect",
    );

    let light = PointLight {
        intensity: Vector::new(255f64, 255f64, 255f64),
        point: LIGHT_CENTER,
    };

    let cam = Camera::new(
        CAMERA_CENTER,
        SPOTTED_POINT,
        UP,
        std::f64::consts::FRAC_PI_2,
    );

    let mut objs: Vec<Box<dyn ObjectTrait>> = vec![
        // Box::new(obj1),
        // Box::new(obj2),
        // Box::new(plane1),
        // Box::new(triangle),
        Box::new(rect),
    ];

    // let mut prng = rand::rngs::StdRng::seed_from_u64(3);
    // let mut next_float = || prng.next_u32() as f64 / u32::MAX as f64;
    // for _ in 0..10 {
    //     let sphere = Sphere {
    //         p: Point {
    //             x: 10f64,
    //             y: next_float() * 10f64,
    //             z: next_float() * 40f64 - 20f64,
    //         },
    //         r: next_float() * 2f64,
    //         texture: Box::new(UniformTexture {
    //             kd: 1f64,
    //             ka: 1f64,
    //             ks: 0.3f64,
    //
    //             color: Color {
    //                 v: Vector {
    //                     x: next_float() * 255f64,
    //                     y: next_float() * 255f64,
    //                     z: next_float() * 255f64,
    //                 },
    //             },
    //         }),
    //         id: "random",
    //     };
    //
    //     objs.push(Box::new(sphere));
    // }

    let scene = Scene {
        cam,
        lights: vec![Box::new(light)],
        objects: objs,
    };

    let img = scene.image(1080, 1920);
    img.save_png("output_0.png");

    // for i in (0..360).step_by(6) {
    //     println!("{}", i);
    //
    //     let angle = (i as f64) * std::f64::consts::PI * 2f64 / 360f64;
    //     let offset = Vector::new(5f64 * angle.cos(), 0f64, 5f64 * angle.sin());
    //
    //     let cam = Camera::new(
    //         OBJ1_POINT - offset,
    //         OBJ1_POINT,
    //         UP,
    //         std::f64::consts::FRAC_PI_2,
    //     );
    //
    //     let img = scene.with_cam(cam).image(1080, 1920);
    //     let _ = img.save_png(&*format!("output_{}.png", i));
    // }
    //
    // let mut file_out = File::create("out.gif").unwrap();
    // let mut encoder = GifEncoder::new(file_out);
    //
    // for i in (0..360).step_by(6) {
    //     dbg!(i);
    //     let img = image::io::Reader::open(&*format!("output_{}.png", i))
    //         .unwrap()
    //         .decode()
    //         .unwrap();
    //     let _ = encoder.encode(img.as_bytes(), 1920, 1080, ColorType::Rgb8);
    // }
}
