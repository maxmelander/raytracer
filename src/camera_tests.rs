#[cfg(test)]
use super::utils::is_equal;
use super::camera::Camera;
use super::matrix::Matrix4;
use super::tuple::Tuple;
use super::world::World;
use super::color::Color;

use std::f64::consts::PI;

#[test]
fn contruct_camera() {
    let c = Camera::new(160, 120, PI / 2.0);

    assert_eq!(c.h_size, 160);
    assert_eq!(c.v_size, 120);
    assert_eq!(c.field_of_view, PI / 2.0);
    assert_eq!(c.transform, Matrix4::new_identity())
}

#[test]
fn pixel_size_horizontal_canvas() {
    let c = Camera::new(200, 125, PI / 2.);
    assert_eq!(is_equal(c.pixel_size, 0.01), true);
}

#[test]
fn pixel_size_vertical_canvas() {
    let c = Camera::new(125, 200, PI / 2.);
    assert_eq!(is_equal(c.pixel_size, 0.01), true);
}

#[test]
fn ray_through_center_canvas() {
    let c = Camera::new(201, 101, PI / 2.);
    let r = c.ray_for_pixel(100, 50).unwrap();

    assert_eq!(r.origin, Tuple::new_point(0., 0., 0.));
    assert_eq!(r.direction, Tuple::new_vector(0., 0., -1.));
}

#[test]
fn ray_through_corner_canvas() {
    let c = Camera::new(201, 101, PI / 2.);
    let r = c.ray_for_pixel(0, 0).unwrap();

    assert_eq!(r.origin, Tuple::new_point(0., 0., 0.));
    assert_eq!(r.direction, Tuple::new_vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn ray_camera_transformed() {
    let mut c = Camera::new(201, 101, PI / 2.);
    c.transform = Matrix4::new_rotation_y(PI / 4.0) * Matrix4::new_translation(0., -2., 5.);

    let r = c.ray_for_pixel(100, 50).unwrap();

    assert_eq!(r.origin, Tuple::new_point(0., 2., -5.));
    assert_eq!(r.direction, Tuple::new_vector(2.0_f64.sqrt()/2.0, 0., -2.0_f64.sqrt()/2.0));
}

#[test]
fn render_world_with_camera() {
    let w: World = Default::default();
    let mut c = Camera::new(11, 11, PI / 2.);
    c.transform = Matrix4::new_view_transform(
        Tuple::new_point(0., 0., -5.),
        Tuple::new_point(0., 0., 0.),
        Tuple::new_vector(0., 1., 0.)
    );

    let canvas = c.render(&w).unwrap();

    assert_eq!(canvas.get_color(5, 5).unwrap(), Color::new(0.38066, 0.47583, 0.2855));
}
