use super::matrix::Matrix4;
use super::tuple::Tuple;
use super::shape::Shape;
use super::generics::Drawable;
use super::ray::Ray;

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
    fn local_intersect(&self, local_ray: Ray) -> Option<[Option<f64>; 2]> {
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

        Some([
            Some(t1),
            Some(t2)
        ])
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        local_point - Tuple::new_point(0., 0., 0.)
    }

    fn get_transform(&self) -> Matrix4 {
        self.shape.transform
    }

    fn get_shape(&self) -> Shape {
        self.shape
    }
}
