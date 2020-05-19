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

    // NOTE: The way I have set up vectors and points to have a shared type is not optimal
    // there is no way right now for me to specify that his function should take a point, not vector
    pub fn normal_at(&self, world_point: Tuple) -> Option<Tuple> {
        if world_point.is_vector() {
            None
        } else {
            let object_point = self.transform.inverse()? * world_point;
            let object_normal = object_point - Tuple::new_point(0., 0., 0.);
            let mut world_normal = self.transform.inverse()?.transpose() * object_normal;
            world_normal.w = 0.0;

            Some(world_normal.normalize())
        }
    }
}
