use super::intersection::Intersection;
use super::matrix::Matrix4;
use super::sphere::Sphere;
use super::tuple::Tuple;
use super::world::World;

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

    pub fn intersect(self, sphere: Sphere) -> Option<[Intersection; 2]> {
        let i_ray = self.transform(sphere.transform.inverse()?);
        let sphere_to_ray = i_ray.origin - sphere.origin;
        let a = i_ray.direction.dot(i_ray.direction);
        let b = 2.0 * i_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some([Intersection::new(t1, sphere), Intersection::new(t2, sphere)])
    }

    // NOTE(Optimization); Is it faster to have a fixed size array here, and just not fill it up
    // if we don't get enough intersections?
    // And then live with the fact that we have a limit to how many intersections we can find
    pub fn intersect_world(self, world: &World) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = vec![];

        let objects_iter = world.objects.iter();
        for o in objects_iter {
            if let Some(i) = self.intersect(*o) {
                xs.extend_from_slice(&i);
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
