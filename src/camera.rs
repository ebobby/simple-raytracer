use super::vector::Vector3;

pub struct Camera {
    pub origin: Vector3,
    pub sensor_width: u32,
    pub sensor_height: u32,
    pub field_of_view: f64,
}
