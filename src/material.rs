extern crate image;

use super::vector::Vector3;

pub type Color = Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    pub diffuse: f64,
    pub specular: f64,
    pub specular_exponent: f64,
    pub reflectiveness: f64,
}

impl Material {
    pub fn new(
        color: Color,
        diffuse: f64,
        specular: f64,
        specular_exponent: f64,
        reflectiveness: f64,
    ) -> Material {
        Material {
            color,
            diffuse,
            specular,
            specular_exponent,
            reflectiveness,
        }
    }

    pub fn neutral() -> Material {
        Material::new(Color::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0)
    }
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::new(
            f64::from(r) / 255.0,
            f64::from(g) / 255.0,
            f64::from(b) / 255.0,
        )
    }

    pub fn to_rgb(&self, gamma_correction: f64) -> image::Rgb<u8> {
        image::Rgb([
            (self.x.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
            (self.y.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
            (self.z.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
        ])
    }
}
