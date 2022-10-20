#[cfg(test)]
use approx::assert_relative_eq;
use std::ops::Mul;

use assert_approx_eq::assert_approx_eq;

use rapier3d::{
    na::{Isometry3, Point3, Scale3, Translation3, Vector3},
    prelude::*,
};

use crate::{intersections::*, materials::Material};
#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub transform: Isometry3<f32>,
    pub scale: Scale3<f32>,
    pub material: Material,
}
impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32) -> Self {
        Self {
            center,
            radius,
            transform: Isometry3::identity(),
            scale: Scale3::new(1.0, 1.0, 1.0),
            material: Material::default(),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Isometry3::identity(),
            scale: Scale3::new(1.0, 1.0, 1.0),
            material: Material::default(),
        }
    }
}

#[test]
fn test_lifetime() {
    let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
    let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
    let xs = intersect(&s, &ray).unwrap();

    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
    assert_eq!(xs[0].object, &s);
}

#[test]
fn test_hit() {
    let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
    let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));

    let i = hit(&s, &ray);
    assert_eq!(i, Some(Intersection { t: 4.0, object: &s }));
}

#[test]
fn test_normal_at() {
    let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

    assert_eq!(
        normal_at(&s, Point3::new(1.0, 0.0, 0.0)),
        Vector3::new(1.0, 0.0, 0.0)
    );
    assert_eq!(
        normal_at(&s, Point3::new(0.0, 0.0, 1.0)),
        Vector3::new(0.0, 0.0, 1.0)
    );
    assert_approx_eq!(
        normal_at(
            &s,
            Point3::new((3.0 as f32).sqrt() / 3.0, 0.577350, 0.577350)
        )
        .x,
        0.577350
    );
}
#[test]
fn test_normal_at_no_origin() {
    let mut s = Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0);
    s.transform
        .append_translation_mut(&Translation3::new(1.0, 1.0, 1.0));

    assert_relative_eq!(
        normal_at(&s, Point3::new(1.0, 2.70711, -0.70711 + 1.0)),
        Vector3::new(0.0, 0.70711, -0.70711),
        epsilon = 0.0001
    );
}

#[test]
fn test_reflect() {
    let v = Vector3::new(1.0, -1.0, 0.0);
    let n = Vector3::new(0.0, 1.0, 0.0);

    let r = reflect(&v, &n);
    assert_relative_eq!(r, Vector3::new(1.0, 1.0, 0.0), epsilon = 0.0001);

    let v1 = Vector3::new(0.0, -1.0, 0.0);
    let sqrt_2 = (2.0 as f32).sqrt() / 2.0;
    let n1 = Vector3::new(sqrt_2, sqrt_2, 0.0);

    let r = reflect(&v1, &n1);
    assert_relative_eq!(r, Vector3::new(1.0, 0.0, 0.0));
}
