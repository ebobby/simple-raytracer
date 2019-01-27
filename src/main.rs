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
    //let scene = Scene {
    //    camera: Camera {
    //        origin: Vector3::new(0.0, 0.0, 5.0),
    //        sensor_width: 1920,
    //        sensor_height: 1080,
    //        field_of_view: std::f64::consts::PI / 3.0,
    //    },
    //    shapes: vec![
    //        Shapes::Sphere(Sphere {
    //            position: Vector3::new(0.0, 0.0, -10.0),
    //            radius: 3.0,
    //            material: Material {
    //                diffuse_color: Color::from_rgb(0x87, 0x1f, 0x78),
    //            },
    //        }),
    //        Shapes::Sphere(Sphere {
    //            position: Vector3::new(-2.0, 0.0, -6.0),
    //            radius: 1.5,
    //            material: Material {
    //                diffuse_color: Color::from_rgb(0xda, 0xa5, 0x20),
    //            },
    //        }),
    //        Shapes::Sphere(Sphere {
    //            position: Vector3::new(0.0, 0.0, -3.0),
    //            radius: 1.0,
    //            material: Material {
    //                diffuse_color: Color::from_rgb(0x2f, 0x8d, 0xff),
    //            },
    //        }),
    //        Shapes::Sphere(Sphere {
    //            position: Vector3::new(4.0, 5.0, -13.0),
    //            radius: 1.25,
    //            material: Material {
    //                diffuse_color: Color::from_rgb(0xff, 0x2f, 0x2f),
    //            },
    //        }),
    //        Shapes::Plane(Plane {
    //            position: Vector3::new(0.0, -10.0, -5.0),
    //            normal: Vector3::new(0.0, -1.0, 0.0).normalize(),
    //            material: Material {
    //                diffuse_color: Color::from_rgb(0x98, 0xfb, 0x98),
    //            },
    //        }),
    //    ],
    //    lights: vec![
    //        Light {
    //            position: Vector3::new(-2.0, 50.0, 50.0),
    //        },
    //        Light {
    //            position: Vector3::new(0.0, -9.0, 50.0),
    //        },
    //    ],
    //    bg_color: Color::new(0.0, 0.0, 0.0),
    //};

    let scene = Scene {
        camera: Camera {
            origin: Vector3::new(0.0, 0.0, 5.0),
            sensor_width: 1920,
            sensor_height: 1080,
            field_of_view: std::f64::consts::PI / 3.0,
        },
        shapes: vec![
            Shapes::Sphere(Sphere {
                position: Vector3::new(-3.0, 0.0, -16.0),
                radius: 2.0,
                material: Material {
                    diffuse_color: Color::from_rgb(0x87, 0x1f, 0x78),
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(-1.0, -1.5, -12.0),
                radius: 2.0,
                material: Material {
                    diffuse_color: Color::from_rgb(0xda, 0xa5, 0x20),
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(1.5, -0.5, -18.0),
                radius: 3.0,
                material: Material {
                    diffuse_color: Color::from_rgb(0x2f, 0x8d, 0xff),
                },
            }),
            Shapes::Sphere(Sphere {
                position: Vector3::new(7.0, 5.0, -18.0),
                radius: 4.0,
                material: Material {
                    diffuse_color: Color::from_rgb(0xff, 0x2f, 0x2f),
                },
            }),
            Shapes::Plane(Plane {
                position: Vector3::new(0.0, -10.0, -5.0),
                normal: Vector3::new(0.0, -1.0, 0.0).normalize(),
                material: Material {
                    diffuse_color: Color::from_rgb(0x98, 0xfb, 0x98),
                },
            }),
        ],
        lights: vec![
            Light {
                position: Vector3::new(-20.0, 20.0, 20.0),
                intensity: 0.5,
            },
            Light {
                position: Vector3::new(30.0, 50.0, -25.0),
                intensity: 0.8,
            },
            Light {
                position: Vector3::new(30.0, 20.0, 30.0),
                intensity: 0.9,
            },
        ],
        bg_color: Color::new(0.0, 0.0, 0.0),
    };

    scene.render("result.png".to_string());
}
