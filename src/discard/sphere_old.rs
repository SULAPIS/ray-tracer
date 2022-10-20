pub mod sphere {

    use rapier3d::na::{IsContiguous, Isometry3, Matrix4};
    use rapier3d::prelude::*;

    use crate::{intersections::intersections::Intersection, ray::ray::Ray, tuples::tuples::Point};
    #[derive(Debug, Clone, PartialEq)]
    pub struct Sphere {
        pub center: Point,
        pub radius: f32,
        pub transform: Matrix4<f32>,
    }

    impl Sphere {
        pub fn new(center: Point, radius: f32) -> Self {
            Sphere {
                center,
                radius,
                transform: Matrix4::identity(),
            }
        }

        pub fn intersect(&self, ray: &Ray) -> Option<[f32; 2]> {
            let sphere_to_ray = ray.origin - self.center;
            let a = ray.direction.length_squared();
            let half_b = sphere_to_ray.dot(&ray.direction);
            let c = sphere_to_ray.length_squared() - self.radius * self.radius;
            let discriminant = (half_b * half_b) - (a * c);

            if discriminant >= 0.0 {
                let sqrtd = discriminant.sqrt();
                let root_a = ((-half_b) - sqrtd) / a;
                let root_b = ((-half_b) + sqrtd) / a;
                return Some([root_a, root_b]);
            }
            None
        }

        pub fn hit(&self, ray: &Ray) -> Option<f32> {
            self.intersect(ray).and_then(|intersection| {
                intersection
                    .iter()
                    .filter(|&i| *i > 0.0)
                    .max_by(|&x, &y| x.partial_cmp(y).unwrap())
                    .and_then(|&i| Some(i))
            })
        }
    }

    // pub fn set_transform(s:&mut Sphere,t:&[f32;3]){

    // }

    #[test]
    fn test_sphere_outside_pass() {
        let p = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Point::new(0.0, 0.0, 1.0));
        let points = p.intersect(&ray);
        assert_eq!(points, Some([4.0, 6.0]));
    }
    #[test]
    fn test_sphere_tangent() {
        let p = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Point::new(0.0, 0.0, 1.0));
        let points = p.intersect(&ray);
        assert_eq!(points, Some([5.0, 5.0]));
    }
    #[test]
    fn test_sphere_inside() {
        let p = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Point::new(0.0, 0.0, 1.0));
        let points = p.intersect(&ray);
        assert_eq!(points, None);
    }
    #[test]
    fn test_sphere_outside_miss() {
        let p = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point::new(0.0, 2.0, 5.0), Point::new(0.0, 0.0, 1.0));
        let points = p.intersect(&ray);
        assert_eq!(points, None);
    }

    #[test]
    fn matrix_transform() {
        let iso = Isometry3::<f32>::new(vector![5.0, -3.0, 2.0], vector![0.0, 0.0, 0.0]);
        let np = iso
            .translation
            .transform_point(&rapier3d::na::Point3::new(-3.0, 4.0, 5.0));
        // let nn=iso.
        assert_eq!(np, rapier3d::na::Point3::new(2.0, 1.0, 7.0));
    }
}
