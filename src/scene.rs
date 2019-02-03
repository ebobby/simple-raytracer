use super::camera::Camera;
use super::light::Light;
use super::material::Color;
use super::ray::Ray;
use super::shapes::Shapes;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Shapes>,
    pub lights: Vec<Light>,
    pub bg_color: Color,
}

impl Scene {
    pub fn render(&self, filename: String) {
        let mut imgbuf = image::ImageBuffer::new(self.camera.width, self.camera.height);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let ray = self.camera.get_ray(x, y);

            let color = match Ray::cast_ray(&ray, &self.objects, &self.lights, 0) {
                Option::Some(color) => color,
                Option::None => self.bg_color,
            };

            *pixel = color.to_rgb();
        }

        imgbuf.save(filename).unwrap();
    }
}
