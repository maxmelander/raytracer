#[cfg(test)]
use super::tuple::*;
#[test]
fn create_point() -> Result<(), String> {
    let point = Tuple::new_point(4.0, -4.0, 3.0);
    assert_eq!(point.w(), 1.0);
    if let TupleType::Point = point.get_type() {
        Ok(())
    } else {
        Err(String::from("The tuple type was not expected"))
    }
}

#[test]
fn create_vector() -> Result<(), String> {
    let vector = Tuple::new_vector(4.0, -4.0, 3.0);
    assert_eq!(vector.w(), 0.0);
    if let TupleType::Vector = vector.get_type() {
        Ok(())
    } else {
        Err(String::from("The tuple type was not expected"))
    }
}

#[test]
fn is_equal(){
    let a = Tuple::new_point(1.0, 2.0, -3.0);
    let b = Tuple::new_point(1.0000001, 2.0, -3.0);
    assert!(a == b);
    assert!(b == a);
}

#[test]
fn is_not_equal() {
    let a = Tuple::new_vector(1.0, 1.0, 1.0);
    let b = Tuple::new_point(1.0, 1.0, 1.0);
    assert!(a != b);
    assert!(b != a);
}

#[test]
fn addition() {
    let a = Tuple::new_point(3.0, -2.0, 5.0);
    let b = Tuple::new_vector(-2.0, 3.0, 1.0);
    let expected = Tuple::new_point(1.0, 1.0, 6.0);
    assert_eq!(a + b, expected);
}

#[test]
fn subtract_two_points() {
    let a = Tuple::new_point(3., 2., 1.);
    let b = Tuple::new_point(5., 6., 7.);
    let expected = Tuple::new_vector(-2., -4., -6.);
    assert_eq!(a - b, expected);
}

#[test]
fn subtract_vector_from_point() {
    let a = Tuple::new_point(3., 2., 1.);
    let b = Tuple::new_vector(5., 6., 7.);
    let expected = Tuple::new_point(-2., -4., -6.);
    assert_eq!(a - b, expected);
}

#[test]
fn subtract_two_vectors() {
    let a = Tuple::new_vector(3., 2., 1.);
    let b = Tuple::new_vector(5., 6., 7.);
    let expected = Tuple::new_vector(-2., -4., -6.);
    assert_eq!(a - b, expected);
}

#[test]
fn negate() {
    let a = -Tuple::new_point(1.0, -2.0, 3.0);
    assert_eq!(a.x(), -1.0);
    assert_eq!(a.y(), 2.0);
    assert_eq!(a.z(), -3.0);
    assert_eq!(a.w(), -1.0);
}

#[test]
fn scalar_mul() {
    let a = Tuple::new_point(1.0, -2.0, 3.0);
    let res = a * 3.5;
    assert_eq!(res.x(), 3.5);
    assert_eq!(res.y(), -7.0);
    assert_eq!(res.z(), 10.5);
    assert_eq!(res.w(), 3.5);
}

#[test]
fn scalar_div() {
    let a = Tuple::new_point(1.0, -2.0, 3.0);
    let res = a / 2.0;
    assert_eq!(res.x(), 0.5);
    assert_eq!(res.y(), -1.0);
    assert_eq!(res.z(), 1.5);
    assert_eq!(res.w(), 0.5);
}

#[test]
fn magnitude() {
    let a = Tuple::new_vector(1.0, 0.0, 0.0);
    let b = Tuple::new_vector(0.0, 1.0, 0.0);
    let c = Tuple::new_vector(0.0, 0.0, 1.0);
    let d = Tuple::new_vector(1.0, 2.0, 3.0);
    let e = Tuple::new_vector(-1.0, -2.0, -3.0);

    assert_eq!(a.magnitude(), 1.0);
    assert_eq!(b.magnitude(), 1.0);
    assert_eq!(c.magnitude(), 1.0);
    assert_eq!(d.magnitude(), 14.0_f64.sqrt());
    assert_eq!(e.magnitude(), 14.0_f64.sqrt());
}

#[test]
fn normalize() {
    let a = Tuple::new_vector(4.0, 0.0, 0.0);
    let b = Tuple::new_vector(1.0, 2.0, 3.0);

    assert_eq!(a.normalize(), Tuple::new_vector(1.0, 0.0, 0.0));
    assert_eq!(b.normalize(), Tuple::new_vector(1.0 / 14.0_f64.sqrt(), 2.0 / 14.0_f64.sqrt(), 3.0 / 14.0_f64.sqrt()));
    assert_eq!(b.normalize().magnitude(), 1.0);
}

#[test]
fn dot_product() {
    let a = Tuple::new_vector(1.0, 2.0, 3.0);
    let b = Tuple::new_vector(2.0, 3.0, 4.0);
    assert_eq!(a.dot(&b), 20.0);
}

#[test]
fn cross_product() {
    let a = Tuple::new_vector(1.0, 2.0, 3.0);
    let b = Tuple::new_vector(2.0, 3.0, 4.0);
    assert_eq!(a.cross(&b), Tuple::new_vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(&a), Tuple::new_vector(1.0, -2.0, 1.0));
}
