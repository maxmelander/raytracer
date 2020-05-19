const EPSILON: f64 = 0.00001;

use std::ops::{Add, Div, Mul, Neg, Sub};

pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug)]
pub enum TupleType {
    Point,
    Vector,
    Other,
}

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[allow(dead_code)]
impl Tuple {
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn is_point(&self) -> bool {
        if let TupleType::Point = self.get_type() {
            return true;
        }
        false
    }

    pub fn is_vector(&self) -> bool {
        if let TupleType::Vector = self.get_type() {
            return true;
        }
        false
    }

    pub fn get_type(&self) -> TupleType {
        if (self.w - 0.0).abs() < EPSILON {
            TupleType::Vector
        } else if (self.w - 1.0).abs() < EPSILON {
            TupleType::Point
        } else {
            TupleType::Other
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }

    //TODO: Is it better to mutate self then to return a new Tuple?
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn reflect(&self, normal: Tuple) -> Self {
        *self - normal * 2.0 * self.dot(&normal)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (is_equal(self.x, other.x)
            && is_equal(self.y, other.y)
            && is_equal(self.z, other.z)
            && is_equal(self.w, other.w))
    }
}

impl Eq for Tuple {}
