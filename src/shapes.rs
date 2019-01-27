pub mod plane;
pub mod sphere;

use crate::ray::Ray;
use crate::vector::Vector3;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn normal(&self, hit_point: Vector3) -> Vector3;
}

#[derive(Debug)]
pub enum Shapes {
    Sphere(sphere::Sphere),
    Plane(plane::Plane),
}

impl Shapes {
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Shapes::Sphere(s) => s.intersect(ray),
            Shapes::Plane(p) => p.intersect(ray)
        }
    }

    pub fn normal(&self, point: Vector3) -> Vector3 {
        match self {
            Shapes::Sphere(s) => s.normal(point),
            Shapes::Plane(p) => p.normal(point)
        }
    }

    pub fn color(&self) -> [u8; 3] {
        match self {
            Shapes::Sphere(s) => s.color.clone(),
            Shapes::Plane(p) => p.color.clone(),
        }
    }
}
