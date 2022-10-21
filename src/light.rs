#[cfg(test)]
use approx::assert_relative_eq;
use num::pow;
use rapier3d::na::{Point3, Vector3};

use crate::{
    intersections::*, materials::Material, pattern::Pattern, ray_rgb::RayRgb, sphere::Sphere,
};

pub struct PointLight {
    pub color: RayRgb,
    pub position: Point3<f32>,
    pub intensity: usize,
}

impl PointLight {
    pub fn new(color: RayRgb, position: Point3<f32>) -> Self {
        Self {
            color,
            position,
            intensity: 1,
        }
    }
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: Point3<f32>,
    eyev: Vector3<f32>,
    normalv: Vector3<f32>,
    in_shadow: bool,
    object: &Sphere,
) -> RayRgb {
    let effective_color;
    if material.pattern.is_some() {
        effective_color =
            stripe_at_object(&material.pattern.as_ref().unwrap(), &object, point) * light.intensity;
    } else {
        effective_color = material.color * light.intensity;
    }

    let lightv = (light.position - point).normalize();
    let ambient = effective_color * material.ambient;
    if in_shadow {
        return ambient;
    }
    let light_dot_normal = lightv.dot(&normalv);

    let reflectv;
    let reflect_dot_eye;
    let factor;
    let diffuse;
    let specular;

    if light_dot_normal < 0.0 {
        diffuse = RayRgb::black();
        specular = 0.0;
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        reflectv = reflect(&(-lightv), &normalv);
        reflect_dot_eye = reflectv.dot(&eyev);
        if reflect_dot_eye <= 0.0 {
            specular = 0.0;
        } else {
            factor = pow(reflect_dot_eye, material.shininess);
            specular = light.intensity as f32 * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}

pub fn stripe_at_object(pattern: &Pattern, object: &Sphere, world_point: Point3<f32>) -> RayRgb {
    let op = object.transform.inverse_transform_point(&world_point);
    pattern.stripe_at(&op)
}

// #[test]
// fn test_lighting() {
//     let m = Material::default();
//     let position = Point3::new(0.0, 0.0, 0.0);

//     let light = PointLight::new(RayRgb::white(), Point3::new(0.0, 0.0, -10.0));
//     let eyev = Vector3::new(0.0, 0.0, -1.0);
//     let normalv = Vector3::new(0.0, 0.0, -1.0);
//     let result = lighting(&m, &light, position, eyev, normalv, false);

//     assert_relative_eq!(result.r, 1.9, epsilon = 0.0001);
//     assert_relative_eq!(result.g, 1.9, epsilon = 0.0001);
//     assert_relative_eq!(result.b, 1.9, epsilon = 0.0001);

//     let light = PointLight::new(RayRgb::white(), Point3::new(0.0, 0.0, -10.0));
//     let normalv = Vector3::new(0.0, 0.0, -1.0);
//     let eyev = Vector3::new(0.0, (2.0 as f32).sqrt() / 2.0, -(2.0 as f32).sqrt() / 2.0);
//     let result = lighting(&m, &light, position, eyev, normalv, false);

//     assert_relative_eq!(result.r, 1.0, epsilon = 0.0001);
//     assert_relative_eq!(result.g, 1.0, epsilon = 0.0001);
//     assert_relative_eq!(result.b, 1.0, epsilon = 0.0001);

//     let light = PointLight::new(RayRgb::white(), Point3::new(0.0, 10.0, -10.0));
//     let eyev = Vector3::new(0.0, 0.0, -1.0);
//     let normalv = Vector3::new(0.0, 0.0, -1.0);
//     let result = lighting(&m, &light, position, eyev, normalv, false);

//     assert_relative_eq!(result.r, 0.7364, epsilon = 0.0001);
//     assert_relative_eq!(result.g, 0.7364, epsilon = 0.0001);
//     assert_relative_eq!(result.b, 0.7364, epsilon = 0.0001);

//     let light = PointLight::new(RayRgb::white(), Point3::new(0.0, 10.0, -10.0));
//     let eyev = Vector3::new(0.0, -(2.0 as f32).sqrt() / 2.0, -(2.0 as f32).sqrt() / 2.0);
//     let normalv = Vector3::new(0.0, 0.0, -1.0);
//     let result = lighting(&m, &light, position, eyev, normalv, false);

//     assert_relative_eq!(result.r, 1.6364, epsilon = 0.0001);
//     assert_relative_eq!(result.g, 1.6364, epsilon = 0.0001);
//     assert_relative_eq!(result.b, 1.6364, epsilon = 0.0001);

//     let light = PointLight::new(RayRgb::white(), Point3::new(0.0, 0.0, 10.0));
//     let eyev = Vector3::new(0.0, 0.0, -1.0);
//     let normalv = Vector3::new(0.0, 0.0, -1.0);
//     let result = lighting(&m, &light, position, eyev, normalv, false);

//     assert_relative_eq!(result.r, 0.1, epsilon = 0.0001);
//     assert_relative_eq!(result.g, 0.1, epsilon = 0.0001);
//     assert_relative_eq!(result.b, 0.1, epsilon = 0.0001);

//     let light = PointLight::new(RayRgb::white(), Point3::new(0.0, 0.0, -10.0));
//     let eyev = Vector3::new(0.0, 0.0, -1.0);
//     let normalv = Vector3::new(0.0, 0.0, -1.0);
//     let result = lighting(&m, &light, position, eyev, normalv, true);

//     assert_relative_eq!(result.r, 0.1, epsilon = 0.0001);
//     assert_relative_eq!(result.g, 0.1, epsilon = 0.0001);
//     assert_relative_eq!(result.b, 0.1, epsilon = 0.0001);
// }
