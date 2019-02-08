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

        let mut diffuse_light = Color::new(0.0, 0.0, 0.0);
        let mut specular_light = Color::new(0.0, 0.0, 0.0);

        for light in lights {
            match light.light_type {
                LightType::Ambient => diffuse_light = diffuse_light + light.color * light.intensity,
                LightType::Point => {
                    let light_dir = (light.position - intersection.hit_point).normalize();
                    let cos_angle = light_dir.dot(&intersection.normal).max(0.0);

                    let light_ray = Ray {
                        origin: if light_dir.dot(&intersection.normal) < 0.0 {
                            intersection.hit_point.correct(&-intersection.normal)
                        } else {
                            intersection.hit_point.correct(&intersection.normal)
                        },
                        direction: light_dir,
                    };

                    match Ray::intersect(&light_ray, objects) {
                        Option::None => {
                            diffuse_light =
                                diffuse_light + light.color * (light.intensity * cos_angle);

                            specular_light = specular_light
                                + light.color
                                    * (-(-light_dir).reflect(&intersection.normal).dot(&direction))
                                        .max(0.0)
                                        .powf(mat.specular_exponent);
                        }
                        Option::Some(_) => (),
                    }
                }
            }
        }

        mat.color * ((diffuse_light * mat.diffuse) + (specular_light * mat.specular))
    }
}
