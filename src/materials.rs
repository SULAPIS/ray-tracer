use image::Rgb;

use crate::ray_rgb::RayRgb;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: RayRgb,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: usize,
}

impl Material {
    pub fn new(color: RayRgb) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: RayRgb::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200,
        }
    }
}
