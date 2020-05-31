#[cfg(test)]
use super::utils::*;
use super::tuple::Tuple;
use super::material::Material;
use super::point_light::PointLight;
use super::color::Color;

#[test]
fn lighting_eye_between_light_and_surface() {
    let m: Material = Default::default();
    let position = Tuple::new_point(0., 0., 0.);

    let eye_v = Tuple::new_vector(0., 0., -1.);
    let normal_v = Tuple::new_vector(0., 0., -1.);

    let light = PointLight::new(Tuple::new_point(0., 0., -10.), Color::new(1., 1., 1.)).unwrap();

    let result = lighting(m, position, light, eye_v, normal_v, false).unwrap();
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_eye_offset_45() {
    let m: Material = Default::default();
    let position = Tuple::new_point(0., 0., 0.);

    let eye_v = Tuple::new_vector(0., 2_f64.sqrt()/2., -2_f64.sqrt()/2.);
    let normal_v = Tuple::new_vector(0., 0., -1.);

    let light = PointLight::new(Tuple::new_point(0., 0., -10.), Color::new(1., 1., 1.)).unwrap();

    let result = lighting(m, position, light, eye_v, normal_v, false).unwrap();
    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn lighting_light_offset_45() {
    let m: Material = Default::default();
    let position = Tuple::new_point(0., 0., 0.);

    let eye_v = Tuple::new_vector(0., 0., -1.);
    let normal_v = Tuple::new_vector(0., 0., -1.);

    let light = PointLight::new(Tuple::new_point(0., 10., -10.), Color::new(1., 1., 1.)).unwrap();

    let result = lighting(m, position, light, eye_v, normal_v, false).unwrap();
    assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_eye_at_reflection() {
    let m: Material = Default::default();
    let position = Tuple::new_point(0., 0., 0.);

    let eye_v = Tuple::new_vector(0., -2_f64.sqrt()/2., -2_f64.sqrt()/2.);
    let normal_v = Tuple::new_vector(0., 0., -1.);

    let light = PointLight::new(Tuple::new_point(0., 10., -10.), Color::new(1., 1., 1.)).unwrap();

    let result = lighting(m, position, light, eye_v, normal_v, false).unwrap();
    assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
}

#[test]
fn lighting_light_behind_surface() {
    let m: Material = Default::default();
    let position = Tuple::new_point(0., 0., 0.);

    let eye_v = Tuple::new_vector(0., 0., -1.);
    let normal_v = Tuple::new_vector(0., 0., -1.);

    let light = PointLight::new(Tuple::new_point(0., 0., 10.), Color::new(1., 1., 1.)).unwrap();

    let result = lighting(m, position, light, eye_v, normal_v, false).unwrap();
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn lighting_with_suface_in_shadow() {
    let m: Material = Default::default();
    let position = Tuple::new_point(0., 0., 0.);

    let eye_v = Tuple::new_vector(0., 0., -1.);
    let normal_v = Tuple::new_vector(0., 0., -1.);

    let light = PointLight::new(Tuple::new_point(0., 0., -10.), Color::new(1., 1., 1.)).unwrap();

    let in_shadow = true;
    let result = lighting(m, position, light, eye_v, normal_v, in_shadow).unwrap();
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}
