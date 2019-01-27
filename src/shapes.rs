use super::Ray;

pub enum Intersection {
    None,
    Distance(f64),
}

trait Shape {
    fn intersect(&self, ray: &Ray) -> Intersection;
}

pub mod sphere;
