use super::light::Light;
use super::material::Color;
use super::material::Material;
use super::shapes::Shapes;
use super::vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug)]
pub struct Intersection {
    pub distance: f64,
    pub hit_point: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn intersect(ray: &Ray, objects: &[Shapes]) -> Option<Intersection> {
        let mut distance = std::f64::INFINITY;
        let mut material = Material::neutral();
        let mut normal = Vector3::zero();
        let mut hit_point = Vector3::zero();

        for shape in objects {
            match shape.intersect(&ray) {
                Option::Some(dist) => {
                    if dist < distance {
                        distance = dist;
                        material = shape.material();

                        hit_point = ray.origin + (ray.direction * distance);
                        normal = shape.normal(hit_point);
                    }
                }
                Option::None => (),
            }
        }

        if distance < std::f64::INFINITY {
            Option::Some(Intersection {
                distance,
                hit_point,
                normal,
                material,
            })
        } else {
            Option::None
        }
    }

    pub fn cast_ray(ray: &Ray, objects: &[Shapes], lights: &[Light], depth: u8) -> Option<Color> {
        if depth >= crate::OPTIONS.max_rays {
            return Option::None;
        }

        match Ray::intersect(ray, objects) {
            Option::Some(intersection) => {
                let mut shaded_color = Light::shade(objects, lights, &intersection, ray.direction);

                if intersection.material.reflectiveness > 0.0 {
                    let reflection = ray.direction.reflect(&intersection.normal).normalize();

                    let reflected_ray = Ray::new(
                        intersection.hit_point.correct(&intersection.normal),
                        reflection,
                    );

                    match Ray::cast_ray(&reflected_ray, objects, lights, depth + 1) {
                        Option::Some(reflected_color) => {
                            shaded_color = shaded_color
                                + reflected_color * intersection.material.reflectiveness;
                        }
                        Option::None => (),
                    }
                }

                Option::Some(shaded_color)
            }
            Option::None => Option::None,
        }
    }
}
