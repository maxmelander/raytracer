#[cfg(test)]

mod patterns_tests {
    use crate::patterns::{Patterns, Pattern};
    use crate::color::Color;
    use crate::tuple::Tuple;
    use crate::material::Material;
    use crate::point_light::PointLight;
    use crate::utils::lighting;
    use crate::sphere::Sphere;
    use crate::matrix::Matrix4;
    use crate::generics::Drawables;

    const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.
    };

    const WHITE: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.
    };

    // TODO: Check that default pattern has identity matrix as transform

    #[test]
    fn create_stripe_pattern() {
        let pattern = Patterns::new_stripe(WHITE, BLACK);
        assert_eq!(pattern.get_a(), WHITE);
        assert_eq!(pattern.get_b(), BLACK);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = Patterns::new_stripe(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 1., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 2., 0.)), WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = Patterns::new_stripe(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 1.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 2.)), WHITE);
    }

    #[test]
    fn stripe_pattern_alternate_in_x() {
        let pattern = Patterns::new_stripe(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(-0.1, 0., 0.)), BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(-1., 0., 0.)), BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(-1.1, 0., 0.)), WHITE);
    }

    #[test]
    fn lighting_with_pattern() {
        let material = Material{
            pattern: Some(Patterns::new_stripe(WHITE, BLACK)),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        };

        let mut sphere = Sphere::new();
        sphere.shape.material = material;
        let object = Drawables::Sphere(sphere);

        let eye_v = Tuple::new_vector(0., 0., -1.);
        let normal_v = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Tuple::new_point(0., 0., -10.), WHITE).unwrap();

        let c1 = lighting(&object, Tuple::new_point(0.9, 0., 0.), light, eye_v, normal_v, false).unwrap();
        let c2 = lighting(&object, Tuple::new_point(1.1, 0., 0.), light, eye_v, normal_v, false).unwrap();

        assert_eq!(c1, WHITE);
        assert_eq!(c2, BLACK);
    }

    #[test]
    fn stripe_with_object_transformation() {
        let object = Drawables::Sphere(Sphere::new_with_transform(Matrix4::new_scaling(2., 2., 2.)));
        let pattern = Patterns::new_stripe(WHITE, BLACK);

        let c = pattern.color_at_object(&object, Tuple::new_point(1.5, 0., 0.)).unwrap();
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripe_with_pattern_transformation() {
        let object = Drawables::Sphere(Sphere::new());
        let mut pattern = Patterns::new_stripe(WHITE, BLACK);
        pattern.set_transform(Matrix4::new_scaling(2., 2., 2.));

        let c = pattern.color_at_object(&object, Tuple::new_point(1.5, 0., 0.)).unwrap();
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripe_with_object_and_pattern_transformation() {
        let object = Drawables::Sphere(Sphere::new_with_transform(Matrix4::new_scaling(2., 2., 2.)));
        let mut pattern = Patterns::new_stripe(WHITE, BLACK);
        pattern.set_transform(Matrix4::new_translation(0.5, 0., 0.));

        let c = pattern.color_at_object(&object, Tuple::new_point(2.5, 0., 0.)).unwrap();
        assert_eq!(c, WHITE);
    }

    #[test]
    fn gradient_pattern_linear_interpol() {
        let pattern = Patterns::new_gradient(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0.25, 0., 0.)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.color_at(Tuple::new_point(0.5, 0., 0.)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.color_at(Tuple::new_point(0.75, 0., 0.)), Color::new(0.25, 0.25, 0.25));
    }

    #[test]
    fn ring_pattern() {
        let pattern = Patterns::new_ring(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 1.)), BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0.708, 0., 0.708)), BLACK);
    }

    #[test]
    fn checker_repeat_x() {
        let pattern = Patterns::new_checker(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0.99, 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(1.01, 0., 0.)), BLACK);
    }

    #[test]
    fn checker_repeat_y() {
        let pattern = Patterns::new_checker(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0.99, 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 1.01, 0.)), BLACK);
    }

    #[test]
    fn checker_repeat_z() {
        let pattern = Patterns::new_checker(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 0.99)), WHITE);
        assert_eq!(pattern.color_at(Tuple::new_point(0., 0., 1.01)), BLACK);
    }
}
