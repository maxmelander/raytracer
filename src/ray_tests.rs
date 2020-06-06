#[cfg(test)]
use super::intersection::*;
use super::matrix::*;
use super::ray::*;
use super::sphere::*;
use super::tuple::*;
use super::world::World;
use super::generics::Drawables;

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
    let ray = Ray::new(Tuple::new_point(2., 3., 4.), Tuple::new_vector(1., 0., 0.)).unwrap();

    assert_eq!(ray.position(0.), Tuple::new_point(2., 3., 4.));
    assert_eq!(ray.position(1.), Tuple::new_point(3., 3., 4.));
    assert_eq!(ray.position(-1.), Tuple::new_point(1., 3., 4.));
    assert_eq!(ray.position(2.5), Tuple::new_point(4.5, 3., 4.));
}

#[test]
fn ray_sphere_intersects() {
    let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let sphere = Drawables::Sphere(Sphere::new());

    let xs = ray.intersect(&sphere).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].unwrap().t, 4.0);
    assert_eq!(xs[1].unwrap().t, 6.0);
}

#[test]
fn ray_sphere_tangent_intersect() {
    let r = Ray::new(Tuple::new_point(0., 1., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let s = Drawables::Sphere(Sphere::new());

    let xs = r.intersect(&s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].unwrap().t, 5.0);
    assert_eq!(xs[1].unwrap().t, 5.0);
}

#[test]
fn ray_sphere_miss() {
    let r = Ray::new(Tuple::new_point(0., 2., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let s = Drawables::Sphere(Sphere::new());

    let xs = r.intersect(&s);

    assert_eq!(xs, None);
}

#[test]
fn ray_sphere_inside() {
    let r = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let s = Drawables::Sphere(Sphere::new());

    let xs = r.intersect(&s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].unwrap().t, -1.0);
    assert_eq!(xs[1].unwrap().t, 1.0);
}

#[test]
fn ray_sphere_behind() {
    let r = Ray::new(Tuple::new_point(0., 0., 5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let s = Drawables::Sphere(Sphere::new());

    let xs = r.intersect(&s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].unwrap().t, -6.0);
    assert_eq!(xs[1].unwrap().t, -4.0);
}

#[test]
fn intersection_has_t_and_object() {
    let r = Ray::new(Tuple::new_point(0., 0., 5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let s = Drawables::Sphere(Sphere::new());

    let i = Intersection::new(3.5, &s);

    assert_eq!(i.t, 3.5);
    assert_eq!(*i.object, s);
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
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let s = Drawables::Sphere(Sphere::new());

    let xs = r.intersect(&s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(*xs[0].unwrap().object, s);
    assert_eq!(*xs[1].unwrap().object, s);
}

#[test]
fn hit_all_positive() {
    let s = Drawables::Sphere(Sphere::new());
    let i1 = Intersection::new(1., &s);
    let i2 = Intersection::new(2., &s);

    let xs = vec![i1, i2];
    let i = hit(&xs);

    assert_eq!(i, Some(i1));
}

#[test]
fn hit_some_negative() {
    let s = Drawables::Sphere(Sphere::new());
    let i1 = Intersection::new(-1., &s);
    let i2 = Intersection::new(1., &s);
    let xs = vec![i1, i2];
    let i = hit(&xs);
    assert_eq!(i, Some(i2));
}

#[test]
fn hit_all_negative() {
    let s = Drawables::Sphere(Sphere::new());
    let i1 = Intersection::new(-2., &s);
    let i2 = Intersection::new(-1., &s);
    let xs = vec![i1, i2];
    let i = hit(&xs);
    assert_eq!(i, None);
}

#[test]
fn hit_lowest_non_negative() {
    let s = Drawables::Sphere(Sphere::new());

    let i1 = Intersection::new(5., &s);
    let i2 = Intersection::new(7., &s);
    let i3 = Intersection::new(-3., &s);
    let i4 = Intersection::new(2., &s);

    let xs = vec![i1, i2, i3, i4];

    let i = hit(&xs);
    assert_eq!(i, Some(i4));
}

#[test]
fn ray_translation() {
    let r = Ray::new(Tuple::new_point(1., 2., 3.), Tuple::new_vector(0., 1., 0.)).unwrap();

    let m = Matrix4::new_translation(3., 4., 5.);

    let r2 = r.transform(m);

    assert_eq!(r2.origin, Tuple::new_point(4., 6., 8.));
    assert_eq!(r2.direction, Tuple::new_vector(0., 1., 0.));
}

#[test]
fn ray_scaling() {
    let r = Ray::new(Tuple::new_point(1., 2., 3.), Tuple::new_vector(0., 1., 0.)).unwrap();

    let m = Matrix4::new_scaling(2., 3., 4.);

    let r2 = r.transform(m);

    assert_eq!(r2.origin, Tuple::new_point(2., 6., 12.));
    assert_eq!(r2.direction, Tuple::new_vector(0., 3., 0.));
}

#[test]
fn scaled_sphere_intersect() {
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let s = Drawables::Sphere(Sphere::new_with_transform(Matrix4::new_scaling(2., 2., 2.)));

    let xs = r.intersect(&s).unwrap();

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].unwrap().t, 3.);
    assert_eq!(xs[1].unwrap().t, 7.);
}

#[test]
fn translated_sphere_intersect() {
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let s = Drawables::Sphere(Sphere::new_with_transform(Matrix4::new_translation(5., 0., 0.)));

    let xs = r.intersect(&s);

    assert_eq!(xs, None);
}

#[test]
fn intersect_world() {
    let w: World = Default::default();
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

    let xs = r.intersect_world(&w);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.);
}

#[test]
fn precompute_intersection_state() {
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();
    let shape = Sphere::new();
    let i = Intersection{
        t: 4.0,
        object: &Drawables::Sphere(shape)
    };

    let comps = i.prepare_computations(r).unwrap();

    assert_eq!(comps.t, i.t);
    assert_eq!(comps.object, i.object);
    assert_eq!(comps.point, Tuple::new_point(0., 0., -1.));
    assert_eq!(comps.eye_v, Tuple::new_vector(0., 0., -1.));
    assert_eq!(comps.normal_v, Tuple::new_vector(0., 0., -1.));
}

#[test]
fn precompue_hit_intersection_outside() {
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();
    let shape = Sphere::new();

    let i = Intersection{
        t: 4.0,
        object: &Drawables::Sphere(shape)
    };

    let comps = i.prepare_computations(r).unwrap();

    assert_eq!(comps.inside, false);
}

#[test]
fn precompue_hit_intersection_inside() {
    let r = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.)).unwrap();
    let shape = Sphere::new();

    let i = Intersection{
        t: 1.0,
        object: &Drawables::Sphere(shape)
    };

    let comps = i.prepare_computations(r).unwrap();
    assert_eq!(comps.t, i.t);
    assert_eq!(comps.object, i.object);
    assert_eq!(comps.point, Tuple::new_point(0., 0., 1.));
    assert_eq!(comps.eye_v, Tuple::new_vector(0., 0., -1.));
    assert_eq!(comps.normal_v, Tuple::new_vector(0., 0., -1.));
    assert_eq!(comps.inside, true);
}
