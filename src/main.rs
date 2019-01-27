mod camera;
mod light;
mod ray;
mod shapes;
mod vector;

use camera::Camera;
use light::Light;
use ray::Ray;
use shapes::sphere::Sphere;
use shapes::Intersection;
use vector::Vector3;

fn render(c: &Camera, spheres: &Vec<Sphere>, light: &Light) {
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
        let mut distance = std::f64::INFINITY;
        let mut color = [0x0, 0x0, 0x0];
        let mut normal = Vector3::zero();
        let mut hit = Vector3::zero();

        for s in spheres {
            match s.intersect(&ray) {
                Intersection::None => (),
                Intersection::Distance(dist) => {
                    if dist < distance {
                        distance = dist;
                        color = s.color.clone();

                        hit = ray.origin + (ray.direction * distance);
                        normal = (hit - s.position).normalize();
                    }
                }
            }
        }

        if hit != Vector3::zero() {
            let light_dir = (light.position - hit).normalize();
            let cos_angle = 0.0f64.max(light_dir.dot(&normal));

            let light_intensity = cos_angle;

            color[0] = (color[0] as f64 * light_intensity) as u8;
            color[1] = (color[1] as f64 * light_intensity) as u8;
            color[2] = (color[2] as f64 * light_intensity) as u8;
        }

        *pixel = image::Rgb(color);
    }

    imgbuf.save("result.png").unwrap();
}

fn main() {
    let camera = Camera {
        origin: Vector3::zero(),
        sensor_width: 1920,
        sensor_height: 1080,
        field_of_view: std::f64::consts::PI / 3.0,
    };

    let spheres = vec![
        Sphere {
            position: Vector3::new(0.0, 0.0, -10.0),
            radius: 3.0,
            color: [0x99, 0x0, 0x99],
        },
        Sphere {
            position: Vector3::new(-2.0, 0.0, -6.0),
            radius: 1.5,
            color: [0xff, 0xd7, 0x0],
        },
        Sphere {
            position: Vector3::new(0.0, 0.0, -3.0),
            radius: 0.5,
            color: [0x0, 0x99, 0],
        },
    ];

    let light =      Light {
        position: Vector3::new(0.0, 5.0, 0.0),
    };

    render(&camera, &spheres, &light);
}
