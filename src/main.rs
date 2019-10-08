mod tuple;
mod tuple_tests;

mod color;
mod color_tests;

mod canvas;
mod canvas_tests;

use crate::tuple::Tuple;
use crate::color::Color;
use crate::canvas::Canvas;

use std::fs;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    pub fn tick(&mut self, env: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn main() {
    let mut canvas = Canvas::new(900, 550);

    let env = Environment {
        gravity: Tuple::new_vector(0., -0.1, 0.),
        wind: Tuple::new_vector(-0.01, 0., 0.)
    };

    let mut proj = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.0, 1.8, 0.0).normalize() * 11.25
    };

    println!("Starting pos: {}", proj.position.y());
    while proj.position.y() >= 0.0 {
        proj.tick(&env);
        let _ = canvas.write_pixel(proj.position.x() as usize, (550_f64 - proj.position.y()) as usize, Color::new(1.0, 0.0, 1.0));
    }

    let ppm = canvas.to_ppm();
    fs::write("/Users/maxmelander/Downloads/test.ppm", ppm).expect("Unable to write file");
}

