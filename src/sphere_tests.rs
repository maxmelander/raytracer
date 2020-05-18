#[cfg(test)]
use super::sphere::*;
use super::matrix::*;

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
