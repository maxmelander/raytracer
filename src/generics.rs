use super::sphere::Sphere;
use super::plane::Plane;
use super::ray::Ray;
use super::tuple::Tuple;
use super::intersection::Intersection;
use super::matrix::Matrix4;
use super::shape::Shape;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Drawables {
    Sphere(Sphere),
    Plane(Plane),
}

impl Drawables {
    pub fn intersect(&self, ray: Ray) -> Option<[Option<Intersection>; 2]> {
        let ts = self.local_intersect(ray)?;

        let i1 = match ts[0] {
            Some(t) => Some(Intersection::new(t, self)),
            None => None,
        };

        let i2 = match ts[1] {
            Some(t) => Some(Intersection::new(t, self)),
            None => None,
        };

        Some([i1, i2])
    }

    pub fn normal_at(&self, world_point: Tuple) -> Option<Tuple> {
        if world_point.is_vector() {
            return None;
        }

        let object_shape = self.get_shape();
        let object_point = object_shape.get_object_point(world_point)?;
        let object_normal = self.local_normal_at(object_point);
        let mut world_normal = object_shape.get_world_normal(object_normal)?;
        world_normal.w = 0.0;

        Some(world_normal.normalize())
    }
}

pub trait Drawable {
    fn local_intersect(&self, local_ray: Ray, ) -> Option<[Option<f64>; 2]>;
    fn local_normal_at(&self, local_point: Tuple) -> Tuple;
    fn get_transform(&self) -> Matrix4;
    fn get_shape(&self) -> Shape;
}

impl Drawable for Drawables {
    // NOTE: This doesn't really have to be a nested optional
    // could just return an array of 2 None values of misses
    fn local_intersect(&self, local_ray: Ray) -> Option<[Option<f64>; 2]> {
        match self {
            Drawables::Sphere(s) => s.local_intersect(local_ray),
            Drawables::Plane(p) => p.local_intersect(local_ray),
        }
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        match self {
            Drawables::Sphere(s) => s.local_normal_at(local_point),
            Drawables::Plane(p) => p.local_normal_at(local_point),
        }
    }

    fn get_transform(&self) -> Matrix4 {
        match self {
            Drawables::Sphere(s) => s.get_transform(),
            Drawables::Plane(p) => p.get_transform(),
        }
    }

    fn get_shape(&self) -> Shape {
         match self {
            Drawables::Sphere(s) => s.get_shape(),
            Drawables::Plane(p) => p.get_shape(),
        }
    }
}
