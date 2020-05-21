#[cfg(test)]

use super::world::World;
use super::point_light::PointLight;
use super::tuple::Tuple;
use super::color::Color;
use super::sphere::Sphere;
use super::material::Material;
use super::matrix::Matrix4;
use super::ray::Ray;
use super::intersection::Intersection;

#[test]
fn default_world() {
    let light = PointLight::new(Tuple::new_point(-10., 10., -10.), Color::new(1., 1., 1.)).unwrap();
    let mut s1 = Sphere::new();
    s1.material = Material{
        color: Color::new(0.8, 1.0, 0.6),
        diffuse: 0.7,
        specular: 0.2,
        ..Default::default()
    };

    let s2 = Sphere::new_with_transform(Matrix4::new_scaling(0.5, 0.5, 0.5));
    let w: World = Default::default();

    assert_eq!(w.lights[0], light);
    assert_eq!(w.objects.contains(&s1), true);
    assert_eq!(w.objects.contains(&s2), true);
}

#[test]
fn shade_intersection() {
    let w: World = Default::default();
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let shape = w.objects[0];
    let i = Intersection {
        t: 4.0,
        object: shape
    };

    let comps = i.prepare_computations(r).unwrap();
    let c = w.shade_hit(comps);
    assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
}

#[test]
fn shade_intersection_inside() {
    let w = World{
        lights: vec![PointLight::new(Tuple::new_point(0., 0.25, 0.), Color::new(1., 1., 1.)).unwrap()],
        ..Default::default()
    };

    let r = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let shape = w.objects[1];
    let i = Intersection {
        t: 0.5,
        object: shape
    };

    let comps = i.prepare_computations(r).unwrap();
    let c = w.shade_hit(comps);
    assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
}

#[test]
fn color_ray_miss() {
    let w: World = Default::default();
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 1., 0.)).unwrap();

    let c = w.color_at(r);
    assert_eq!(c, Color::new(0., 0., 0.));
}

#[test]
fn color_ray_hit() {
    let w: World = Default::default();
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let c = w.color_at(r);
    assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
}

#[test]
fn color_ray_intersect_behind() {
    let material = Material {
        ambient: 1.0,
        ..Default::default()
    };

    let mut outer = Sphere::new();
    outer.material = material;

    let mut inner = Sphere::new();
    inner.material = material;


    let w = World {
        objects: vec![outer, inner],
        ..Default::default()
    };

    let r = Ray::new(Tuple::new_point(0., 0., 0.75), Tuple::new_vector(0., 0., -1.)).unwrap();

    let c = w.color_at(r);
    assert_eq!(c, inner.material.color);
}
