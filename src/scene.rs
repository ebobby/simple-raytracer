use super::camera::Camera;
use super::color::Color;
use super::intersectable::Intersectable;
use super::light::Light;
use super::options::Options;
use super::ray::Ray;

#[derive(Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,

    pub camera: Camera,
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Light>,
    pub bg_color: Color,
    pub options: Options,
}

impl Scene {
    pub fn render(&self, filename: String) {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);

        let gamma_correction = self.options.gamma.recip();

        let w = f64::from(self.width);
        let h = f64::from(self.height);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let u = f64::from(x) / w;
            let v = f64::from(y) / h;

            let ray = self.camera.get_ray(u, v);

            let color = Ray::cast_ray(ray, &self.objects, &self.lights, &self.options, 0)
                .unwrap_or(self.bg_color);

            *pixel = color.gamma_rgb(gamma_correction);
        }

        imgbuf.save(filename).unwrap();
    }
}
