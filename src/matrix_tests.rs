#[cfg(test)]
use super::matrix::*;
use super::tuple::Tuple;
use std::f64::consts::PI;

#[test]
fn create_4x4_matrix() {
    let matrix = Matrix4::new(Some([
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
    ]));

    assert_eq!(matrix[0][0], 1.0);
    assert_eq!(matrix[0][3], 4.0);
    assert_eq!(matrix[1][0], 5.5);
    assert_eq!(matrix[1][2], 7.5);
}

#[test]
fn create_3x3_matrix() {
    let matrix = Matrix3::new(Some([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]));

    assert_eq!(matrix[0][0], -3.0);
    assert_eq!(matrix[1][1], -2.0);
    assert_eq!(matrix[2][2], 1.0);
}

#[test]
fn create_2x2_matrix() {
    let matrix = Matrix2::new(Some([[-3.0, 5.0], [1.0, -2.0]]));

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
        [5.0, 4.0, 3.0, 2.0],
    ]));

    let b = Matrix4::new(Some([
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]));

    let expected = Matrix4::new(Some([
        [20., 22., 50., 48.],
        [44., 54., 114., 108.],
        [40., 58., 110., 102.],
        [16., 26., 46., 42.],
    ]));

    assert_eq!(a * b, expected);
}

#[test]
fn mul_matrix_by_tuple() {
    let a = Matrix4::new(Some([
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    ]));

    let b = Tuple::new_point(1.0, 2.0, 3.0);

    let expected = Tuple::new_point(18., 24., 33.);

    assert_eq!(a * b, expected);
}

#[test]
fn mul_matrix_by_identity() {
    let a = Matrix4::new(Some([
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    ]));

    let expected = Matrix4::new(Some([
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    ]));

    assert_eq!(a * Matrix4::new_identity(), expected);
}

#[test]
fn mul_identity_by_tuple() {
    let a = Tuple::new_point(1.0, 2.0, 3.0);
    let expected = Tuple::new_point(1.0, 2.0, 3.0);
    assert_eq!(Matrix4::new_identity() * a, expected);
}

#[test]
fn transpose() {
    let a = Matrix4::new(Some([
        [0., 9., 3., 0.],
        [9., 8., 0., 8.],
        [1., 8., 5., 3.],
        [0., 0., 5., 8.],
    ]));

    let expected = Matrix4::new(Some([
        [0., 9., 1., 0.],
        [9., 8., 8., 0.],
        [3., 0., 5., 5.],
        [0., 8., 3., 8.],
    ]));

    assert_eq!(a.transpose(), expected);
}

#[test]
fn transpose_identity() {
    assert_eq!(Matrix4::new_identity().transpose(), Matrix4::new_identity());
}

#[test]
fn determinant_matrix2() {
    let a = Matrix2::new(Some([[1., 5.], [-3., 2.]]));

    assert_eq!(a.determinant(), 17.);
}

#[test]
fn submatrix_3_2() {
    let a = Matrix3::new(Some([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]));

    let expected = Matrix2::new(Some([[-3., 2.], [0., 6.]]));

    assert_eq!(a.submatrix(0, 2), expected);
}

#[test]
fn submatrix_4_3() {
    let a = Matrix4::new(Some([
        [-6., 1., 1., 6.],
        [-8., 5., 8., 6.],
        [-1., 0., 8., 2.],
        [-7., 1., -1., 1.],
    ]));

    let expected = Matrix3::new(Some([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]));

    assert_eq!(a.submatrix(2, 1), expected);
}

#[test]
fn minor() {
    let a = Matrix3::new(Some([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]));

    let b = a.submatrix(1, 0);

    assert_eq!(b.determinant(), 25.);
    assert_eq!(a.minor(1, 0), 25.);
}

#[test]
fn cofactor() {
    let a = Matrix3::new(Some([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]));

    assert_eq!(a.minor(0, 0), -12.);
    assert_eq!(a.cofactor(0, 0), -12.);
    assert_eq!(a.minor(1, 0), 25.);
    assert_eq!(a.cofactor(1, 0), -25.);
}

#[test]
fn determinant_3() {
    let a = Matrix3::new(Some([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]));

    assert_eq!(a.cofactor(0, 0), 56.);
    assert_eq!(a.cofactor(0, 1), 12.);
    assert_eq!(a.cofactor(0, 2), -46.);
    assert_eq!(a.determinant(), -196.);
}

#[test]
fn determinant_4() {
    let a = Matrix4::new(Some([
        [-2., -8., 3., 5.],
        [-3., 1., 7., 3.],
        [1., 2., -9., 6.],
        [-6., 7., 7., -9.],
    ]));

    assert_eq!(a.cofactor(0, 0), 690.);
    assert_eq!(a.cofactor(0, 1), 447.);
    assert_eq!(a.cofactor(0, 2), 210.);
    assert_eq!(a.cofactor(0, 3), 51.);
    assert_eq!(a.determinant(), -4071.);
}

#[test]
fn inverse() {
    let a = Matrix4::new(Some([
        [-5., 2., 6., -8.],
        [1., -5., 1., 8.],
        [7., 7., -6., -7.],
        [1., -3., 7., 4.],
    ]));

    let b = Matrix4::new(Some([
        [-4., 2., -2., -3.],
        [9., 6., 2., 6.],
        [0., -5., 1., -5.],
        [0., 0., 0., 0.],
    ]));

    let expected = Matrix4::new(Some([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]));

    let result = a.inverse().unwrap();

    assert_eq!(a.determinant(), 532.);
    assert_eq!(a.cofactor(2, 3), -160.);
    assert_eq!(result[3][2], -160. / 532.);
    assert_eq!(a.cofactor(3, 2), 105.);
    assert_eq!(result[2][3], 105. / 532.);

    assert_eq!(result, expected);
    assert_eq!(b.inverse(), None);
}

#[test]
fn inverse_2() {
    let a = Matrix4::new(Some([
        [8., -5., 9., 2.],
        [7., 5., 6., 1.],
        [-6., 0., 9., 6.],
        [-3., 0., -9., -4.],
    ]));

    let expected = Matrix4::new(Some([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]));

    assert_eq!(a.inverse(), Some(expected));
}

#[test]
fn inverse_mul() {
    let a = Matrix4::new(Some([
        [3., -9., 7., 3.],
        [3., -8., 2., -9.],
        [-4., 4., 4., 1.],
        [-6., 5., -1., 1.],
    ]));

    let b = Matrix4::new(Some([
        [8., 2., 2., 2.],
        [3., -1., 7., 0.],
        [7., 0., 5., 4.],
        [6., -2., 0., 5.],
    ]));

    let c = a * b;

    assert_eq!(c * b.inverse().unwrap(), a);
}

#[test]
fn translation_mul() {
    let p = Tuple::new_point(-3.0, 4.0, 5.0);
    let transform = Matrix4::new_translation(5.0, -3.0, 2.0);
    let expected = Tuple::new_point(2.0, 1.0, 7.0);

    assert_eq!(transform * p, expected);
}

#[test]
fn translation_inverse_mul() {
    let p = Tuple::new_point(-3.0, 4.0, 5.0);
    let transform = Matrix4::new_translation(5.0, -3.0, 2.0);
    let i_trans = transform.inverse().unwrap();
    let expected = Tuple::new_point(-8.0, 7.0, 3.0);

    assert_eq!(i_trans * p, expected);
}

#[test]
fn translation_vec_mul() {
    let v = Tuple::new_vector(-3.0, 4.0, 5.0);
    let transform = Matrix4::new_translation(5.0, -3.0, 2.0);

    assert_eq!(transform * v, v);
}

#[test]
fn scaling_point() {
    let p = Tuple::new_point(-4., 6., 8.);
    let transform = Matrix4::new_scaling(2., 3., 4.);
    let expected = Tuple::new_point(-8., 18., 32.);

    assert_eq!(transform * p, expected);
}

#[test]
fn scaling_vector() {
    let p = Tuple::new_vector(-4., 6., 8.);
    let transform = Matrix4::new_scaling(2., 3., 4.);
    let expected = Tuple::new_vector(-8., 18., 32.);

    assert_eq!(transform * p, expected);
}

#[test]
fn scaling_inverse_vector() {
    let p = Tuple::new_vector(-4., 6., 8.);
    let transform = Matrix4::new_scaling(2., 3., 4.);
    let i_trans = transform.inverse().unwrap();
    let expected = Tuple::new_vector(-2., 2., 2.);

    assert_eq!(i_trans * p, expected);
}

#[test]
fn scaling_reflection() {
    let p = Tuple::new_point(2., 3., 4.);
    let transform = Matrix4::new_scaling(-1., 1., 1.);
    let expected = Tuple::new_point(-2., 3., 4.);

    assert_eq!(transform * p, expected);
}

#[test]
fn rotate_x_point() {
    let p = Tuple::new_point(0., 1., 0.);
    let half_quarter = Matrix4::new_rotation_x(PI / 4.);
    let full_quarter = Matrix4::new_rotation_x(PI / 2.);

    let expected_half = Tuple::new_point(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.);
    let expected_full = Tuple::new_point(0., 0., 1.);

    assert_eq!(half_quarter * p, expected_half);
    assert_eq!(full_quarter * p, expected_full);
}

#[test]
fn rotate_x_inverse_point() {
    let p = Tuple::new_point(0., 1., 0.);
    let half_quarter = Matrix4::new_rotation_x(PI / 4.);
    let i_half_quarter = half_quarter.inverse().unwrap();

    let expected_half = Tuple::new_point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);

    assert_eq!(i_half_quarter * p, expected_half);
}

#[test]
fn rotate_y_point() {
    let p = Tuple::new_point(0., 0., 1.);
    let half_quarter = Matrix4::new_rotation_y(PI / 4.);
    let full_quarter = Matrix4::new_rotation_y(PI / 2.);

    let expected_half = Tuple::new_point(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.);
    let expected_full = Tuple::new_point(1., 0., 0.);

    assert_eq!(half_quarter * p, expected_half);
    assert_eq!(full_quarter * p, expected_full);
}

#[test]
fn rotate_z_point() {
    let p = Tuple::new_point(0., 1., 0.);
    let half_quarter = Matrix4::new_rotation_z(PI / 4.);
    let full_quarter = Matrix4::new_rotation_z(PI / 2.);

    let expected_half = Tuple::new_point(-2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.);
    let expected_full = Tuple::new_point(-1., 0., 0.);

    assert_eq!(half_quarter * p, expected_half);
    assert_eq!(full_quarter * p, expected_full);
}

#[test]
fn shearing_x_y() {
    let transform = Matrix4::new_shearing(1., 0., 0., 0., 0., 0.);
    let p = Tuple::new_point(2., 3., 4.);
    let expected = Tuple::new_point(5., 3., 4.);

    assert_eq!(transform * p, expected);
}

#[test]
fn shearing_x_z() {
    let transform = Matrix4::new_shearing(0., 1., 0., 0., 0., 0.);
    let p = Tuple::new_point(2., 3., 4.);
    let expected = Tuple::new_point(6., 3., 4.);

    assert_eq!(transform * p, expected);
}

#[test]
fn shearing_y_x() {
    let transform = Matrix4::new_shearing(0., 0., 1., 0., 0., 0.);
    let p = Tuple::new_point(2., 3., 4.);
    let expected = Tuple::new_point(2., 5., 4.);

    assert_eq!(transform * p, expected);
}

#[test]
fn shearing_y_z() {
    let transform = Matrix4::new_shearing(0., 0., 0., 1., 0., 0.);
    let p = Tuple::new_point(2., 3., 4.);
    let expected = Tuple::new_point(2., 7., 4.);

    assert_eq!(transform * p, expected);
}

#[test]
fn shearing_z_x() {
    let transform = Matrix4::new_shearing(0., 0., 0., 0., 1., 0.);
    let p = Tuple::new_point(2., 3., 4.);
    let expected = Tuple::new_point(2., 3., 6.);

    assert_eq!(transform * p, expected);
}

#[test]
fn shearing_z_y() {
    let transform = Matrix4::new_shearing(0., 0., 0., 0., 0., 1.);
    let p = Tuple::new_point(2., 3., 4.);
    let expected = Tuple::new_point(2., 3., 7.);

    assert_eq!(transform * p, expected);
}

#[test]
fn trans_sequence() {
    let a = Matrix4::new_rotation_x(PI / 2.);
    let b = Matrix4::new_scaling(5., 5., 5.);
    let c = Matrix4::new_translation(10., 5., 7.);
    let p = Tuple::new_point(1., 0., 1.);

    // Apply rotation first
    let p2 = a * p;
    assert_eq!(p2, Tuple::new_point(1., -1., 0.,));

    // Then scaling
    let p3 = b * p2;
    assert_eq!(p3, Tuple::new_point(5., -5., 0.));

    // Then translation
    let p4 = c * p3;
    assert_eq!(p4, Tuple::new_point(15., 0., 7.));
}

#[test]
fn trans_chain() {
    let a = Matrix4::new_rotation_x(PI / 2.);
    let b = Matrix4::new_scaling(5., 5., 5.);
    let c = Matrix4::new_translation(10., 5., 7.);
    let p = Tuple::new_point(1., 0., 1.);

    let t = c * b * a;
    assert_eq!(t * p, Tuple::new_point(15., 0., 7.));
}

#[test]
fn default_orientation() {
    let from = Tuple::new_point(0., 0., 0.);
    let to = Tuple::new_point(0., 0., -1.);
    let up = Tuple::new_vector(0., 1., 0.);

    let t = Matrix4::new_view_transform(from, to, up);
    assert_eq!(t, Matrix4::new_identity());
}

#[test]
fn view_transform_positive_z() {
    let from = Tuple::new_point(0., 0., 0.);
    let to = Tuple::new_point(0., 0., 1.);
    let up = Tuple::new_vector(0., 1., 0.);

    let t = Matrix4::new_view_transform(from, to, up);
    assert_eq!(t, Matrix4::new_scaling(-1., 1., -1.));
}

#[test]
fn view_transform_moves_world() {
    let from = Tuple::new_point(0., 0., 8.);
    let to = Tuple::new_point(0., 0., 0.);
    let up = Tuple::new_vector(0., 1., 0.);

    let t = Matrix4::new_view_transform(from, to, up);
    assert_eq!(t, Matrix4::new_translation(0., 0., -8.));
}

#[test]
fn view_transform_arbitrary() {
    let from = Tuple::new_point(1., 3., 2.);
    let to = Tuple::new_point(4., -2., 8.);
    let up = Tuple::new_vector(1., 1., 0.);

    let t = Matrix4::new_view_transform(from, to, up);
    let m = Matrix4::new(Some([
        [-0.50709, 0.50709, 0.67612, -2.36643],
        [0.76772, 0.60609, 0.12122, -2.82843],
        [-0.35857, 0.59761, -0.71714, 0.00000],
        [0.00000, 0.00000, 0.00000, 1.00000],
    ]));
    assert_eq!(t, m);
}
