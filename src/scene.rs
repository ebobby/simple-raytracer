use super::camera::Camera;
use super::light::Light;
use super::material::Color;
use super::ray::Ray;
use super::shapes::Shapes;
use super::vector::Vector3;

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

        let width_f = f64::from(self.camera.width);
        let height_f = f64::from(self.camera.height);
        let aspect_ratio = width_f / height_f;

        let vp_h = self.camera.d * (f64::from(self.camera.fov).to_radians() / 2.0).tan();
        let vp_l = vp_h * 2.0;
        let rate_w = vp_l / width_f;
        let rate_h = vp_l / height_f;

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let i = ((f64::from(x) + 0.5) * rate_w - vp_h) * aspect_ratio;
            let j = vp_h - ((f64::from(y) + 0.5) * rate_h);

            let ray = Ray {
                origin: self.camera.origin,
                direction: Vector3::new(i, j, -1.0).normalize(),
            };

            // casting ray
            let color = match Ray::cast_ray(&ray, &self.objects, &self.lights, 0) {
                Option::Some(color) => color,
                Option::None => self.bg_color,
            };

            *pixel = color.to_rgb();
        }

        imgbuf.save(filename).unwrap();
    }
}
