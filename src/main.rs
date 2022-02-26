use crate::camera::Camera;
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

const LIGHT_CENTER: Point = Point::new(0f64, 2f64, 0f64);

fn main() {
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
        texture: UniformTexture {
            kd: 1f64,
            ka: 1f64,
            ks: 1f64,
        },
    };
    let obj2 = Sphere {
        p: OBJ2_POINT,
        r: 1f64,
        texture: UniformTexture {
            kd: 1f64,
            ka: 1f64,
            ks: 1f64,
        },
    };

    let light = PointLight {
        point: LIGHT_CENTER,
    };

    let scene = Scene {
        cam,
        lights: vec![light],
        objects: vec![obj1, obj2],
    };

    let img = scene.image(800, 800);
    let _ = img.save_png("test.png");
}
