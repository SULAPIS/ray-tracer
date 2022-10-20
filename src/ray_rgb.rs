use std::ops::{Add, Mul, Sub};

#[cfg(test)]
use approx::assert_relative_eq;
use approx::RelativeEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayRgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RayRgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
    pub fn to_rgb(&self) -> image::Rgb<u8> {
        let r = Self::to_u8(self.r);
        let g = Self::to_u8(self.g);
        let b = Self::to_u8(self.b);
        image::Rgb([r, g, b])
    }

    fn to_u8(i: f32) -> u8 {
        if i >= 1.0 {
            return 255;
        } else if i <= 0.0 {
            return 0;
        } else {
            return (255.0 * i) as u8;
        }
    }
}

impl Add<Self> for RayRgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        RayRgb {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl Add<f32> for RayRgb {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        RayRgb {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
        }
    }
}
impl Sub for RayRgb {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        RayRgb {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<usize> for RayRgb {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            r: self.r * rhs as f32,
            g: self.g * rhs as f32,
            b: self.b * rhs as f32,
        }
    }
}
impl Mul<f32> for RayRgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
