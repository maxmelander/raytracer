mod tuple;
mod tuple_tests;

use crate::tuple::Tuple;

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
    let env = Environment {
        gravity: Tuple::new_vector(0., -0.1, 0.),
        wind: Tuple::new_vector(-0.01, 0., 0.)
    };

    let mut proj = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.0, 1.0, 0.0)
    };

    println!("Starting pos: {}", proj.position.y());
    while proj.position.y() >= 0.0 {
        proj.tick(&env);
        println!("Position: {:?}", proj.position);
    }
}

