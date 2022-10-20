use rapier3d::{
    na::{Point3, Scale3, Vector3},
    prelude::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        Point3::from(self.origin + self.direction * t)
    }

    pub fn translation(&self, t: &Translation<f64>) -> Self {
        Ray {
            origin: t.transform_point(&self.origin),
            direction: self.direction.clone(),
        }
    }

    pub fn scaling(&self, v: &Scale3<f64>) -> Self {
        Ray {
            origin: Point3::from(v.transform_point(&self.origin)),
            direction: Vector3::from(self.direction.component_mul(&v.vector)),
        }
    }
}

#[test]
fn test_ray_at() {
    let p = Point3::new(0.0, 0.0, 0.0);
    let q = Vector3::new(1.0, 2.0, 3.0);

    let r = Ray::new(p, q);
    let s = r.at(0.5);

    assert_eq!(s, Point3::new(0.5, 1.0, 1.5));
}

#[test]
fn test_translation() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let q = Vector3::new(0.0, 1.0, 0.0);

    let r = Ray::new(p, q);

    let t_r = r.translation(&Translation::from(Vector3::new(3.0, 4.0, 5.0)));

    assert_eq!(t_r.origin, Point3::new(4.0, 6.0, 8.0));
    assert_eq!(r.direction, t_r.direction);
}

#[test]
fn test_scaling() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let q = Vector3::new(0.0, 1.0, 0.0);

    let r = Ray::new(p, q);

    let t_r = r.scaling(&Scale3::new(2.0, 3.0, 4.0));

    assert_eq!(t_r.origin, Point3::new(2.0, 6.0, 12.0));
    assert_eq!(t_r.direction, Vector3::new(0.0, 3.0, 0.0));
}
