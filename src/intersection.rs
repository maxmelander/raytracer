use std::cmp::Ordering;

use super::utils::{is_equal, EPSILON};
use super::ray::Ray;
use super::tuple::Tuple;
use super::generics::{Drawables, Drawable};

pub fn float_compare(a: f64, b: f64) -> Ordering {
    if is_equal(a, b) {
        return Ordering::Equal;
    }
    if a - b < 0.0 {
        return Ordering::Less;
    }
    Ordering::Greater
}

#[derive(Debug)]
pub struct Comps<'a> {
    pub t: f64,
    pub object: &'a Drawables,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Drawables,
}

#[allow(dead_code)]
impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Drawables) -> Self {
        Self { t, object }
    }

    pub fn prepare_computations(&self, ray: Ray) -> Option<Comps> {
        let t = self.t;
        let object = self.object;
        let point = ray.position(self.t);

        let eye_v = -ray.direction;

        let mut normal_v = object.normal_at(point)?;

        let mut inside = false;

        if normal_v.dot(eye_v) < 0.0 {
            inside = true;
            normal_v = -normal_v;
        }

        let over_point = point + normal_v * EPSILON;

        Some(Comps {
            t,
            object,
            point,
            over_point,
            eye_v,
            normal_v,
            inside,
        })
    }
}

impl Ord for Intersection<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        float_compare(self.t, other.t)
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(float_compare(self.t, other.t))
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.t, other.t) && self.object == other.object
    }
}

impl Eq for Intersection<'_> {}

#[allow(dead_code)]
pub fn hit<'a>(xs: &[Intersection<'a>]) -> Option<Intersection<'a>> {
    let mut iter = xs.iter().filter(|x| x.t.is_sign_positive());
    let init = iter.next()?;

    iter.try_fold(*init, |acc, x| {
        let cmp = x.partial_cmp(&acc)?;
        let min = if let Ordering::Less = cmp { x } else { &acc };
        Some(*min)
    })

}
