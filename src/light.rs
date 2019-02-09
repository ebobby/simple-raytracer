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
    pub color: Color,
}

#[derive(Debug)]
pub enum LightType {
    Ambient,
    Point,
}

impl Light {
    pub fn new(light_type: LightType, position: Vector3, intensity: f64, color: Color) -> Light {
        Light {
            light_type,
            position,
            intensity,
            color,
        }
    }

    pub fn shade(
        objects: &[Shapes],
        lights: &[Light],
        intersection: &Intersection,
        direction: Vector3,
    ) -> Color {
        let mat = intersection.material;

        let mut diff_light = Color::new(0.0, 0.0, 0.0);
        let mut spec_light = Color::new(0.0, 0.0, 0.0);

        for light in lights {
            match light.light_type {
                LightType::Ambient => diff_light = diff_light + light.color * light.intensity,
                LightType::Point => {
                    let light_vec = light.position - intersection.hit_point;
                    let light_dir = light_vec.normalize();
                    let light_dis = light_vec.length();
                    let light_angle = light_dir.dot(&intersection.normal);

                    let light_ray = Ray {
                        origin: if light_angle < 0.0 {
                            intersection.hit_point.correct(&-intersection.normal)
                        } else {
                            intersection.hit_point.correct(&intersection.normal)
                        },
                        direction: light_dir,
                    };

                    // This point gets hit by this light if there are no objects between us
                    // or the object is farther away than the light.
                    let hit_by_light = match Ray::intersect(&light_ray, objects) {
                        Option::None => true,
                        Option::Some(intersection) => light_dis <= intersection.distance,
                    };

                    if hit_by_light || !crate::OPTIONS.shadows {
                        let light_reflection = (-light_dir).reflect(&intersection.normal);
                        let reflection_angle = -(light_reflection.dot(&direction));

                        diff_light =
                            diff_light + light.color * (light.intensity * light_angle.max(0.0));

                        spec_light = spec_light
                            + light.color * reflection_angle.max(0.0).powf(mat.specular_exponent);
                    }
                }
            }
        }

        let mut factor = if crate::OPTIONS.diffuse | crate::OPTIONS.specular == false {
            Vector3::new(1.0, 1.0, 1.0)
        } else {
            Vector3::zero()
        };

        if crate::OPTIONS.diffuse {
            factor = factor + (diff_light * mat.diffuse);
        }

        if crate::OPTIONS.specular {
            factor = factor + (spec_light * mat.specular);
        }

        mat.color * factor
    }
}
