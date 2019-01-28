mod camera;
mod light;
mod material;
mod ray;
mod scene;
mod shapes;
mod vector;

use camera::Camera;
use light::Light;
use material::Color;
use material::Material;
use scene::Scene;
use shapes::plane::Plane;
use shapes::sphere::Sphere;
use shapes::Shapes;
use vector::Vector3;

fn main() {
    let scene = Scene {
        camera: Camera {
            origin: Vector3::new(0.0, 0.0, 5.0),
            sensor_width: 1920,
            sensor_height: 1080,
            field_of_view: std::f64::consts::PI / 3.0,
        },
        shapes: vec![
            Shapes::Sphere(Sphere {
                position: Vector3::new(-3.0, -5.0, -16.0),
                radius: 3.1,
                material: Material {
                    diffuse_color: Color::from_rgb(0xbb, 0x77, 0x77),
                    specular_exponent: 10.0,
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(3.0, -5.0, -17.0),
                radius: 3.0,
                material: Material {
                    diffuse_color: Color::from_rgb(0x77, 0xbb, 0x77),
                    specular_exponent: 50.0,
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(0.0, -4.0, -20.0),
                radius: 3.0,
                material: Material {
                    diffuse_color: Color::from_rgb(0x2f, 0x8d, 0xff),
                    specular_exponent: 20.0,
                },
            }),
            Shapes::Plane(Plane {
                position: Vector3::new(0.0, -8.0, 0.0),
                normal: Vector3::new(0.0, -1.0, 0.0).normalize(),
                material: Material {
                    diffuse_color: Color::from_rgb(0xbb, 0xbb, 0xbb),
                    specular_exponent: 5.0,
                },
            }),
        ],
        lights: vec![
            Light {
                position: Vector3::new(0.0, 50.0, -50.0),
                intensity: 0.75,
            },
            Light {
                position: Vector3::new(-40.0, 20.0, 20.0),
                intensity: 0.75,
            },
        ],
        bg_color: Color::new(0.05, 0.05, 0.05),
        ambient_light: 0.20,
    };

    scene.render("result.png".to_string());
}
