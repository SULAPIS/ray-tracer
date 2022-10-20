use rapier3d::na::{Point3, Vector3};

use crate::sphere::Sphere;

#[derive(Debug, Clone, PartialEq)]
pub struct Computation<'a> {
    pub t: f32,
    pub object: &'a Sphere,
    pub point: Point3<f32>,
    pub eyev: Vector3<f32>,
    pub normalv: Vector3<f32>,
    pub inside: bool,
    pub over_point: Point3<f32>,
}
impl<'a> Computation<'a> {
    pub fn new(
        t: f32,
        object: &'a Sphere,
        point: Point3<f32>,
        eyev: Vector3<f32>,
        normalv: Vector3<f32>,
        over_point: Point3<f32>,
    ) -> Self {
        Self {
            t,
            object: &object,
            point,
            eyev,
            normalv,
            inside: false,
            over_point,
        }
    }
}
