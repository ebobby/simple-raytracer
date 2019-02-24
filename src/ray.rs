use super::color::Color;
use super::intersectable::Intersectable;
use super::light::Light;
use super::material::Material;
use super::options::Options;
use super::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub distance: f64,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Ray {
    pub fn intersect(ray: Ray, objects: &[Box<dyn Intersectable>]) -> Option<Intersection> {
        let mut distance = std::f64::INFINITY;
        let mut material = Material::neutral();
        let mut normal = Vec3::zero();
        let mut hit_point = Vec3::zero();

        for shape in objects {
            if let Some(dist) = shape.intersect(ray) {
                if dist < distance {
                    distance = dist;
                    material = shape.material();

                    hit_point = ray.origin + (ray.direction * distance);
                    normal = shape.normal(hit_point);
                }
            }
        }

        if distance < std::f64::INFINITY {
            Some(Intersection {
                distance,
                hit_point,
                normal,
                material,
            })
        } else {
            None
        }
    }

    pub fn cast_ray(
        ray: Ray,
        objects: &[Box<dyn Intersectable>],
        lights: &[Light],
        options: &Options,
        depth: u8,
    ) -> Option<Color> {
        if depth >= options.max_rays {
            return None;
        }

        let intersection = Ray::intersect(ray, objects)?;

        let mut shaded_color = Light::shade(objects, lights, options, intersection, ray.direction);

        if intersection.material.reflectiveness > 0.0 && options.reflections {
            let reflection = ray.direction.reflect(intersection.normal).normalize();

            let reflected_ray = Ray {
                origin: intersection.hit_point.correct(intersection.normal),
                direction: reflection,
            };

            if let Some(reflected_color) =
                Ray::cast_ray(reflected_ray, objects, lights, options, depth + 1)
            {
                shaded_color += reflected_color * intersection.material.reflectiveness;
            }
        }

        Some(shaded_color)
    }
}
