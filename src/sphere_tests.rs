#[cfg(test)]
use super::sphere::*;
use super::matrix::*;
use super::tuple::Tuple;

#[test]
fn sphere_default_transform() {
    let s = Sphere::new();
    assert_eq!(s.transform, Matrix4::new_identity());
}

#[test]
fn sphere_change_transform() {
    let mut s = Sphere::new();
    let t = Matrix4::new_translation(2., 3., 4.);
    s.set_transform(t);
    assert_eq!(s.transform, t);
}

#[test]
fn normal_point_x_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(1., 0., 0.)).unwrap();
    assert_eq!(n, Tuple::new_vector(1., 0., 0.));
}

#[test]
fn normal_point_y_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(0., 1., 0.)).unwrap();
    assert_eq!(n, Tuple::new_vector(0., 1., 0.));
}

#[test]
fn normal_point_z_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(0., 0., 1.)).unwrap();
    assert_eq!(n, Tuple::new_vector(0., 0., 1.));
}

#[test]
fn normal_point_non_axial() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)).unwrap();
    assert_eq!(n, Tuple::new_vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.));
}

#[test]
fn normal_point_normalized() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)).unwrap();
    assert_eq!(n, n.normalize());
}

#[test]
fn normal_translated_sphere() {
    let s = Sphere::new_with_transform(Matrix4::new_translation(0., 1., 0.));
    let n = s.normal_at(Tuple::new_point(0., 1.70711, -0.70711)).unwrap();
    assert_eq!(n, Tuple::new_vector(0., 0.70711, -0.70711));
}

#[test]
fn normal_transformed_sphere() {
    use std::f64::consts::PI;
    let m = Matrix4::new_scaling(1., 0.5, 1.) * Matrix4::new_rotation_z(PI / 5.);
    let s = Sphere::new_with_transform(m);
    let n = s.normal_at(Tuple::new_point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.)).unwrap();
    assert_eq!(n, Tuple::new_vector(0., 0.97014, -0.24254));
}
