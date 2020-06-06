use super::matrix::Matrix4;
use super::tuple::Tuple;
use super::shape::Shape;
use super::generics::{Drawable, Drawables};
use super::ray::Ray;
use super::intersection::Intersection;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub shape: Shape
}

impl Sphere {
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

impl Drawable for Sphere {
    fn local_intersect(self, local_ray: Ray) -> Option<[Intersection; 2]> {
        let sphere_to_ray = local_ray.origin - self.shape.origin;
        let a = local_ray.direction.dot(local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some([Intersection::new(t1, Drawables::Sphere(self)), Intersection::new(t2, Drawables::Sphere(self))])
    }

    // NOTE: The way I have set up vectors and points to have a shared type is not optimal
    // there is no way right now for me to specify that his function should take a point, not vector
    fn local_normal_at(self, world_point: Tuple) -> Option<Tuple> {
        if world_point.is_vector() {
            None
        } else {
            let object_point = self.shape.get_object_point(world_point)?;
            let object_normal = object_point - Tuple::new_point(0., 0., 0.);
            let mut world_normal = self.shape.get_world_normal(object_normal)?;
            world_normal.w = 0.0;

            Some(world_normal.normalize())
        }
    }

    fn get_transform(&self) -> Matrix4 {
        self.shape.transform
    }
}
