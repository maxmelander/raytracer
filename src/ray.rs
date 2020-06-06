use super::intersection::Intersection;
use super::matrix::Matrix4;
use super::sphere::Sphere;
use super::tuple::Tuple;
use super::world::World;
use super::generics::{Drawables, Drawable};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Result<Self, &'static str> {
        if !origin.is_point() || !direction.is_vector() {
            return Err("Origin must be a point and direction must be a vector");
        }
        Ok(Self { origin, direction })
    }

    pub fn position(self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn intersect(self, object: &Drawables) -> Option<[Option<Intersection>; 2]> {
        let local_ray = self.transform(object.get_transform().inverse()?);
        object.intersect(local_ray)
    }

    // NOTE(Optimization); Is it faster to have a fixed size array here, and just not fill it up
    // if we don't get enough intersections?
    // And then live with the fact that we have a limit to how many intersections we can find
    pub fn intersect_world(self, world: &World) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = vec![];

        let objects_iter = world.objects.iter();
        for o in objects_iter {
            if let Some(intersections) = self.intersect(o) {
                for o_i in intersections.iter() {
                    if let Some(i) = o_i {
                        xs.push(*i)
                    }
                }
            }
        }

        xs.sort();
        xs
    }

    pub fn transform(self, matrix: Matrix4) -> Self {
        Self {
            origin: matrix * self.origin,
            direction: matrix * self.direction,
        }
    }
}
