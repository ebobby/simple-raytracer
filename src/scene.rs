use super::camera::Camera;
use super::light::Light;
use super::material::Color;
use super::ray::Ray;
use super::shapes::Shapes;
use super::vector::Vector3;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Shapes>,
    pub lights: Vec<Light>,
    pub bg_color: Color,
}

impl Scene {
    pub fn render(&self, filename: String) {
        let mut imgbuf =
            image::ImageBuffer::new(self.camera.sensor_width, self.camera.sensor_height);
        let aspect_ratio_adjustment =
            self.camera.sensor_width as f64 / self.camera.sensor_height as f64;
        let fov_adjustment = (self.camera.field_of_view / 2.0).tan();

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let i = ((((x as f64 + 0.5) / (self.camera.sensor_width - 1) as f64) * 2.0) - 1.0)
                * fov_adjustment
                * aspect_ratio_adjustment;
            let j = (1.0 - (((y as f64 + 0.5) / (self.camera.sensor_height - 1) as f64) * 2.0))
                * fov_adjustment;

            let ray = Ray {
                origin: self.camera.origin,
                direction: Vector3::new(i, j, -1.0).normalize(),
            };

            // casting ray
            let color = match Ray::cast_ray(&ray, &self.shapes) {
                Option::Some(intersection) => {
                    let mut diffuse_light_intensity = 0.0f64;

                    for light in &self.lights {
                        // Calculate light direction and angle
                        let light_dir = (light.position - intersection.hit_point).normalize();
                        let cos_angle = 0.0f64.max(light_dir.dot(&intersection.normal));

                        let light_ray = Ray {
                            origin: if light_dir.dot(&intersection.normal) < 0.0 {
                                intersection.hit_point - (intersection.normal * 1e-2)
                            } else {
                                intersection.hit_point + (intersection.normal * 1e-2)
                            },
                            direction: light_dir,
                        };

                        match Ray::cast_ray(&light_ray, &self.shapes) {
                            Option::None => diffuse_light_intensity += light.intensity * cos_angle,
                            Option::Some(_) => (),
                        }
                    }

                    let mat = intersection.material;

                    mat.diffuse_color * diffuse_light_intensity
                }
                Option::None => self.bg_color,
            };

            *pixel = color.to_rgb();
        }

        imgbuf.save(filename).unwrap();
    }
}
