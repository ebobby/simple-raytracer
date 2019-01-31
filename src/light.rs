use super::material::Color;
use super::ray::Intersection;
use super::ray::Ray;
use super::shapes::Shapes;
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

impl Light {
    pub fn shade(
        objects: &Vec<Shapes>,
        lights: &Vec<Light>,
        intersection: &Intersection,
        direction: Vector3,
    ) -> Color {
        let mut light_amount = 0.0;
        let mut specular_amount = 0.0;
        let mat = intersection.material;

        for light in lights {
            match light.light_type {
                LightType::Ambient => light_amount += light.intensity,
                LightType::Point => {
                    let light_dir = (light.position - intersection.hit_point).normalize();
                    let cos_angle = light_dir.dot(&intersection.normal).max(0.0);

                    let light_ray = Ray {
                        origin: if light_dir.dot(&intersection.normal) < 0.0 {
                            intersection.hit_point - (intersection.normal * 1e-2)
                        } else {
                            intersection.hit_point + (intersection.normal * 1e-2)
                        },
                        direction: light_dir,
                    };

                    match Ray::intersect(&light_ray, objects) {
                        Option::None => {
                            light_amount += light.intensity * cos_angle;

                            specular_amount +=
                                (-(-light_dir).reflect(&intersection.normal).dot(&direction))
                                    .max(0.0)
                                    .powf(mat.specular_exponent);
                        }
                        Option::Some(_) => (),
                    }
                }
            }
        }

        mat.color * (light_amount * mat.diffuse + specular_amount * mat.specular)
    }
}
