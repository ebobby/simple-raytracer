use super::vector::Vector3;

#[derive(Debug)]
pub struct Light {
    pub light_type: LightType,
    pub position: Vector3,
    pub intensity: f64,
}

#[derive(Debug)]
pub enum LightType {
    Ambient,
    Point,
}
