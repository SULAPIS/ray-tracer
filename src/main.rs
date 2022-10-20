use std::f32::consts::PI;

use image::Rgb;
use rapier3d::{
    na::{Isometry3, Point3, Vector3},
    prelude::*,
};
use ray_tracer::{
    camera::{render, Camera},
    sphere::*,
};
use ray_tracer::{ray_rgb::RayRgb, world::World};
use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut w = World::default();
    let mut m = Sphere::default();
    m.material.color = RayRgb::new(0.1, 1.0, 0.5);
    m.material.diffuse = 0.6;
    m.material.specular = 0.3;
    m.material.shininess = 10;
    m.center = Point3::new(-0.5, 1.0, 0.5);
    w.objects.push(m);
    let mut r = Sphere::default();
    r.material.color = RayRgb::new(0.5, 1.0, 0.1);
    r.material.diffuse = 0.5;
    r.material.specular = 0.6;
    r.center = Point3::new(1.5, 0.5, -0.5);
    r.radius = 0.5;
    w.objects.push(r);

    let mut q = Sphere::default();
    q.material.color = RayRgb::new(0.2, 1.0, 0.9);
    q.material.diffuse = 0.7;
    q.material.specular = 0.3;
    q.center = Point3::new(0.0, 0.5, 10.5);
    q.radius = 10.0;
    w.objects.push(q);

    let mut q = Sphere::default();
    q.material.color = RayRgb::new(0.2, 0.0, 0.9);
    q.material.diffuse = 0.3;
    q.material.specular = 0.3;
    q.material.shininess = 10;
    q.center = Point3::new(0.0, 2.5, 0.5);
    q.radius = 0.6;
    w.objects.push(q);

    let mut camera = Camera::new(200, 250, PI / 3.0);
    camera.transform = Isometry3::look_at_rh(
        &Point3::new(0.0, 1.5, -5.0),
        &Point3::new(0.0, 1.0, 0.0),
        &Vector3::new(0.0, 1.0, 0.0),
    );

    let canvas = render(&camera, &w);
    canvas.save("ray.jpg");
}
