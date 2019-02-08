pub mod plane;
pub mod sphere;

use crate::material::Material;
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
    pub fn sphere(position: Vector3, radius: f64, material: Material) -> Shapes {
        Shapes::Sphere(sphere::Sphere {
            position,
            radius,
            material,
        })
    }

    pub fn plane(position: Vector3, normal: Vector3, material: Material) -> Shapes {
        Shapes::Plane(plane::Plane {
            position,
            normal,
            material,
        })
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Shapes::Sphere(s) => s.intersect(ray),
            Shapes::Plane(p) => p.intersect(ray),
        }
    }

    pub fn normal(&self, point: Vector3) -> Vector3 {
        match self {
            Shapes::Sphere(s) => s.normal(point),
            Shapes::Plane(p) => p.normal(point),
        }
    }

    pub fn material(&self) -> Material {
        match self {
            Shapes::Sphere(s) => s.material,
            Shapes::Plane(p) => p.material,
        }
    }
}
