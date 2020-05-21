#[cfg(test)]
use super::camera::Camera;
use super::matrix::Matrix4;

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
    let c = Camera::new(200, 125, PI / 2);
    assert_eq!(c.pixel_size, 0.01);
}

#[test]
fn pixel_size_vertical_canvas() {
    let c = Camera::new(125, 200, PI / 2);
    assert_eq!(c.pixel_size, 0.01);
}

#[test]
fn ray_through_center_canvas() {
    let c = Camera::new(201, 101, PI / 2);
    let r = c.ray_for_pixel(0, 0);
}
