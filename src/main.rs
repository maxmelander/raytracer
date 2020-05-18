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
mod sphere_tests;

mod intersection;

use std::f64::consts::PI;

use crate::tuple::Tuple;
use crate::color::Color;
use crate::canvas::Canvas;
use crate::matrix::Matrix4;
use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::intersection::*;

use std::fs;


fn draw_clock() {
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

fn main() {
    // Scene setup
    let ray_origin = Tuple::new_point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels = 100.;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.;
    let mut sphere = Sphere::new();
    sphere.set_transform(Matrix4::new_scaling(1., 0.5, 1.));

    // Canvas setup
    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    let color = Color::new(1., 0., 1.);


    // For each coordinate in our "screen", shoot a ray from the ray origin,
    // through the sphere and to the screen coordinate. If there was a hit,
    // then draw the coordinate to the screen
    for y in 0..canvas_pixels as usize - 1 {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels as usize - 1 {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::new_point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize()).unwrap();

            let xs = r.intersect(sphere);

            if let Some(intersections) = xs {
                if hit(&intersections).is_some() {
                    let _ = canvas.write_pixel(x, y, color);
                }
            }
        }
    }
    let ppm = canvas.to_ppm();
    fs::write("/home/maxmelander/test.ppm", ppm).expect("Unable to write file");
}
