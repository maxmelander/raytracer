use super::color::Color;
use super::tuple::Tuple;
use super::generics::{Drawables, Drawable};
use super::matrix::Matrix4;

// TODO: This maybe doesn't have to be an enum and just some structs
// implementing a trait with a shared struct they hold.
// But then it would mean the material has to hold a pointer to the pattern because
// of possible size differences

pub trait Pattern {
    fn color_at(&self, point: Tuple) -> Color;
    fn set_transform(&mut self, transform: Matrix4);
    fn get_transform(&self) -> Matrix4;
    fn get_a(&self) -> Color;
    fn get_b(&self) -> Color;
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Patterns {
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checker(Checker),
    Test(Test),
}


#[allow(dead_code)]
impl Patterns {
    pub fn new_stripe(a: Color, b: Color) -> Self {
        Self::Stripe(Stripe{
            a,
            b,
            transform: Matrix4::new_identity()
        })
    }

    pub fn new_gradient(a: Color, b: Color) -> Self {
        Self::Gradient(Gradient{
            a,
            b,
            transform: Matrix4::new_identity()
        })
    }

    pub fn new_ring(a: Color, b: Color) -> Self {
        Self::Ring(Ring{
            a,
            b,
            transform: Matrix4::new_identity()
        })
    }

    pub fn new_checker(a: Color, b: Color) -> Self {
        Self::Checker(Checker{
            a,
            b,
            transform: Matrix4::new_identity()
        })
    }

    pub fn new_test() -> Self {
        Self::Test(Test{
            transform: Matrix4::new_identity()
        })
    }

    pub fn color_at_object(&self, object: &Drawables, point: Tuple) -> Option<Color> {
        let object_point = object.get_transform().inverse()? * point;
        let pattern_point = self.get_transform().inverse()? * object_point;

        Some(self.color_at(pattern_point))
    }
}

impl Pattern for Patterns {
    fn color_at(&self, point: Tuple) -> Color {
        match self {
            Self::Stripe(s) => s.color_at(point),
            Self::Gradient(g) => g.color_at(point),
            Self::Ring(r) => r.color_at(point),
            Self::Checker(c) => c.color_at(point),
            Self::Test(t) => t.color_at(point)
        }
    }

    fn set_transform(&mut self, transform: Matrix4) {
        match self {
            Self::Stripe(s) => s.set_transform(transform),
            Self::Gradient(g) => g.set_transform(transform),
            Self::Ring(r) => r.set_transform(transform),
            Self::Checker(c) => c.set_transform(transform),
            Self::Test(t) => t.set_transform(transform)
        }
    }

    fn get_transform(& self) -> Matrix4 {
         match self {
            Self::Stripe(s) => s.get_transform(),
            Self::Gradient(g) => g.get_transform(),
            Self::Ring(r) => r.get_transform(),
            Self::Checker(c) => c.get_transform(),
            Self::Test(t) => t.get_transform()
        }
    }

    fn get_a(&self) -> Color {
        match self {
            Self::Stripe(s) => s.get_a(),
            Self::Gradient(g) => g.get_a(),
            Self::Ring(r) => r.get_a(),
            Self::Checker(c) => c.get_a(),
            Self::Test(t) => t.get_a()
        }
    }

    fn get_b(&self) -> Color {
        match self {
            Self::Stripe(s) => s.get_b(),
            Self::Gradient(g) => g.get_b(),
            Self::Ring(r) => r.get_b(),
            Self::Checker(c) => c.get_b(),
            Self::Test(t) => t.get_b()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stripe {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4,

}

impl Pattern for Stripe {
    fn color_at(&self, point: Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            return self.a;
        }
        self.b
    }


    fn set_transform(&mut self, transform: Matrix4) { self.transform = transform }
    fn get_transform(&self) -> Matrix4 { self.transform }
    fn get_a(&self) -> Color { self.a }
    fn get_b(&self) -> Color { self.b }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gradient {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4,
}

impl Pattern for Gradient {
    fn color_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + (distance * fraction)
    }

    fn set_transform(&mut self, transform: Matrix4) { self.transform = transform }
    fn get_transform(&self) -> Matrix4 { self.transform }
    fn get_a(&self) -> Color { self.a }
    fn get_b(&self) -> Color { self.b }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ring {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4,
}

impl Pattern for Ring {
    fn color_at(&self, point: Tuple) -> Color {
        if (point.x.powf(2.) + point.z.powf(2.)).sqrt() % 2. == 0. {
           return self.a;
        }
        self.b
    }

    fn set_transform(&mut self, transform: Matrix4) { self.transform = transform }
    fn get_transform(&self) -> Matrix4 { self.transform }
    fn get_a(&self) -> Color { self.a }
    fn get_b(&self) -> Color { self.b }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Checker {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4,
}

impl Pattern for Checker {
    fn color_at(&self, point: Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2. == 0. {
           return self.a;
        }
        self.b
    }

    fn set_transform(&mut self, transform: Matrix4) { self.transform = transform }
    fn get_transform(&self) -> Matrix4 { self.transform }
    fn get_a(&self) -> Color { self.a }
    fn get_b(&self) -> Color { self.b }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Test {
    pub transform: Matrix4,
}

impl Pattern for Test {
    fn color_at(&self, point: Tuple) -> Color {
        Color::new(point.x, point.y, point.z)
    }

    fn set_transform(&mut self, transform: Matrix4) { self.transform = transform }
    fn get_transform(&self) -> Matrix4 { self.transform }
    fn get_a(&self) -> Color { Color::new(0., 0., 0.) }
    fn get_b(&self) -> Color { Color::new(1., 1., 1.) }
}
