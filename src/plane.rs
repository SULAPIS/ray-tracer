use crate::shape::*;
use rapier3d::{na::Point3, prelude::Ray};

pub struct Plane {
    pub base: ShapeBase,
    pub center: Point3<f32>,
}

impl ShapeT for Plane {
    fn intersect(&self, ray: &Ray) -> Ray {
        self.base.intersect(ray)
    }

    fn normal_at(&self, point: &Point3<f32>) -> rapier3d::na::Vector3<f32> {
        self.base.normal_at(&point)
    }
}

#[test]
fn test_transform() {}
