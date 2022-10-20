pub mod ray {
    use crate::tuples::tuples::Point;
    #[cfg(test)]
    use assert_approx_eq::assert_approx_eq;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Ray {
        pub origin: Point,
        pub direction: Point,
    }

    impl Ray {
        pub fn new(origin: Point, direction: Point) -> Self {
            Self { origin, direction }
        }

        pub fn at(&self, t: f32) -> Point {
            self.origin + self.direction * t
        }

        pub fn translation(&self, v: &[f32; 3]) -> Self {
            Ray {
                origin: Point::new(
                    self.origin.x() + v[0],
                    self.origin.y() + v[1],
                    self.origin.z() + v[2],
                ),
                direction: self.direction.clone(),
            }
        }

        pub fn scaling(&self, v: &[f32; 3]) -> Self {
            Ray {
                origin: Point::new(
                    self.origin.x() * v[0],
                    self.origin.y() * v[1],
                    self.origin.z() * v[2],
                ),
                direction: Point::new(
                    self.direction.x() * v[0],
                    self.direction.y() * v[1],
                    self.direction.z() * v[2],
                ),
            }
        }
    }

    #[test]
    fn test_ray() {
        let p = Point::new(0.1, 0.2, 0.3);
        let q = Point::new(0.2, 0.3, 0.4);

        let r = Ray::new(p, q);

        assert_approx_eq!(r.origin.x(), 0.1);
        assert_approx_eq!(r.origin.y(), 0.2);
        assert_approx_eq!(r.origin.z(), 0.3);
        assert_approx_eq!(r.direction.x(), 0.2);
        assert_approx_eq!(r.direction.y(), 0.3);
        assert_approx_eq!(r.direction.z(), 0.4);
    }

    #[test]
    fn test_ray_at() {
        let p = Point::new(0.0, 0.0, 0.0);
        let q = Point::new(1.0, 2.0, 3.0);

        let r = Ray::new(p, q);
        let s = r.at(0.5);

        assert_approx_eq!(s.x(), 0.5);
        assert_approx_eq!(s.y(), 1.0);
        assert_approx_eq!(s.z(), 1.5)
    }

    #[test]
    fn test_translation() {
        let p = Point::new(1.0, 2.0, 3.0);
        let q = Point::new(0.0, 1.0, 0.0);

        let r = Ray::new(p, q);

        let t_r = r.translation(&[3.0, 4.0, 5.0]);

        assert_eq!(t_r.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r.direction, t_r.direction);
    }

    #[test]
    fn test_scaling() {
        let p = Point::new(1.0, 2.0, 3.0);
        let q = Point::new(0.0, 1.0, 0.0);

        let r = Ray::new(p, q);

        let t_r = r.scaling(&[2.0, 3.0, 4.0]);

        assert_eq!(t_r.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(t_r.direction, Point::new(0.0, 3.0, 0.0));
    }
}
