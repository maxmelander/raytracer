use super::material::Material;
use super::matrix::Matrix4;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Shape {
    pub origin: Tuple,
    pub transform: Matrix4,
    pub material: Material,
}

impl Shape {
     pub fn new_with_transform(transform: Matrix4) -> Self {
        Self {
            transform,
            ..Default::default()
        }
    }

    pub fn get_object_point(self, world_point: Tuple) -> Option<Tuple> {
        Some(self.transform.inverse()? * world_point)
    }

    pub fn get_world_normal(self, object_normal: Tuple) -> Option<Tuple> {
        Some(self.transform.inverse()?.transpose() * object_normal)
    }

}

impl Default for Shape {
    fn default() -> Self {
        let material: Material = Default::default();
        Self {
            origin: Tuple::new_point(0., 0., 0.),
            transform: Matrix4::new_identity(),
            material,
        }
    }
}
