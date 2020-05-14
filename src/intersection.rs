use std::cmp::Ordering;

use super::sphere::Sphere;

const EPSILON: f64 = 0.00001;

pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn float_compare(a: f64, b: f64) -> Ordering {
    if is_equal(a, b) {
        return Ordering::Equal;
    }
    if a - b < 0.0 {
        return Ordering::Less;
    }
    Ordering::Greater
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere // NOTE: This one should be more general later
}

#[allow(dead_code)]
impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Self {
        Self{t, object}
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        float_compare(self.t, other.t)
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(float_compare(self.t, other.t))
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.t, other.t) && self.object == other.object
    }
}

impl Eq for Intersection {}


#[allow(dead_code)]
pub fn hit(xs: &[Intersection]) -> Option<&Intersection> {

    let mut iter = xs.iter().filter(|x| x.t.is_sign_positive());
    let init = iter.next()?;

    iter
        .try_fold(init, |acc, x| {
            let cmp = x.partial_cmp(acc)?;
            let min = if let Ordering::Less = cmp {
                x
            } else {
                acc
            };
            Some(min)
    })
}
