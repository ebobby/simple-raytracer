use super::ray::Ray;
use super::vector::Vector3;

pub enum Intersection {
    None,
    Distance(f64),
}

pub struct Sphere {
    pub position: Vector3,
    pub radius: f64,
    pub color: [u8; 3],
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> Intersection {
        let voc = self.position - ray.origin; // Vector from the origin to the sphere center
        let voc_len_sqr = voc.dot(&voc); // The length squared of voc
        let vod_len = voc.dot(&ray.direction); // The length of the projected vector voc into the ray direction

        let a_sqr = voc_len_sqr - (vod_len * vod_len); // The length squared of the line between c and the ray
        let r_sqr = self.radius * self.radius; // Radius squared

        if a_sqr <= self.radius * self.radius {
            // the ray is inside the sphere

            let b = (r_sqr - a_sqr).sqrt();
            // the distance between o and the intersection with the sphere

            let vod_prime_len = if vod_len - b < 0.0 {
                vod_len + b
            }
            else {
                vod_len - b
            };

            return Intersection::Distance(vod_prime_len);
        } else {
            return Intersection::None;
        }
    }
}
