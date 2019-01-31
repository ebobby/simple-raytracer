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
use shapes::plane::Plane;
use shapes::sphere::Sphere;
use shapes::Shapes;
use vector::Vector3;

fn main() {
    let scene = Scene {
        camera: Camera {
            origin: Vector3::new(0.0, 3.0, 15.0),
            sensor_width: 1920,
            sensor_height: 1080,
            field_of_view: std::f64::consts::PI / 3.0,
        },
        objects: vec![
            Shapes::Sphere(Sphere {
                position: Vector3::new(-3.0, -5.0, -16.0),
                radius: 3.1,
                material: Material {

                    color: Color::from_rgb(0xcc, 0x33, 0x33),
                    diffuse: 0.8,
                    specular: 2.5,
                    specular_exponent: 30.0,
                    reflectiveness: 0.0,
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(3.0, -5.0, -17.0),
                radius: 3.0,
                material: Material {
                    color: Color::from_rgb(0x77, 0xbb, 0x77),
                    diffuse: 0.8,
                    specular: 2.0,
                    specular_exponent: 50.0,
                    reflectiveness: 0.0,
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(0.0, -4.0, -20.0),
                radius: 3.0,
                material: Material {
                    color: Color::from_rgb(0x2f, 0x8d, 0xff),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 20.0,
                    reflectiveness: 0.0,
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(5.0, 4.0, -20.0),
                radius: 5.0,
                material: Material {
                    color: Color::new(0.1, 0.1, 0.1),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 50.0,
                    reflectiveness: 1.0,
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(-10.0, 10.0, -25.0),
                radius: 10.0,
                material: Material {
                    color: Color::new(0.1, 0.1, 0.1),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 2.0,
                    reflectiveness: 1.0,
                },
            }),
            Shapes::Plane(Plane {
                position: Vector3::new(0.0, -9.0, 0.0),
                normal: Vector3::new(0.0, -1.0, 0.0).normalize(),
                material: Material {
                    color: Color::from_rgb(0x66, 0x33, 0x66),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 5.0,
                    reflectiveness: 0.6,
                },
            }),
        ],
        lights: vec![
            Light {
                position: Vector3::new(-40.0, 20.0, 20.0),
                intensity: 1.0,
                light_type: LightType::Point,
            },
            Light {
                position: Vector3::new(40.0, 20.0, 20.0),
                intensity: 1.0,
                light_type: LightType::Point,
            },
            Light {
                position: Vector3::zero(),
                intensity: 0.25,
                light_type: LightType::Ambient,
            },
        ],
        bg_color: Color::new(0.05, 0.05, 0.05),
    };

    scene.render("result.png".to_string());
}
