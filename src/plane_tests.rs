#[cfg(test)]
use super::plane::*;
use super::tuple::Tuple;
use super::ray::Ray;
use super::utils::EPSILON;
use super::generics::{Drawable, Drawables};

#[test]
fn normal_at_plane() {
    let p = Plane::new();
    let expected = Tuple::new_vector(0., 1., 0.);

    assert_eq!(p.local_normal_at(Tuple::new_point(0., 0., 0.)), expected);
    assert_eq!(p.local_normal_at(Tuple::new_point(10., 0., -10.)), expected);
    assert_eq!(p.local_normal_at(Tuple::new_point(-5., 0., 150.)), expected);
}

#[test]
fn intersect_plane_parallel() {
    let p = Plane::new();
    let r = Ray::new(Tuple::new_point(0., 10., 0.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let xs = p.local_intersect(r);
    assert_eq!(xs, None);
}

#[test]
fn intersect_plane_coplanar() {
    let p = Plane::new();
    let r = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let xs = p.local_intersect(r);
    assert_eq!(xs, None);
}

#[test]
fn intersect_plane_from_above() {
    let p = Drawables::Plane(Plane::new());
    let r = Ray::new(Tuple::new_point(0., 1., 0.), Tuple::new_vector(0., -1., 0.)).unwrap();
    let xs = p.intersect(r).unwrap();

    assert_eq!(xs[0].unwrap().t, 1.);
    assert_eq!(*xs[0].unwrap().object, p);

    assert_eq!(xs[1], None);
}

#[test]
fn intersect_plane_from_below() {
    let p = Drawables::Plane(Plane::new());
    let r = Ray::new(Tuple::new_point(0., -1., 0.), Tuple::new_vector(0., 1., 0.)).unwrap();
    let xs = p.intersect(r).unwrap();

    assert_eq!(xs[0].unwrap().t, 1.);
    assert_eq!(*xs[0].unwrap().object, p);

    assert_eq!(xs[1], None);
}
