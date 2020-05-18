use super::tuple::Tuple;
use super::matrix::Matrix4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Tuple,
    pub transform: Matrix4

}

#[allow(dead_code)]
impl Sphere {
    pub fn new() -> Self {
        Self{origin: Tuple::new_point(0., 0., 0.), transform: Matrix4::new_identity()}
    }

    pub fn new_with_transform(transform: Matrix4) -> Self {
        Self{origin: Tuple::new_point(0., 0., 0.), transform}
    }

    // NOTE: Should this mutate the struct, or just return a new copy?
    pub fn set_transform(&mut self, matrix: Matrix4) {
        self.transform = matrix;
    }
}
