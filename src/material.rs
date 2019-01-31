extern crate image;

use super::vector::Vector3;
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub color: Vector3,
}

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
            color: Color::new(0.0, 0.0, 0.0),
            diffuse: 0.0,
            specular: 0.0,
            specular_exponent: 0.0,
            reflectiveness: 0.0,
        }
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            color: Vector3::new(r, g, b),
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            color: Vector3::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0),
        }
    }

    pub fn to_rgb(&self) -> image::Rgb<u8> {
        image::Rgb([
            (self.color.x.min(1.0).max(0.0) * 255.0) as u8,
            (self.color.y.min(1.0).max(0.0) * 255.0) as u8,
            (self.color.z.min(1.0).max(0.0) * 255.0) as u8,
        ])
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, factor: f64) -> Color {
        Color::new(
            self.color.x * factor,
            self.color.y * factor,
            self.color.z * factor,
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Color {
        Color::new(
            self.color.x + rhs.color.x,
            self.color.y + rhs.color.y,
            self.color.z + rhs.color.z,
        )
    }
}
