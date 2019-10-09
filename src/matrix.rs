const EPSILON: f64 = 0.00001;

use std::ops::{Add, Sub, Neg, Mul, Div, Index, IndexMut};

pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    data: [[f64; 4]; 4]
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix3 {
    data: [[f64; 3]; 3]
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix2 {
    data: [[f64; 2]; 2]
}

#[allow(dead_code)]
impl Matrix4 {
    pub fn new(data: Option<[[f64; 4]; 4]>) -> Self {
        match data {
            Some(data) => Self{data: data},
            None => Self{data: [[0.0; 4]; 4]}
        }
    }
}

#[allow(dead_code)]
impl Matrix3 {
    pub fn new(data: Option<[[f64; 3]; 3]>) -> Self {
        match data {
            Some(data) => Self{data: data},
            None => Self{data: [[0.0; 3]; 3]}
        }
    }
}

#[allow(dead_code)]
impl Matrix2 {
    pub fn new(data: Option<[[f64; 2]; 2]>) -> Self {
        match data {
            Some(data) => Self{data: data},
            None => Self{data: [[0.0; 2]; 2]}
        }
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<usize> for Matrix3 {
    type Output = [f64; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<usize> for Matrix2 {
    type Output = [f64; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Mul for Matrix4 {
    type Output = Self;


    fn mul(self, other: Self) -> Self {
        let mut result = Self::new(None);

        for row in 0..4 {
            for col in 0..4 {
                result[row][col] =
                    (self[row][0] * other[0][col]) +
                    (self[row][1] * other[1][col]) +
                    (self[row][2] * other[2][col]) +
                    (self[row][3] * other[3][col]);
            }
        }
        result
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if self[row][col] != other[row][col] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Eq for Matrix4 {}

