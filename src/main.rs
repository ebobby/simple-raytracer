mod camera;
mod light;
mod material;
mod ray;
mod shapes;
mod vector;

use camera::Camera;
use light::Light;
use material::Color;
use material::Material;
use ray::Ray;
use shapes::plane::Plane;
use shapes::sphere::Sphere;
use shapes::Shapes;
use vector::Vector3;

fn render(c: &Camera, shapes: &Vec<Shapes>, light: &Light, bg_color: Color) {
    let mut imgbuf = image::ImageBuffer::new(c.sensor_width, c.sensor_height);

    let aspect_ratio_adjustment = c.sensor_width as f64 / c.sensor_height as f64;
    let fov_adjustment = (c.field_of_view / 2.0).tan();

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = ((((x as f64 + 0.5) / c.sensor_width as f64) * 2.0) - 1.0)
            * fov_adjustment
            * aspect_ratio_adjustment;
        let j = (1.0 - (((y as f64 + 0.5) / c.sensor_height as f64) * 2.0)) * fov_adjustment;

        let ray = Ray {
            origin: c.origin,
            direction: Vector3::new(i, j, -1.0).normalize(),
        };

        // casting ray
        let color = match Ray::cast_ray(&ray, shapes) {
            Option::Some(int) => {
                let light_dir = (light.position - int.hit_point).normalize();
                let cos_angle = 0.0f64.max(light_dir.dot(&int.normal));

                let light_ray = Ray {
                    origin: if light_dir.dot(&int.normal) < 0.0 {
                        int.hit_point - (int.normal * 1e-2)
                    } else {
                        int.hit_point + (int.normal * 1e-2)
                    },
                    direction: light_dir,
                };

                let mat = int.material;

                match Ray::cast_ray(&light_ray, shapes) {
                    Option::None => mat.diffuse_color * cos_angle,
                    Option::Some(_) => bg_color,
                }
            }
            Option::None => bg_color,
        };

        *pixel = color.to_rgb();
    }

    imgbuf.save("result.png").unwrap();
}

fn main() {
    let camera = Camera {
        origin: Vector3::new(0.0, 0.0, 5.0),
        sensor_width: 1920,
        sensor_height: 1080,
        field_of_view: std::f64::consts::PI / 3.0,
    };

    let shapes: Vec<Shapes> = vec![
        Shapes::Sphere(Sphere {
            position: Vector3::new(0.0, 0.0, -10.0),
            radius: 3.0,
            material: Material {
                diffuse_color: Color::from_rgb(0x87, 0x1f, 0x78),
            },
        }),
        Shapes::Sphere(Sphere {
            position: Vector3::new(-2.0, 0.0, -6.0),
            radius: 1.5,
            material: Material {
                diffuse_color: Color::from_rgb(0xda, 0xa5, 0x20),
            },
        }),
        Shapes::Sphere(Sphere {
            position: Vector3::new(0.0, 0.0, -3.0),
            radius: 1.0,
            material: Material {
                diffuse_color: Color::from_rgb(0x2f, 0x8d, 0xff),
            },
        }),
        Shapes::Sphere(Sphere {
            position: Vector3::new(4.0, 5.0, -13.0),
            radius: 1.25,
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
    ];

    let light = Light {
        position: Vector3::new(-2.0, 50.0, 50.0),
    };

    let background_color = Color::new(0.0, 0.0, 0.0);

    render(&camera, &shapes, &light, background_color);
}
