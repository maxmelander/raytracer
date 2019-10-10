#[cfg(test)]
use super::matrix::*;
use super::tuple::Tuple;

#[test]
fn create_4x4_matrix() {
    let matrix = Matrix4::new(Some([
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5]
    ]));

    assert_eq!(matrix[0][0], 1.0);
    assert_eq!(matrix[0][3], 4.0);
    assert_eq!(matrix[1][0], 5.5);
    assert_eq!(matrix[1][2], 7.5);
}

#[test]
fn create_3x3_matrix() {
    let matrix = Matrix3::new(Some([
        [-3.0, 5.0, 0.0],
        [1.0, -2.0, -7.0],
        [0.0, 1.0, 1.0]
    ]));

    assert_eq!(matrix[0][0], -3.0);
    assert_eq!(matrix[1][1], -2.0);
    assert_eq!(matrix[2][2], 1.0);
}

#[test]
fn create_2x2_matrix() {
    let matrix = Matrix2::new(Some([
        [-3.0, 5.0],
        [1.0, -2.0],
    ]));

    assert_eq!(matrix[0][0], -3.0);
    assert_eq!(matrix[1][1], -2.0);
    assert_eq!(matrix[1][0], 1.0);
    assert_eq!(matrix[0][1], 5.0);
}

#[test]
fn mul_matrices() {
    let a = Matrix4::new(Some([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0]
    ]));

    let b = Matrix4::new(Some([
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0]
    ]));

    let expected = Matrix4::new(Some([
        [20., 22., 50., 48.],
        [44., 54., 114., 108.],
        [40., 58., 110., 102.],
        [16., 26., 46., 42.]
    ]));

    assert_eq!(a * b, expected);
}

#[test]
fn mul_matrix_by_tuple() {
    let a = Matrix4::new(Some([
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.]
    ]));

    let b = Tuple::new_point(1.0, 2.0, 3.0);

    let expected = Tuple::new_point(18., 24., 33.);

    assert_eq!(a * b, expected);
}
