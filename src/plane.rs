use super::utils::EPSILON;
use super::shape::Shape;
use super::ray::Ray;
use super::generics::Drawable;
use super::tuple::Tuple;
use super::matrix::Matrix4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    pub shape: Shape
}

#[allow(dead_code)]
impl Plane {
    pub fn new() -> Self {
        let shape: Shape = Default::default();
        Self {
            shape
        }
    }

    pub fn new_with_transform(transform: Matrix4) -> Self {
        let shape = Shape::new_with_transform(transform);
        Self {
            shape
        }
    }
}

impl Drawable for Plane {
    fn local_intersect(&self, local_ray: Ray) -> Option<[Option<f64>; 2]> {
        if local_ray.direction.y.abs() < EPSILON {
            return None;
        }
        let t = -local_ray.origin.y / local_ray.direction.y;
        Some([Some(t), None])
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        Tuple::new_vector(0., 1., 0.)
    }

    fn get_transform(&self) -> Matrix4 {
        self.shape.transform
    }

    fn get_shape(&self) -> Shape {
        self.shape
    }
}
