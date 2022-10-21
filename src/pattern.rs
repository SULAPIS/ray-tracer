use rapier3d::na::Point3;

use crate::ray_rgb::RayRgb;
#[cfg(test)]
use approx::assert_relative_eq;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    a: RayRgb,
    b: RayRgb,
}
impl Default for Pattern {
    fn default() -> Self {
        Self {
            a: RayRgb::white(),
            b: RayRgb::black(),
        }
    }
}
impl Pattern {
    pub fn new(a: RayRgb, b: RayRgb) -> Pattern {
        Pattern { a, b }
    }
    pub fn stripe_at(&self, point: &Point3<f32>) -> RayRgb {
        if ((point.y * 2.0).floor()) % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[test]
fn test_stripe_pattern() {
    let p = Pattern::new(RayRgb::white(), RayRgb::black());
    assert_relative_eq!(
        RayRgb::black().r,
        p.stripe_at(&Point3::new(1.0, 1.0, 0.0)).r,
        epsilon = 0.0001
    );
    let p = Pattern::new(RayRgb::white(), RayRgb::black());
    assert_relative_eq!(
        RayRgb::black().r,
        p.stripe_at(&Point3::new(-0.1, 1.0, 0.0)).r,
        epsilon = 0.0001
    );
}
