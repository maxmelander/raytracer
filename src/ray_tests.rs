#[cfg(test)]
use super::sphere::*;
use super::ray::*;
use super::tuple::*;
use super::intersection::*;

#[test]
fn create_ray() {
    let origin = Tuple::new_point(1., 2., 3.);
    let direction = Tuple::new_vector(4., 5., 6.);

    let ray = Ray::new(origin, direction).unwrap();

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}

#[test]
fn point_from_dist() {
    let ray = Ray::new(
        Tuple::new_point(2., 3., 4.),
        Tuple::new_vector(1., 0., 0.)
    ).unwrap();


    assert_eq!(ray.position(0.), Tuple::new_point(2., 3., 4.));
    assert_eq!(ray.position(1.), Tuple::new_point(3., 3., 4.));
    assert_eq!(ray.position(-1.), Tuple::new_point(1., 3., 4.));
    assert_eq!(ray.position(2.5), Tuple::new_point(4.5, 3., 4.));
}

#[test]
fn ray_sphere_intersects() {
    let ray = Ray::new(
        Tuple::new_point(0., 0., -5.),
        Tuple::new_vector(0., 0., 1.)
    ).unwrap();

    let sphere = Sphere::new();

    let xs = ray.intersect(sphere).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn ray_sphere_tangent_intersect() {
    let r = Ray::new(
        Tuple::new_point(0., 1., -5.),
        Tuple::new_vector(0., 0., 1.)
    ).unwrap();

    let s = Sphere::new();

    let xs = r.intersect(s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn ray_sphere_miss() {
    let r = Ray::new(
        Tuple::new_point(0., 2., -5.),
        Tuple::new_vector(0., 0., 1.)
    ).unwrap();

    let s = Sphere::new();

    let xs = r.intersect(s);

    assert_eq!(xs, None);
}

#[test]
fn ray_sphere_inside() {
    let r = Ray::new(
        Tuple::new_point(0., 0., 0.),
        Tuple::new_vector(0., 0., 1.)
    ).unwrap();

    let s = Sphere::new();

    let xs = r.intersect(s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn ray_sphere_behind() {
    let r = Ray::new(
        Tuple::new_point(0., 0., 5.),
        Tuple::new_vector(0., 0., 1.)
    ).unwrap();

    let s = Sphere::new();

    let xs = r.intersect(s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn intersection_has_t_and_object() {
    let r = Ray::new(
        Tuple::new_point(0., 0., 5.),
        Tuple::new_vector(0., 0., 1.)
    ).unwrap();

    let s = Sphere::new();

    let i = Intersection::new(3.5, s);

    assert_eq!(i.t, 3.5);
    assert_eq!(i.object, s);
}

// NOTE: Using a list primitive aka array for now
// #[test]
// fn aggregating_intersections() {
//     let s = Sphere::new();
//     let i1 = Intersection::new(1., s);
//     let i2 = Intersection::new(2., s);
//     let xs = Intersection::instersections(i1, i2).unwrap();
//     assert_eq!(xs.len(), 2);
//     assert_eq!(xs[0], i1);
//     assert_eq!(xs[1], i2);
// }

#[test]
fn intersect_sets_object_on_intersection() {
    let r = Ray::new(
        Tuple::new_point(0., 0., -5.),
        Tuple::new_vector(0., 0., 1.)
    ).unwrap();

    let s = Sphere::new();

    let xs = r.intersect(s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, s);
    assert_eq!(xs[1].object, s);
}
