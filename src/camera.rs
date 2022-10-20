#[cfg(test)]
use approx::assert_relative_eq;
use image::{ImageBuffer, Rgb};
use rapier3d::{
    na::{Isometry3, Point3, Rotation3, Translation3, UnitQuaternion, Vector3},
    prelude::*,
};
use std::f32::consts::PI;

use crate::{
    ray_rgb::RayRgb,
    world::{color_at, World},
};
use std::env;
pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f32,
    pub transform: Isometry3<f32>,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f32) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f32 / vsize as f32;
        let half_width;
        let half_height;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Isometry3::identity(),
            pixel_size: (half_width * 2.0) / hsize as f32,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let x = x as f32;
        let y = y as f32;
        let xoffset = (x + 0.5) * self.pixel_size;
        let yoffset = (y + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self
            .transform
            .inverse_transform_point(&Point3::new(world_x, world_y, -1.0));
        let origin = self
            .transform
            .inverse_transform_point(&Point3::new(0.0, 0.0, 0.0));
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }
}

pub fn render(camera: &Camera, world: &World) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = image::ImageBuffer::new(camera.vsize, camera.hsize);

    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = camera.ray_for_pixel(x, y);
            let color = color_at(&world, &ray);
            let pixel = image.get_pixel_mut(y, x);
            *pixel = color.to_rgb();
        }
    }
    image
}

#[test]
fn test_camera() {
    let c = Camera::new(200, 125, PI / 2.0);
    assert_relative_eq!(c.pixel_size, 0.01, epsilon = 0.0001);
    let c = Camera::new(125, 200, PI / 2.0);
    assert_relative_eq!(c.pixel_size, 0.01, epsilon = 0.0001);
}

#[test]
fn test_ray_for_pixel() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(100, 50);
    assert_relative_eq!(r.origin, Point3::new(0.0, 0.0, 0.0));
    assert_relative_eq!(r.dir, Vector3::new(0.0, 0.0, -1.0));

    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(0, 0);
    assert_relative_eq!(r.origin, Point3::new(0.0, 0.0, 0.0));
    assert_relative_eq!(
        r.dir,
        Vector3::new(0.66519, 0.33259, -0.66851),
        epsilon = 0.0001
    );

    let mut c = Camera::new(201, 101, PI / 2.0);

    c.transform
        .append_translation_mut(&Translation3::new(0.0, -2.0, 5.0));
    c.transform
        .append_rotation_mut(&UnitQuaternion::new(Vector3::y() * PI / 4.0));

    let r = c.ray_for_pixel(100, 50);
    assert_relative_eq!(r.origin, Point3::new(0.0, 2.0, -5.0), epsilon = 0.001);
    assert_relative_eq!(
        r.dir,
        Vector3::new((2.0 as f32).sqrt() / 2.0, 0.0, -(2.0 as f32).sqrt() / 2.0),
        epsilon = 0.0001
    );
}

#[test]
fn test_render() {
    let w = World::default();
    let mut c = Camera::new(11, 11, PI / 2.0);

    let from = Point3::new(0.0, 0.0, -5.0);
    let to = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    c.transform = Isometry3::look_at_rh(&from, &to, &up);
    let image = render(&c, &w);
    let p = image.get_pixel(5, 5);
    let eq_p = RayRgb::new(0.38066, 0.47583, 0.2855).to_rgb();
    assert_relative_eq!(p.0[0] as f32, eq_p.0[0] as f32, epsilon = 0.001);
    assert_relative_eq!(p.0[1] as f32, eq_p.0[1] as f32, epsilon = 0.001);
    assert_relative_eq!(p.0[2] as f32, eq_p.0[2] as f32, epsilon = 0.001);
}
