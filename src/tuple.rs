const EPSILON: f64 = 0.00001;

use std::ops::{Add, Sub, Neg, Mul, Div};

pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug)]
pub enum TupleType {
    Point,
    Vector,
    Other
}

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[allow(dead_code)]
impl Tuple {
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self{x: x, y: y, z: z, w: 1.0}
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self{x: x, y: y, z: z, w: 0.0}
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

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn magnitude(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0
        }
    }

    //TODO: Is it better to mutate self then to return a new Tuple?
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
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
            w: self.w - other.w
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
            w: -self.w
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
            w: self.w * scalar
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
            w: self.w / scalar
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (is_equal(self.x(), other.x())
         && is_equal(self.y(), other.y())
         && is_equal(self.z(), other.z())
         && is_equal(self.w(), other.w()))
    }
}

impl Eq for Tuple {}
