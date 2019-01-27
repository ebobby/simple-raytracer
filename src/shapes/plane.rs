use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;

use super::Intersectable;

#[derive(Debug)]
pub struct Plane {
    pub position: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let denom = self.normal.dot(&ray.direction);

        if denom > 1e-6 {
            let v = self.position - ray.origin;

            let distance = v.dot(&self.normal) / denom;

            if distance >= 0.0 {
                return Option::Some(distance);
            } else {
                return Option::None;
            }
        } else {
            return Option::None;
        }
    }

    fn normal(&self, _point: Vector3) -> Vector3 {
        -self.normal
    }
}
