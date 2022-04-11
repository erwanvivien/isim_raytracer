use crate::camera::Camera;
use crate::color::Color;
use crate::light::point::PointLight;
use crate::object::menger::Menger;
use crate::object::turtle::Turtle;
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

const CAMERA_CENTER: Point = Point::new(-5f64, 2f64, 2f64 + OFFSET);
const SPOTTED_POINT: Point = Point::new(0f64, 2f64, 2f64 + OFFSET);
const UP: Vector = Vector::new(0f64, 1f64, 0f64);

const LIGHT_CENTER: Point = Point::new(-3f64, 2f64, 0f64 + OFFSET);

fn main() {
    let menger = Menger::new(
        3,
        Point::new(0f64, 0f64, 0f64),
        Point::new(4f64, 4f64, 4f64),
        Box::new(UniformTexture {
            kd: 1f64,
            ka: 0f64,
            ks: 0.1f64,

            color: Color::RED,
        }),
        String::from("rect"),
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

    // let turtle = Turtle::new(
    //     String::from("./grammar.json"),
    //     Box::new(UniformTexture {
    //         kd: 1f64,
    //         ka: 0f64,
    //         ks: 0.1f64,
    //
    //         color: Color::WHITE,
    //     }),
    //     String::from("turtle"),
    // );

    let objs: Vec<Box<dyn ObjectTrait>> = vec![Box::new(menger)];

    let scene = Scene {
        cam,
        lights: vec![Box::new(light)],
        objects: objs,
    };

    let img = scene.image(1080 / 2, 1920 / 2);
    img.save_png("output_0.png");
}
