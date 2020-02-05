mod tuple;
mod tuple_tests;

mod color;
mod color_tests;

mod canvas;
mod canvas_tests;

mod matrix;
mod matrix_tests;

mod ray;
mod ray_tests;

mod sphere;

use std::f64::consts::PI;

use crate::tuple::Tuple;
use crate::color::Color;
use crate::canvas::Canvas;
use crate::matrix::Matrix4;

use std::fs;


fn main() {
    let width = 500;
    let height = 500;

    let mut canvas = Canvas::new(width, height);

    let mut p = Tuple::new_point(0., -200., 0.);
    let rotation = Matrix4::new_rotation_z((2. * PI) / 12.);

    let _ = canvas.write_pixel(
        (p.x + (width as f64 / 2.)) as usize,
        (p.y + (height as f64 / 2.)) as usize,
        Color::new(1., 0., 1.));

    for _ in 1..12 {
        p = rotation * p;

        let _ = canvas.write_pixel(
            (p.x + (width as f64 / 2.)) as usize,
            (p.y + (height as f64 / 2.)) as usize,
            Color::new(1., 0., 1.));
    }

    let ppm = canvas.to_ppm();
    fs::write("/Users/maxmelander/Development/test.ppm", ppm).expect("Unable to write file");
}

