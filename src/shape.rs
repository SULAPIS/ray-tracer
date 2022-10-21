#[cfg(test)]
use approx::assert_relative_eq;
use rapier3d::{
    na::{Isometry3, Point3, Scale, Scale3, Transform3, Translation3, Vector3},
    parry::query::RayCast,
    prelude::{Shape, *},
};

use crate::materials::Material;

#[derive(Debug, Clone, PartialEq)]
pub struct ShapeBase {
    pub transform: Isometry3<f32>,
    pub scale: Scale3<f32>,
    pub material: Material,
    pub center: Point3<f32>,
}

impl Default for ShapeBase {
    fn default() -> Self {
        Self {
            transform: Isometry3::default(),
            scale: Scale3::new(1.0, 1.0, 1.0),
            material: Material::default(),
            center: Point3::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait ShapeT {
    fn intersect(&self, ray: &Ray) -> Ray;
    fn normal_at(&self, point: &Point3<f32>) -> Vector3<f32>;
}

impl ShapeBase {
    pub fn intersect(&self, ray: &Ray) -> Ray {
        ray.inverse_transform_by(&self.transform)
    }
    pub fn normal_at(&self, p: &Point3<f32>) -> Vector3<f32> {
        local_normal_at(&self, *p)
    }
}
pub fn local_normal_at(sphere: &ShapeBase, p: Point3<f32>) -> Vector3<f32> {
    let object_point = sphere.transform.inverse() * p;
    let object_normal = object_point - sphere.center;
    sphere
        .transform
        .inverse_transform_vector(&object_normal)
        .normalize()
}

#[test]
fn test_intersect() {
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
    let mut s = ShapeBase::default();
    s.scale = Scale3::new(2.0, 2.0, 2.0);
    s.transform
        .append_translation_mut(&Translation3::new(5.0, 0.0, 0.0));

    let tr = s.intersect(&r);
    assert_relative_eq!(tr.origin, Point3::new(-5.0, 0.0, -2.5));
    assert_relative_eq!(tr.dir, Vector3::new(0.0, 0.0, 0.5));
}

#[test]
fn test_normal_at() {
    let mut s = ShapeBase::default();
    s.transform
        .append_translation_mut(&Translation3::new(0.0, 1.0, 0.0));
    let p = s.normal_at(&Point3::new(0.0, 1.70711, -0.70711));
    assert_relative_eq!(p, Vector3::new(0.0, 0.70711, -0.70711), epsilon = 0.0001);
}
