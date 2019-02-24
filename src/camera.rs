use super::ray::Ray;
use super::vector::Vec3;

#[derive(Debug)]
pub struct Camera {
    pub width: u32,       // image width
    pub height: u32,      // image height
    pub origin: Vec3,  // camera eye location
    pub look_at: Vec3, // camera looking at
    pub fov: i16,         // field of view
    pub roll: i16,        // roll angle

    w: Vec3, // camera coordinate system
    u: Vec3, // camera coordinate system
    v: Vec3, // camera coordinate system

    vp_d: f64,        // view plane distance from the eye
    aspect_ratio: f64, // aspect ratio
    vp_h: f64,         // view plane length / 2
    vp_l: f64,         // view plane length
    rate_w: f64,       // image to world width rate
    rate_h: f64,       // image to world height rate
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        fov: i16,
        roll: i16,
        width: u32,
        height: u32,
    ) -> Camera {
        let up = Vec3::new(0.0, 1.0, 0.0);
        let ra = f64::from(roll).to_radians();
        let rotated_up = Vec3::new(
            -ra.sin(),
            ra.cos(),
            0.0,
        );

        let w = (origin - look_at).normalize();
        let u = rotated_up.cross(w).normalize();
        let v = w.cross(u).normalize();

        let width_f = f64::from(width);
        let height_f = f64::from(height);
        let aspect_ratio = width_f / height_f;

        let vp_d = 1.0;
        let vp_h = vp_d * (f64::from(fov).to_radians() / 2.0).tan();
        let vp_l = vp_h * 2.0;
        let rate_w = vp_l / width_f;
        let rate_h = vp_l / height_f;

        Camera {
            // Camera in the world
            origin,
            look_at,
            fov,
            roll,

            // Image properties
            width,
            height,

            // For ray direction calculations
            w,
            u,
            v,
            aspect_ratio,
            vp_d,
            vp_h,
            vp_l,
            rate_w,
            rate_h,
        }
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let i = ((f64::from(x) + 0.5) * self.rate_w - self.vp_h) * self.aspect_ratio;
        let j = self.vp_h - ((f64::from(y) + 0.5) * self.rate_h);

        let direction = (self.u * i + self.v * j - self.w * self.vp_d).normalize();

        Ray {
            origin: self.origin,
            direction,
        }
    }
}
