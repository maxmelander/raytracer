use super::tuple::Tuple;
use super::sphere::Sphere;
use super::intersection::Intersection;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Result<Self, &'static str> {
        if !origin.is_point() || !direction.is_vector() {
            return Err("Origin must be a point and direction must be a vector");
        }
        Ok(Self{origin, direction})
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn intersect(&self, sphere: Sphere) -> Option<[Intersection; 2]> {
        let sphere_to_ray = self.origin - sphere.origin;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * self.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {return None}

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some([Intersection::new(t1, sphere), Intersection::new(t2, sphere)])
    }
}
