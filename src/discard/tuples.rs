pub mod tuples {
    #[cfg(test)]
    use assert_approx_eq::assert_approx_eq;

    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use std::cmp::PartialEq;
    use std::f32;
    use std::ops::{Add, Div, Mul, Neg, Sub};
    #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
    pub struct Point {
        x: f32,
        y: f32,
        z: f32,
    }

    impl Point {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Point { x, y, z }
        }

        pub fn random(min: f32, max: f32) -> Self {
            let mut rng = rand::thread_rng();
            Point {
                x: rng.gen_range(min..max),
                y: rng.gen_range(min..max),
                z: rng.gen_range(min..max),
            }
        }

        pub fn distance(&self, other: &Point) -> f32 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            let dz = self.z - other.z;
            (dx * dx + dy * dy + dz * dz).sqrt()
        }

        pub fn length_squared(&self) -> f32 {
            self.x * self.x + self.y * self.y + self.z * self.z
        }

        pub fn length(&self) -> f32 {
            self.distance(&Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
        }

        pub fn unit_vector(&self) -> Self {
            let length = self.length();
            Point::new(self.x / length, self.y / length, self.z / length)
        }

        pub fn dot(&self, other: &Self) -> f32 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }

        pub fn cross(&self, other: &Self) -> Self {
            Point::new(
                self.y * other.z - self.z * other.y,
                self.z * other.x - self.x * other.z,
                self.x * other.y - self.y * other.x,
            )
        }

        pub fn near_zero(&self) -> bool {
            self.x.abs() < f32::EPSILON
                && self.y.abs() < f32::EPSILON
                && self.z.abs() < f32::EPSILON
        }

        pub fn x(&self) -> f32 {
            self.x
        }
        pub fn y(&self) -> f32 {
            self.y
        }
        pub fn z(&self) -> f32 {
            self.z
        }
    }

    impl Add for Point {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl Sub for Point {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }

    impl Neg for Point {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Point {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl Mul<Self> for Point {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
            }
        }
    }

    impl Mul<f32> for Point {
        type Output = Self;
        fn mul(self, rhs: f32) -> Self::Output {
            Point {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    impl Div<Self> for Point {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
                z: self.z / rhs.z,
            }
        }
    }
    impl Div<f32> for Point {
        type Output = Self;
        fn div(self, rhs: f32) -> Self::Output {
            Point {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }

    #[test]
    fn test_gen() {
        let p = Point {
            x: 0.1,
            y: 0.2,
            z: 0.3,
        };
        assert_eq!(p.x(), 0.1);
        assert_eq!(p.y(), 0.2);
        assert_eq!(p.z(), 0.3);

        let q = Point::new(0.2, 0.3, 0.4);
        assert_eq!(q.x(), 0.2);
        assert_eq!(q.y(), 0.3);
        assert_eq!(q.z(), 0.4);
    }

    #[test]
    fn test_add() {
        let p = Point::new(0.1, 0.2, 0.3);
        let q = Point::new(0.2, 0.3, 0.4);
        let r = p + q;
        assert_approx_eq!(r.x(), 0.3);
        assert_approx_eq!(r.y(), 0.5);
        assert_approx_eq!(r.z(), 0.7);
    }

    #[test]
    fn test_sub() {
        let p = Point::new(0.1, 0.2, 0.3);
        let q = Point::new(0.2, 0.3, 0.4);
        let r = p - q;
        assert_approx_eq!(r.x(), -0.1);
        assert_approx_eq!(r.y(), -0.1);
        assert_approx_eq!(r.z(), -0.1);
    }

    #[test]
    fn test_neg() {
        let p = Point::new(0.1, 0.2, 0.3);
        let q = -p;
        assert_approx_eq!(q.x(), -0.1);
        assert_approx_eq!(q.y(), -0.2);
        assert_approx_eq!(q.z(), -0.3);
    }

    #[test]
    fn test_mul() {
        let p = Point::new(0.1, 0.2, 0.3);
        let q = Point::new(0.2, 0.3, 0.4);
        let r = p * q;
        assert_approx_eq!(r.x(), 0.02);
        assert_approx_eq!(r.y(), 0.06);
        assert_approx_eq!(r.z(), 0.12);
    }

    #[test]
    fn test_div() {
        let p = Point::new(0.1, 0.2, 0.3);
        let q = Point::new(0.2, 0.3, 0.4);
        let r = p / q;
        assert_approx_eq!(r.x(), 0.5);
        assert_approx_eq!(r.y(), 0.6666666666666666);
        assert_approx_eq!(r.z(), 0.3 / 0.4);
    }

    #[test]
    fn test_dot() {
        let p = Point::new(0.1, 0.2, 0.3);
        let q = Point::new(0.2, 0.3, 0.4);
        assert_approx_eq!(p.dot(&q), 0.2);
    }

    #[test]
    fn test_length_squared() {
        let p = Point::new(0.1, 0.2, 0.3);
        assert_approx_eq!(p.length_squared(), 0.14);
    }

    #[test]
    fn test_random() {
        let p = Point::random(-1.0, 1.0);
        assert!(p.x() >= -1.0 && p.x() <= 1.0);
        assert!(p.y() >= -1.0 && p.y() <= 1.0);
        assert!(p.z() >= -1.0 && p.z() <= 1.0);
    }

    #[test]
    fn test_near_zero() {
        let p = Point::new(0.1, 0.2, 0.3);
        assert!(!p.near_zero());
        let p = Point::new(0.0, 0.0, 0.0);
        assert!(p.near_zero());
    }
}
