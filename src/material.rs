use super::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    pub diffuse: f64,
    pub specular: f64,
    pub specular_exponent: f64,
    pub reflectiveness: f64,
}

impl Material {
    pub fn neutral() -> Material {
        Material {
            color: Color::black(),
            diffuse: 0.0,
            specular: 0.0,
            specular_exponent: 0.0,
            reflectiveness: 0.0,
        }
    }
}
