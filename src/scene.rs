use super::camera::Camera;
use super::color::Color;
use super::intersectable::Intersectable;
use super::light::Light;
use super::ray::Ray;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Light>,
    pub bg_color: Color,
}

impl Scene {
    pub fn render(&self, filename: String) {
        let mut imgbuf = image::ImageBuffer::new(self.camera.width, self.camera.height);

        let gamma_correction = crate::OPTIONS.gamma.recip();

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let ray = self.camera.get_ray(x, y);

            let color = Ray::cast_ray(ray, &self.objects, &self.lights, 0).unwrap_or(self.bg_color);

            *pixel = color.gamma_rgb(gamma_correction);
        }

        imgbuf.save(filename).unwrap();
    }
}
