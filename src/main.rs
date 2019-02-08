mod camera;
mod light;
mod material;
mod ray;
mod scene;
mod shapes;
mod vector;

use camera::Camera;
use light::Light;
use light::LightType;
use material::Color;
use material::Material;
use scene::Scene;
use shapes::Shapes;
use vector::Vector3;

use std::time::Instant;

fn main() {
    let scene = Scene {
        camera: Camera::new(
            Vector3::new(0.0, -3.0, 5.0),
            Vector3::new(0.0, 0.0, -20.0),
            1.0,
            60,
            -30,
            1024,
            768,
        ),
        objects: vec![
            Shapes::sphere(
                Vector3::new(-3.0, -5.0, -16.0),
                2.8,
                Material::new(Color::from_rgb(0xff, 0x55, 0x55), 0.6, 50.0, 100.0, 0.0),
            ),
            Shapes::sphere(
                Vector3::new(0.0, -5.0, -13.0),
                2.0,
                Material::new(Color::from_rgb(0x40, 0xe0, 0xd0), 0.6, 5.0, 500.0, 0.0),
            ),
            Shapes::sphere(
                Vector3::new(3.0, -5.0, -17.0),
                2.8,
                Material::new(Color::from_rgb(0x77, 0xbb, 0x77), 0.5, 0.2, 2.0, 0.0),
            ),
            Shapes::sphere(
                Vector3::new(0.0, -4.0, -20.0),
                3.0,
                Material::new(Color::from_rgb(0x2f, 0x8d, 0xff), 0.6, 3.0, 50.0, 0.0),
            ),
            Shapes::sphere(
                Vector3::new(-10.0, 5.0, -20.0),
                5.0,
                Material::new(Color::new(0.1, 0.1, 0.1), 1.0, 50.0, 100.0, 1.0),
            ),
            Shapes::plane(
                Vector3::new(0.0, -8.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0).normalize(),
                Material::new(Color::from_rgb(0x66, 0x33, 0x66), 0.8, 0.2, 5.0, 0.6),
            ),
        ],
        lights: vec![
            Light::new(
                LightType::Point,
                Vector3::new(-40.0, 20.0, 20.0),
                1.0,
                Color::new(1.0, 1.0, 1.0),
            ),
            Light::new(
                LightType::Point,
                Vector3::new(40.0, 20.0, 20.0),
                0.8,
                Color::new(0.66, 0.0, 0.66),
            ),
            Light::new(
                LightType::Point,
                Vector3::new(00.0, 50.0, 0.0),
                0.8,
                Color::from_rgb(0xa6, 0x7c, 0x00),
            ),
            Light::new(
                LightType::Ambient,
                Vector3::zero(),
                0.25,
                Color::new(1.0, 1.0, 1.0),
            ),
        ],
        bg_color: Color::new(0.05, 0.05, 0.05),
    };

    let now = Instant::now();

    scene.render("result.png".to_string());

    let duration = now.elapsed();

    println!("{} milliseconds elapsed.", duration.as_secs() * 1000 + duration.subsec_millis() as u64);
}
