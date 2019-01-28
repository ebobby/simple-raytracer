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
                    let light_int = Light::calculate_intensity(
                        &self.shapes,
                        &self.lights,
                        &intersection,
                        ray.direction,
                    );

                    intersection.material.diffuse_color * light_int
                }
                Option::None => self.bg_color,
            };

            *pixel = color.to_rgb();
        }

        imgbuf.save(filename).unwrap();
    }
}
