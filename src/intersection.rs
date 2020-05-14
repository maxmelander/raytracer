use super::sphere::Sphere;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere // NOTE: This one should be more general later
}

#[allow(dead_code)]
impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Self {
        Self{t, object}
    }
}
