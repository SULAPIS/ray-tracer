#[cfg(test)]
use approx::assert_relative_eq;
use rapier3d::{
    na::{Point3, Vector3},
    prelude::*,
};

use crate::{computation::Computation, light::lighting, ray_rgb::RayRgb, sphere::Sphere};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    fn new(t: f32, object: &'a Sphere) -> Self {
        Intersection { t, object }
    }
}

pub fn intersect<'a>(s: &'a Sphere, r: &Ray) -> Option<Vec<Intersection<'a>>> {
    let sphere_to_ray = Vector3::new(
        r.origin.x - s.center.x,
        r.origin.y - s.center.y,
        r.origin.z - s.center.z,
    );
    let a = r.dir.dot(&r.dir);
    let b = 2.0 * r.dir.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - s.radius * s.radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        let sqrt_d = discriminant.sqrt();
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);

        let xs: Vec<Intersection> = [t1, t2].iter().map(|&x| Intersection::new(x, &s)).collect();
        return Some(xs);
    }
    None
}

pub fn intersection<'a>(t: f32, s: &'a Sphere) -> Intersection<'a> {
    Intersection::new(t, s)
}

pub fn hit<'a>(s: &'a Sphere, r: &Ray) -> Option<Intersection<'a>> {
    let xs = intersect(s, r);
    match xs {
        Some(xs) => xs
            .iter()
            .filter(|&i| i.t > 0.0)
            .min_by(|&x, &y| (x.t).partial_cmp(&y.t).unwrap())
            .and_then(|&i| Some(i)),
        None => None,
    }
}

pub fn normal_at(sphere: &Sphere, p: Point3<f32>) -> Vector3<f32> {
    let object_point = sphere.transform.inverse() * p;
    let object_normal = object_point - sphere.center;
    sphere
        .transform
        .inverse_transform_vector(&object_normal)
        .normalize()
}

pub fn reflect(v_in: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    v_in - normal * 2.0 * v_in.dot(normal)
}

pub fn prepare_computations<'a>(intersection: &'a Intersection, ray: &Ray) -> Computation<'a> {
    let point = ray.point_at(intersection.t);
    let normalv = normal_at(&intersection.object, point);
    let mut comps = Computation::new(
        intersection.t,
        intersection.object,
        point,
        -ray.dir,
        normalv,
    );
    if normalv.dot(&comps.eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = -comps.normalv;
    } else {
        comps.inside = false;
    }

    comps
}

#[test]
fn test_comp() {
    //inside
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let i = intersection(4.0, &shape);

    let comps = prepare_computations(&i, &r);

    assert_eq!(comps.inside, false);

    //outside
    let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let i = intersection(1.0, &shape);

    let comps = prepare_computations(&i, &r);

    assert_eq!(comps.inside, true);
    assert_relative_eq!(comps.point, Point3::new(0.0, 0.0, 1.0));
    assert_relative_eq!(comps.eyev, Vector3::new(0.0, 0.0, -1.0));
    assert_relative_eq!(comps.normalv, Vector3::new(0.0, 0.0, -1.0));
}
