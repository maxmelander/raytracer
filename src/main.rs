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

mod point_light;
mod point_light_tests;

mod material;
mod material_tests;

mod utils;
mod utils_tests;

mod world;
mod world_tests;

mod camera;
mod camera_tests;

use std::f64::consts::PI;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::intersection::*;
use crate::material::Material;
use crate::matrix::Matrix4;
use crate::point_light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::utils::lighting;
use crate::world::World;
use crate::camera::Camera;

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
        Color::new(1., 0., 1.),
    );

    for _ in 1..12 {
        p = rotation * p;

        let _ = canvas.write_pixel(
            (p.x + (width as f64 / 2.)) as usize,
            (p.y + (height as f64 / 2.)) as usize,
            Color::new(1., 0., 1.),
        );
    }

    let ppm = canvas.to_ppm();
    fs::write("/Users/maxmelander/Development/test.ppm", ppm).expect("Unable to write file");
}

fn draw_sphere_world() {
    let mut floor = Sphere::new_with_transform(Matrix4::new_scaling(10., 0.01, 10.));
    floor.material = Material {
        color: Color::new(1., 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    let left_wall_transform =
        Matrix4::new_translation(0., 0., 5.) *
        Matrix4::new_rotation_y(-PI / 4.) *
        Matrix4::new_rotation_x(PI / 2.0) *
        Matrix4::new_scaling(10., 0.01, 10.);

    let mut left_wall = Sphere::new_with_transform(left_wall_transform);
    left_wall.material = floor.material;

    let right_wall_transform =
        Matrix4::new_translation(0., 0., 5.) *
        Matrix4::new_rotation_y(PI / 4.) *
        Matrix4::new_rotation_x(PI / 2.0) *
        Matrix4::new_scaling(10., 0.01, 10.);

    let mut right_wall = Sphere::new_with_transform(right_wall_transform);
    right_wall.material = floor.material;


    let mut middle_sphere = Sphere::new_with_transform(Matrix4::new_translation(-0.5, 1., 0.5));
    middle_sphere.material = Material {
        color: Color::new(0.8, 0.2, 0.8),
        diffuse: 0.7,
        specular: 0.9,
        shininess: 400.,
        ..Default::default()
    };

    let mut right_sphere = Sphere::new_with_transform(
        Matrix4::new_translation(1.5, 0.5, -0.5) *
        Matrix4::new_scaling(0.5, 0.5, 0.5)
    );
    right_sphere.material = Material {
        color: Color::new(0.2, 0.2, 1.0),
        diffuse: 0.7,
        specular: 0.1,
        shininess: 100.,
        ..Default::default()
    };

    let mut small_sphere = Sphere::new_with_transform(
        Matrix4::new_translation(-1.5, 0.33, -0.75) *
        Matrix4::new_scaling(0.33, 0.33, 0.33)
    );
    small_sphere.material = Material {
        color: Color::new(1.0, 0.2, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    };

    let light =
        PointLight::new(Tuple::new_point(-10., 10., -10.), Color::new(0.4, 0.1, 0.4)).unwrap();

    let light2 =
        PointLight::new(Tuple::new_point(15., 15., -10.), Color::new(0.1, 0.0, 0.0)).unwrap();

    let world = World {
        lights: vec![light, light2],
        objects: vec![floor, left_wall, right_wall, middle_sphere, right_sphere, small_sphere]
    };

    let mut camera = Camera::new(600, 600, PI / 3.);
    camera.transform = Matrix4::new_view_transform(
        Tuple::new_point(0., 1.5, -5.),
        Tuple::new_point(0., 1., 0.),
        Tuple::new_vector(0., 1., 0.)
    );
    let canvas = camera.render(&world).unwrap();

    fs::write("/home/maxmelander/test.ppm", canvas.to_ppm()).expect("Unable to write file");
}

fn draw_sphere() {
    // Scene setup
    let ray_origin = Tuple::new_point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels = 800.;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.;
    let mut sphere = Sphere::new();
    sphere.material = Material {
        color: Color::new(1., 0.2, 1.),
        ..Default::default()
    };

    let light =
        PointLight::new(Tuple::new_point(12., 10., -10.), Color::new(0.3, 0.3, 1.0)).unwrap();

    //sphere.set_transform(Matrix4::new_scaling(1.0, 0.9, 1.));

    // Canvas setup
    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);

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
                if let Some(hit) = hit(&intersections) {
                    let point = r.position(hit.t);
                    let normal = hit.object.normal_at(point).unwrap();
                    let eye = -r.direction;
                    let color = lighting(hit.object.material, point, light, eye, normal, false).unwrap();
                    let _ = canvas.write_pixel(x, y, color);
                }
            }
        }
    }
    let ppm = canvas.to_ppm();
    fs::write("/Users/maxm/Development/test.ppm", ppm).expect("Unable to write file");
}

fn main() {
    draw_sphere_world();
}
