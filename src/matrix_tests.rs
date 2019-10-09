#[cfg(test)]
use super::matrix::*;

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
