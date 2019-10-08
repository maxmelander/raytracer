#[cfg(test)]
use super::color::*;

#[test]
fn create_color() {
    let color = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(color.r(), -0.5);
    assert_eq!(color.g(), 0.4);
    assert_eq!(color.b(), 1.7);
}

#[test]
fn adding_colors() {
    let a = Color::new(0.9, 0.6, 0.75);
    let b = Color::new(0.7, 0.1, 0.25);
    let expected = Color::new(1.6, 0.7, 1.0);
    assert_eq!(a + b, expected);
}

#[test]
fn subtracting_colors() {
    let a = Color::new(0.9, 0.6, 0.75);
    let b = Color::new(0.7, 0.1, 0.25);
    let expected = Color::new(0.2, 0.5, 0.5);
    assert_eq!(a - b, expected);
}

#[test]
fn mul_color_by_scalar() {
    let a = Color::new(0.2, 0.3, 0.4);
    let scalar: f64 = 2.0;
    let expected = Color::new(0.4, 0.6, 0.8);
    assert_eq!(a * scalar, expected);
}

#[test]
fn mul_colors() {
    let a = Color::new(1.0, 0.2, 0.4);
    let b = Color::new(0.9, 1.0, 0.1);
    let expected = Color::new(0.9, 0.2, 0.04);
    assert_eq!(a * b, expected);
}
