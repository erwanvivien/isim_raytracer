use crate::camera::Camera;
use crate::color::Color;
use crate::light::point::PointLight;
use crate::object::sphere::Sphere;
use crate::point::Point;
use crate::scene::Scene;
use crate::texture::uniform::UniformTexture;
use crate::vector::Vector;

mod camera;
mod color;
mod image;
mod light;
mod object;
mod point;
mod scene;
mod texture;
mod vector;

const CAMERA_CENTER: Point = Point::new(0f64, 0f64, 0f64);
const SPOTTED_POINT: Point = Point::new(1f64, 0f64, 0f64);
const UP: Vector = Vector::new(0f64, 1f64, 0f64);

const OBJ1_POINT: Point = Point::new(5f64, 0f64, 0f64);
const OBJ2_POINT: Point = Point::new(5f64, 0f64, 3f64);

const OBJ1_COLOR: Color = Color::new(66, 135, 245);
const OBJ2_COLOR: Color = Color::new(227, 66, 245);

const LIGHT_CENTER: Point = Point::new(0f64, 2f64, -3f64);

fn main() {
    let cam = Camera::new(
        CAMERA_CENTER,
        SPOTTED_POINT,
        UP,
        std::f64::consts::FRAC_PI_2 * 110f64 / 90f64,
    );

    let obj1 = Sphere {
        p: OBJ1_POINT,
        r: 2f64,
        texture: Box::new(UniformTexture {
            kd: 1f64,
            ka: 1f64,
            ks: 1f64,
            color: OBJ1_COLOR,
        }),
    };
    let obj2 = Sphere {
        p: OBJ2_POINT,
        r: 1f64,
        texture: Box::new(UniformTexture {
            kd: 1f64,
            ka: 1f64,
            ks: 1f64,

            color: OBJ2_COLOR,
        }),
    };

    let light = PointLight {
        intensity: Vector::new(1f64, 1f64, 1f64),
        point: LIGHT_CENTER,
    };

    let scene = Scene {
        cam,
        lights: vec![Box::new(light)],
        objects: vec![Box::new(obj1), Box::new(obj2)],
    };

    let img = scene.image(400 * 2, 600 * 2);
    let _ = img.save_png("test.png");
}
