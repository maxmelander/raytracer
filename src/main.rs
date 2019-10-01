const EPSILON: f64 = 0.00001;

mod tuple {
    use super::EPSILON;
    use std::ops::{Add, Sub};

    pub fn is_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    pub enum TupleType {
        Point,
        Vector,
        Error
    }

    #[derive(Debug)]
    pub struct Tuple {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }

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
                TupleType::Error
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

    impl PartialEq for Tuple {
        fn eq(&self, other: &Self) -> bool {
            (is_equal(self.x(), other.x())
             && is_equal(self.y(), other.y())
             && is_equal(self.z(), other.z())
             && is_equal(self.w(), other.w()))
        }
    }

    impl Eq for Tuple {}
}



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::tuple::*;
    #[test]
    fn create_point() -> Result<(), String> {
        let point = Tuple::new_point(4.0, -4.0, 3.0);
        assert_eq!(point.w(), 1.0);
        if let TupleType::Point = point.get_type() {
            Ok(())
        } else {
            Err(String::from("The tuple type was not expected"))
        }
    }

    #[test]
    fn create_vector() -> Result<(), String> {
        let vector = Tuple::new_vector(4.0, -4.0, 3.0);
        assert_eq!(vector.w(), 0.0);
        if let TupleType::Vector = vector.get_type() {
            Ok(())
        } else {
            Err(String::from("The tuple type was not expected"))
        }
    }

    #[test]
    fn is_equal(){
        let a = Tuple::new_point(1.0, 2.0, -3.0);
        let b = Tuple::new_point(1.0000001, 2.0, -3.0);
        assert!(a == b);
        assert!(b == a);
    }

    #[test]
    fn is_not_equal() {
        let a = Tuple::new_vector(1.0, 1.0, 1.0);
        let b = Tuple::new_point(1.0, 1.0, 1.0);
        assert!(a != b);
        assert!(b != a);
    }

    #[test]
    fn addition() {
        let a = Tuple::new_point(3.0, -2.0, 5.0);
        let b = Tuple::new_vector(-2.0, 3.0, 1.0);
        let expected = Tuple::new_point(1.0, 1.0, 6.0);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtract_two_points() {
        let a = Tuple::new_point(3., 2., 1.);
        let b = Tuple::new_point(5., 6., 7.);
        let expected = Tuple::new_vector(-2., -4., -6.);
        assert_eq!(a - b, expected);
    }

    #[test]
    fn subtract_vector_from_point() {
        let a = Tuple::new_point(3., 2., 1.);
        let b = Tuple::new_vector(5., 6., 7.);
        let expected = Tuple::new_point(-2., -4., -6.);
        assert_eq!(a - b, expected);
    }

    #[test]
    fn subtract_two_vectors() {
        let a = Tuple::new_vector(3., 2., 1.);
        let b = Tuple::new_vector(5., 6., 7.);
        let expected = Tuple::new_vector(-2., -4., -6.);
        assert_eq!(a - b, expected);
    }
}
