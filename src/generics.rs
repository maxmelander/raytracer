use super::sphere::Sphere;
use super::ray::Ray;
use super::tuple::Tuple;
use super::intersection::Intersection;
use super::matrix::Matrix4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Drawables {
    Sphere(Sphere),
}

pub trait Drawable {
    fn local_intersect(self, local_ray: Ray, ) -> Option<[Intersection; 2]>;
    fn local_normal_at(self, local_point: Tuple) -> Option<Tuple>;
    fn get_transform(&self) -> Matrix4;
}

impl Drawable for Drawables {
    fn local_intersect(self, local_ray: Ray) -> Option<[Intersection; 2]> {
        match self {
            Drawables::Sphere(s) => s.local_intersect(local_ray)
        }
    }

    fn local_normal_at(self, local_point: Tuple) -> Option<Tuple> {
        match self {
            Drawables::Sphere(s) => s.local_normal_at(local_point)
        }
    }

    fn get_transform(&self) -> Matrix4 {
        match self {
            Drawables::Sphere(s) => s.get_transform()
        }
    }
}
