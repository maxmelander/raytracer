#[cfg(test)]

mod point_light_tests {
    use crate::point_light::*;
    use crate::color::*;
    use crate::tuple::Tuple;

    #[test]
    fn point_light_with_position_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = Tuple::new_point(0., 0., 0.);
        let light = PointLight::new(position, intensity).unwrap();

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
