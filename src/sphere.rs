use super::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Tuple
}

#[allow(dead_code)]
impl Sphere {
    pub fn new() -> Self {
        Self{origin: Tuple::new_point(0., 0., 0.)}
    }
}
