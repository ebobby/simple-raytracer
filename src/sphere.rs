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
    pub fn intersect(&self, o: Vector3, dir: Vector3) -> Intersection {
        let voc = self.position - o; // Vector from the origin to the sphere center
        let voc_len_sqr = voc.dot(&voc); // The length squared of voc
        let vod_len = voc.dot(&dir); // The length of the projected vector voc into the ray direction

        let a_sqr = voc_len_sqr - (vod_len * vod_len); // The length squared of the line between c and the ray
        let r_sqr = self.radius * self.radius; // Radius squared

        if a_sqr < self.radius * self.radius {
            // the ray is inside the sphere

            let b = (r_sqr - a_sqr).sqrt();
            let vod_prime_len = vod_len - b; // the distance between o and the intersection with the sphere

            return Intersection::Distance(vod_prime_len);
        } else {
            return Intersection::None;
        }
    }
}
