use std::ops::{Add, Div, Mul, Sub};
use super::utils::is_equal;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn r(self) -> f64 {
        self.r
    }

    pub fn g(self) -> f64 {
        self.g
    }

    pub fn b(self) -> f64 {
        self.b
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            r: self.r / scalar,
            g: self.g / scalar,
            b: self.b / scalar,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.r(), other.r())
            && is_equal(self.g(), other.g())
            && is_equal(self.b(), other.b())
    }
}

impl Eq for Color {}
