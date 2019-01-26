mod camera;
mod sphere;
mod vector;

use camera::Camera;
use sphere::{Intersection, Sphere};
use vector::Vector3;

fn render(c: &Camera, spheres: &Vec<Sphere>) {
    let mut imgbuf = image::ImageBuffer::new(c.sensor_width, c.sensor_height);

    let aspect_ratio_adjustment = c.sensor_width as f64 / c.sensor_height as f64;
    let fov_adjustment = (c.field_of_view / 2.0).tan();

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = ((((x as f64 + 0.5) / c.sensor_width as f64) * 2.0) - 1.0)
            * fov_adjustment
            * aspect_ratio_adjustment;
        let j = (1.0 - (((y as f64 + 0.5) / c.sensor_height as f64) * 2.0)) * fov_adjustment;

        let ray = Vector3::new(i, j, -1.0).normalize();

        // casting ray
        let mut color = image::Rgb([0x11, 0x33, 0x66]);
        let mut distance = std::f64::INFINITY;

        for s in spheres {
            match s.intersect(c.origin, ray) {
                Intersection::None => (),
                Intersection::Distance(dist) => {
                    if dist < distance {
                        distance = dist;
                        color = image::Rgb(s.color);
                    }
                }
            }
        }

        *pixel = color;
    }

    imgbuf.save("result.png").unwrap();
}

fn main() {
    let camera = Camera {
        origin: Vector3::zero(),
        sensor_width: 1024,
        sensor_height: 768,
        field_of_view: std::f64::consts::PI / 3.0,
    };

    let spheres = vec![
        Sphere {
            position: Vector3::new(0.0, 0.0, -10.0),
            radius: 3.0,
            color: [0x99, 0x0, 0x0],
        },
        Sphere {
            position: Vector3::new(0.0, 0.0, -5.0),
            radius: 0.5,
            color: [0x0, 0x99, 0],
        },
        Sphere {
            position: Vector3::new(-2.0, 0.0, -7.0),
            radius: 1.5,
            color: [0xff, 0xd7, 0x0],
        },
    ];

    render(&camera, &spheres);
}
