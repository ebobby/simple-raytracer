use super::color::Color;
use super::intersectable::Intersectable;
use super::options::Options;
use super::ray::Intersection;
use super::ray::Ray;
use super::vector::Vec3;

#[derive(Debug)]
pub struct Light {
    pub light_type: LightType,
    pub position: Vec3,
    pub intensity: f64,
    pub color: Color,
}

#[derive(Debug)]
pub enum LightType {
    Ambient,
    Point,
}

impl Light {
    pub fn shade(
        objects: &[Box<dyn Intersectable>],
        lights: &[Light],
        options: &Options,
        intersection: Intersection,
        direction: Vec3,
    ) -> Color {
        let mat = intersection.material;

        let mut diff_light = Color::black();
        let mut spec_light = Color::black();

        for light in lights {
            match light.light_type {
                LightType::Ambient => diff_light += light.color * light.intensity,
                LightType::Point => {
                    let light_vec = light.position - intersection.hit_point;
                    let light_dir = light_vec.normalize();
                    let light_dis = light_vec.length();
                    let light_angle = light_dir.dot(intersection.normal);

                    let light_ray = Ray {
                        origin: if light_angle < 0.0 {
                            intersection.hit_point.correct(-intersection.normal)
                        } else {
                            intersection.hit_point.correct(intersection.normal)
                        },
                        direction: light_dir,
                    };

                    // This point gets hit by this light if there are no objects between us
                    // or the object is farther away than the light.
                    let hit_by_light = match Ray::intersect(light_ray, objects) {
                        None => true,
                        Some(intersection) => light_dis <= intersection.distance,
                    };

                    if hit_by_light || !options.shadows {
                        let light_reflection = (-light_dir).reflect(intersection.normal);
                        let angle = -(light_reflection.dot(direction));

                        diff_light += light.color * (light.intensity * light_angle.max(0.0));
                        spec_light += light.color * angle.max(0.0).powf(mat.specular_exponent);
                    }
                }
            }
        }

        let mut factor = if !(options.diffuse | options.specular) {
            Color::white()
        } else {
            Color::black()
        };

        if options.diffuse {
            factor += diff_light * mat.diffuse;
        }

        if options.specular {
            factor += spec_light * mat.specular;
        }

        mat.color * factor
    }
}
