use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color::new(0., 0., 0.)
    }

    pub fn white() -> Color {
        Color::new(1., 1., 1.)
    }

    pub fn from_u8(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: f64::from(r) / 255.0,
            g: f64::from(g) / 255.0,
            b: f64::from(b) / 255.0,
        }
    }

    pub fn gamma_rgb(&self, gamma_correction: f64) -> image::Rgb<u8> {
        image::Rgb([
            (self.r.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
            (self.g.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
            (self.b.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
        ])
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, factor: f64) -> Color {
        Color {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
