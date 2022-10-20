use crate::computation::Computation;
use crate::intersections::*;
use crate::light::lighting;
use crate::ray_rgb::RayRgb;
use crate::{light::PointLight, sphere::Sphere};
#[cfg(test)]
use approx::assert_relative_eq;
use rapier3d::na::{Isometry3, Point3, Scale3, Vector3};
use rapier3d::prelude::*;
pub struct World {
    pub lights: Vec<PointLight>,
    pub objects: Vec<Sphere>,
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(RayRgb::white(), Point3::new(-10.0, 10.0, -10.0));
        let mut sphere1 = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        sphere1.material.color = RayRgb::new(0.8, 0.4, 0.6);
        sphere1.material.diffuse = 0.9;
        sphere1.material.specular = 0.4;
        sphere1.material.ambient = 0.5;
        // let mut sphere2 = Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.5);
        // sphere2.scale = Scale3::new(0.5, 0.5, 0.5);

        Self {
            lights: vec![light],
            objects: vec![sphere1],
        }
    }
}

pub fn intersect_world<'a>(world: &'a World, ray: &Ray) -> Vec<Intersection<'a>> {
    let mut intersections = Vec::new();
    for object in &world.objects {
        let res = intersect(&object, &ray);
        match res {
            Some(i) => {
                intersections = [intersections, i].concat();
            }
            None => {}
        }
    }
    intersections.sort_by(|x, y| (x.t).partial_cmp(&y.t).unwrap());
    intersections
}

pub fn shade_hit(world: &World, comps: &Computation) -> RayRgb {
    let shadowed = is_shadowed(&world, &comps.over_point);
    lighting(
        &comps.object.material,
        &world.lights[0],
        comps.over_point,
        comps.eyev,
        comps.normalv,
        shadowed,
    )
}

pub fn color_at(world: &World, ray: &Ray) -> RayRgb {
    let i = intersect_world(&world, &ray);

    if i.len() > 0 {
        let comps = prepare_computations(&i[0], &ray);
        let color = shade_hit(&world, &comps);
        return color;
    }
    RayRgb::black()
}

pub fn is_shadowed(world: &World, point: &Point3<f32>) -> bool {
    let v = world.lights[0].position - point;
    let distance = v.magnitude();
    let direction = v.normalize();
    let r = Ray::new(point.clone(), direction);
    let intersections = intersect_world(&world, &r);
    let intersections: Vec<Intersection> =
        intersections.into_iter().filter(|&i| i.t >= 0.0).collect();

    if intersections.len() > 0 && intersections[0].t < distance {
        return true;
    }
    false
}

#[test]
fn test_world() {
    let w = World::default();
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));

    let xs = intersect_world(&w, &r);
    assert_eq!(xs.len(), 4);
    assert_relative_eq!(xs[0].t, 4.0, epsilon = 0.0001);
    assert_relative_eq!(xs[1].t, 4.5, epsilon = 0.0001);
    assert_relative_eq!(xs[2].t, 5.5, epsilon = 0.0001);
    assert_relative_eq!(xs[3].t, 6.0, epsilon = 0.0001);
}

#[test]
fn test_color_at() {
    let w = World::default();
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
    let c = color_at(&w, &r);
    assert_relative_eq!(c.r, 0.0, epsilon = 0.0001);
    assert_relative_eq!(c.g, 0.0, epsilon = 0.0001);
    assert_relative_eq!(c.b, 0.0, epsilon = 0.0001);

    let w = World::default();
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
    let c = color_at(&w, &r);
    assert_relative_eq!(c.r, 0.38066, epsilon = 0.0001);
    assert_relative_eq!(c.g, 0.47583, epsilon = 0.0001);
    assert_relative_eq!(c.b, 0.2855, epsilon = 0.0001);

    let mut w = World::default();
    w.objects[0].material.ambient = 1.0;
    w.objects[1].material.ambient = 1.0;
    let r = Ray::new(Point3::new(0.0, 0.0, 0.75), Vector3::new(0.0, 0.0, -1.0));
    let c = color_at(&w, &r);

    // assert_relative_eq!(c.r, w.objects[1].material.color.r, epsilon = 0.0001);
    // assert_relative_eq!(c.g, w.objects[1].material.color.g, epsilon = 0.0001);
    // assert_relative_eq!(c.b, w.objects[1].material.color.b, epsilon = 0.0001);
}

#[test]
fn test_look_at() {
    let from = Point3::new(0.0, 0.0, 0.0);
    let to = Point3::new(0.0, 0.0, -1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let m = Isometry3::look_at_rh(&from, &to, &up);
    println!("{}", m.to_matrix());
    let from = Point3::new(1.0, 3.0, 2.0);
    let to = Point3::new(4.0, -2.0, 8.0);
    let up = Vector3::new(1.0, 1.0, 0.0);
    let m = Isometry3::look_at_rh(&from, &to, &up);

    println!("{}", m.to_matrix());
}

#[test]
fn test_is_shadowed() {
    let w = World::default();
    let p = Point3::new(0.0, 10.0, 0.0);
    let s = is_shadowed(&w, &p);
    assert_eq!(s, false);

    let w = World::default();
    let p = Point3::new(10.0, -10.0, 10.0);
    let s = is_shadowed(&w, &p);
    assert_eq!(s, true);

    let w = World::default();
    let p = Point3::new(-20.0, 20.0, -20.0);
    let s = is_shadowed(&w, &p);
    assert_eq!(s, false);

    let w = World::default();
    let p = Point3::new(-2.0, 2.0, -2.0);
    let s = is_shadowed(&w, &p);
    assert_eq!(s, false);
}
