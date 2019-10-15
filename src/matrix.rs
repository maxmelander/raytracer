const EPSILON: f64 = 0.00001;

use std::ops::{Add, Sub, Neg, Mul, Div, Index, IndexMut};
use super::tuple::Tuple;

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

    pub fn new_identity() -> Self {
        Self{data: [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.]
        ]}
    }

    pub fn transpose(self) -> Self {
        Self{data: [
            [self[0][0], self[1][0], self[2][0], self[3][0]],
            [self[0][1], self[1][1], self[2][1], self[3][1]],
            [self[0][2], self[1][2], self[2][2], self[3][2]],
            [self[0][3], self[1][3], self[2][3], self[3][3]],
        ]}
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let mut values = [0.0; 9];
        let mut index = 0;
        for r in 0..4 {
            if r == row {continue;}
            for c in 0..4 {
                if c == col {continue;}
                values[index] = self[r][c];
                index += 1;
            }
        }

        Matrix3::new(Some([
            [values[0], values[1], values[2]],
            [values[3], values[4], values[5]],
            [values[6], values[7], values[8]],
        ]))
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let m3 = self.submatrix(row, col);
        m3.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col).checked_rem(2) == Some(0) {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for c in 0..4 {
            result += self[0][c] * self.cofactor(0, c);
        }
        result
    }

    pub fn inverse(&self) -> Option<Self>{
        let determinant = self.determinant();
        if determinant == 0.0 {
            None
        } else {
            Some(Matrix4::new(Some([
                [self.cofactor(0,0), self.cofactor(1,0), self.cofactor(2,0), self.cofactor(3,0)],
                [self.cofactor(0,1), self.cofactor(1,1), self.cofactor(2,1), self.cofactor(3,1)],
                [self.cofactor(0,2), self.cofactor(1,2), self.cofactor(2,2), self.cofactor(3,2)],
                [self.cofactor(0,3), self.cofactor(1,3), self.cofactor(2,3), self.cofactor(3,3)],
            ])) / determinant)
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

    // TODO: There must be a nicer way of doing this
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut values = [0.0; 4];
        let mut index = 0;
        for r in 0..3 {
            if r == row {continue;}
            for c in 0..3 {
                if c == col {continue;}
                values[index] = self[r][c];
                index += 1;
            }
        }

        Matrix2::new(Some([
            [values[0], values[1]],
            [values[2], values[3]],
        ]))
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let m2 = self.submatrix(row, col);
        m2.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col).checked_rem(2) == Some(0) {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for c in 0..3 {
            result += self[0][c] * self.cofactor(0, c);
        }
        result
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

    pub fn determinant(&self) -> f64 {
        (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut [f64; 4] {
        &mut self.data[index]
    }
}

impl Index<usize> for Matrix3 {
    type Output = [f64; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut [f64; 3] {
        &mut self.data[index]
    }
}

impl Index<usize> for Matrix2 {
    type Output = [f64; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, index: usize) -> &mut [f64; 2] {
        &mut self.data[index]
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

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let mut result = Tuple::new_point(0.0, 0.0, 0.0);
        for row in 0..4 {
            let value = 
                (self[row][0] * other.x()) +
                (self[row][1] * other.y()) +
                (self[row][2] * other.z()) +
                (self[row][3] * other.w());

            match row {
                0 => result.x = value,
                1 => result.y = value,
                2 => result.z = value,
                3 => result.w = value,
                _ => ()
            }
        }
        result
    }
}

impl Div<f64> for Matrix4 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self::new(Some([
            [self[0][0] / other, self[0][1] / other, self[0][2] / other, self[0][3] / other, ],
            [self[1][0] / other, self[1][1] / other, self[1][2] / other, self[1][3] / other, ],
            [self[2][0] / other, self[2][1] / other, self[2][2] / other, self[2][3] / other, ],
            [self[3][0] / other, self[3][1] / other, self[3][2] / other, self[3][3] / other, ],
        ]))
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if !is_equal(self[row][col], other[row][col]) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Eq for Matrix4 {}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if !is_equal(self[row][col], other[row][col]) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Eq for Matrix3 {}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..2 {
            for col in 0..2 {
                if !is_equal(self[row][col], other[row][col]) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Eq for Matrix2 {}
