mod vector;

use vector::Vector3;

pub struct Sphere {
    position: Vector3,
    radius: f64,
    color: [u8; 3]
}

pub struct Camera {
    origin: Vector3,
    sensor_width: u32,
    sensor_height: u32,
    field_of_view: f64,
}

pub enum Intersection {
    None,
    Distance(f64)
}

impl Sphere {
    pub fn intersect(&self, o: Vector3, dir: Vector3) -> Intersection {
        let voc = self.position - o;      // Vector from the origin to the sphere center
        let voc_len_sqr = voc.dot(&voc);  // The length squared of voc
        let vod_len = voc.dot(&dir);      // The length of the projected vector voc into the ray direction

        let a_sqr = voc_len_sqr - (vod_len * vod_len);  // The length squared of the line between c and the ray
        let r_sqr = self.radius * self.radius;          // Radius squared

        if a_sqr < self.radius * self.radius {    // the ray is inside the sphere

            let b = (r_sqr - a_sqr).sqrt();
            let vod_prime_len = vod_len - b;  // the distance between o and the intersection with the sphere

            return Intersection::Distance(vod_prime_len)
        }
        else {
            return Intersection::None
        }
    }
}

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

    let spheres = vec! [
        Sphere {
            position: Vector3::new(0.0, 0.0, -10.0),
            radius: 3.0,
            color: [0x99, 0x0, 0x0]
        },
        Sphere {
            position: Vector3::new(0.0, 0.0, -5.0),
            radius: 0.5,
            color: [0x0, 0x99, 0]
        },
        Sphere {
            position: Vector3::new(-2.0, 0.0, -7.0),
            radius: 1.5,
            color: [0xff, 0xd7, 0x0]
        },
    ];

    render(&camera, &spheres);
}
