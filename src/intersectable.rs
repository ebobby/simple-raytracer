pub mod plane;
pub mod sphere;

use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

use std::fmt::Debug;

pub trait Intersectable: Debug {
    fn intersect(&self, ray: Ray) -> Option<f64>;
    fn material(&self) -> Material;
    fn normal(&self, hit_point: Vec3) -> Vec3;
}
