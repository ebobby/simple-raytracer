use super::vector::Vector3;

#[derive(Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub fov: u16,

    pub origin: Vector3,
    pub d: f64,
}
