use super::Intersectable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray) -> Option<f64> {

        let oc = ray.origin - self.position;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        let mut temp: f64;

        if discriminant > 0.0 {
            temp = (-b - discriminant.sqrt()) / a;

            if temp < std::f64::INFINITY && temp > crate::EPSILON {
                return Some(temp)
            }
        }

        temp = (-b + discriminant.sqrt()) / a;

        if temp < std::f64::INFINITY && temp > crate::EPSILON {
            return Some(temp)
        }

        None
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, point: Vec3) -> Vec3 {
        (point - self.position).normalize()
    }
}
