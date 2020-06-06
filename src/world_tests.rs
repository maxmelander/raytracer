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
use super::generics::Drawables;
use super::utils::EPSILON;

#[test]
fn default_world() {
    let light = PointLight::new(Tuple::new_point(-10., 10., -10.), Color::new(1., 1., 1.)).unwrap();
    let mut s1 = Sphere::new();
    s1.shape.material = Material{
        color: Color::new(0.8, 1.0, 0.6),
        diffuse: 0.7,
        specular: 0.2,
        ..Default::default()
    };

    let s2 = Sphere::new_with_transform(Matrix4::new_scaling(0.5, 0.5, 0.5));
    let w: World = Default::default();

    assert_eq!(w.lights[0], light);
    assert_eq!(w.objects.contains(&Drawables::Sphere(s1)), true);
    assert_eq!(w.objects.contains(&Drawables::Sphere(s2)), true);
}

#[test]
fn shade_intersection() {
    let w: World = Default::default();
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let shape = w.objects[0];
    let i = Intersection {
        t: 4.0,
        object: &shape
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
        object: &shape
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
    outer.shape.material = material;

    let mut inner = Sphere::new();
    inner.shape.material = material;


    let w = World {
        objects: vec![Drawables::Sphere(outer), Drawables::Sphere(inner)],
        ..Default::default()
    };

    let r = Ray::new(Tuple::new_point(0., 0., 0.75), Tuple::new_vector(0., 0., -1.)).unwrap();

    let c = w.color_at(r);
    assert_eq!(c, inner.shape.material.color);
}

#[test]
fn no_shadow() {
    let w: World = Default::default();
    let p = Tuple::new_point(0., 10., 0.);

    assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
}

#[test]
fn shadow_object_between_point_and_light() {
    let w: World = Default::default();
    let p = Tuple::new_point(10., -10., 10.);

    assert_eq!(w.is_shadowed(p, &w.lights[0]), true);
}

#[test]
fn no_shadow_object_behing_light() {
    let w: World = Default::default();
    let p = Tuple::new_point(-20., 20., -20.);

    assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
}

#[test]
fn no_shadow_object_behind_point() {
    let w: World = Default::default();
    let p = Tuple::new_point(-2., 2., -2.);

    assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
}

#[test]
fn shade_hit_intersection_in_shadow() {
    let s1 = Drawables::Sphere(Sphere::new());
    let s2 = Drawables::Sphere(Sphere::new_with_transform(Matrix4::new_translation(0., 0., 10.)));
    let mut w = World {
        lights: vec![PointLight::new(Tuple::new_point(0., 0., -10.), Color::new(1., 1., 1.)).unwrap()],
        objects: vec![s1, s2]
    };

    let r = Ray::new(Tuple::new_point(0., 0., 5.), Tuple::new_vector(0., 0., 1.)).unwrap();
    let i = Intersection::new(4., &s2);
    let comps = i.prepare_computations(r).unwrap();

    let c = w.shade_hit(comps);
    assert_eq!(c, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn hit_should_offset() {
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();
    let s = Drawables::Sphere(Sphere::new_with_transform(Matrix4::new_translation(0., 0., 1.)));
    let i = Intersection::new(5., &s);
    let comps = i.prepare_computations(r).unwrap();

    assert_eq!(comps.over_point.z < -EPSILON / 2.0, true);
    assert_eq!(comps.point.z > comps.over_point.z, true);


}