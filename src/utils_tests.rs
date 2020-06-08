#[cfg(test)]

mod utils_tests {
    use crate::utils::*;
    use crate::tuple::Tuple;
    use crate::point_light::PointLight;
    use crate::color::Color;
    use crate::generics::Drawables;
    use crate::sphere::Sphere;
    use crate::intersection::Intersection;
    use crate::ray::Ray;


    #[test]
    fn lighting_eye_between_light_and_surface() {
        let sphere = Drawables::Sphere(Sphere::new());
        let position = Tuple::new_point(0., 0., 0.);

        let eye_v = Tuple::new_vector(0., 0., -1.);
        let normal_v = Tuple::new_vector(0., 0., -1.);

        let light = PointLight::new(Tuple::new_point(0., 0., -10.), Color::new(1., 1., 1.)).unwrap();

        let result = lighting(&sphere, position, light, eye_v, normal_v, false).unwrap();
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_offset_45() {
        let sphere = Drawables::Sphere(Sphere::new());
        let position = Tuple::new_point(0., 0., 0.);

        let eye_v = Tuple::new_vector(0., 2_f64.sqrt()/2., -2_f64.sqrt()/2.);
        let normal_v = Tuple::new_vector(0., 0., -1.);

        let light = PointLight::new(Tuple::new_point(0., 0., -10.), Color::new(1., 1., 1.)).unwrap();

        let result = lighting(&sphere, position, light, eye_v, normal_v, false).unwrap();
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_light_offset_45() {
        let sphere = Drawables::Sphere(Sphere::new());
        let position = Tuple::new_point(0., 0., 0.);

        let eye_v = Tuple::new_vector(0., 0., -1.);
        let normal_v = Tuple::new_vector(0., 0., -1.);

        let light = PointLight::new(Tuple::new_point(0., 10., -10.), Color::new(1., 1., 1.)).unwrap();

        let result = lighting(&sphere, position, light, eye_v, normal_v, false).unwrap();
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_at_reflection() {
        let sphere = Drawables::Sphere(Sphere::new());
        let position = Tuple::new_point(0., 0., 0.);

        let eye_v = Tuple::new_vector(0., -2_f64.sqrt()/2., -2_f64.sqrt()/2.);
        let normal_v = Tuple::new_vector(0., 0., -1.);

        let light = PointLight::new(Tuple::new_point(0., 10., -10.), Color::new(1., 1., 1.)).unwrap();

        let result = lighting(&sphere, position, light, eye_v, normal_v, false).unwrap();
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_light_behind_surface() {
        let sphere = Drawables::Sphere(Sphere::new());
        let position = Tuple::new_point(0., 0., 0.);

        let eye_v = Tuple::new_vector(0., 0., -1.);
        let normal_v = Tuple::new_vector(0., 0., -1.);

        let light = PointLight::new(Tuple::new_point(0., 0., 10.), Color::new(1., 1., 1.)).unwrap();

        let result = lighting(&sphere, position, light, eye_v, normal_v, false).unwrap();
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_suface_in_shadow() {
        let sphere = Drawables::Sphere(Sphere::new());
        let position = Tuple::new_point(0., 0., 0.);

        let eye_v = Tuple::new_vector(0., 0., -1.);
        let normal_v = Tuple::new_vector(0., 0., -1.);

        let light = PointLight::new(Tuple::new_point(0., 0., -10.), Color::new(1., 1., 1.)).unwrap();

        let in_shadow = true;
        let result = lighting(&sphere, position, light, eye_v, normal_v, in_shadow).unwrap();
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn schlick_total_internal_reflection() {
        let mut s = Sphere::new();
        s.shape.material.transparency = 1.0;
        s.shape.material.refractive_index = 1.5;

        let shape = Drawables::Sphere(s);

        let r = Ray::new(
            Tuple::new_point(0., 0., -2.0_f64.sqrt()/2.0),
            Tuple::new_vector(0., 1., 0.)
        ).unwrap();

        let i1 = Intersection::new(-2.0_f64.sqrt()/2.0, &shape);
        let i2 = Intersection::new(2.0_f64.sqrt()/2.0, &shape);
        let xs = vec![i1, i2];

        let comps = i2.prepare_computations(r, Some(&xs)).unwrap();

        let reflectance = schlick(comps);

        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn schlick_perpendicular() {
        let mut s = Sphere::new();
        s.shape.material.transparency = 1.0;
        s.shape.material.refractive_index = 1.5;

        let shape = Drawables::Sphere(s);

        let r = Ray::new(
            Tuple::new_point(0., 0., 0.),
            Tuple::new_vector(0., 1., 0.)
        ).unwrap();

        let i1 = Intersection::new(-1., &shape);
        let i2 = Intersection::new(1., &shape);
        let xs = vec![i1, i2];

        let comps = i2.prepare_computations(r, Some(&xs)).unwrap();

        let reflectance = schlick(comps);

        assert_eq!(is_equal(reflectance, 0.04), true);
    }

    #[test]
    fn schlick_small_angle_n2_bigger_than_n1() {
        let mut s = Sphere::new();
        s.shape.material.transparency = 1.0;
        s.shape.material.refractive_index = 1.5;

        let shape = Drawables::Sphere(s);

        let r = Ray::new(
            Tuple::new_point(0., 0.99, -2.),
            Tuple::new_vector(0., 0., 1.)
        ).unwrap();

        let i1 = Intersection::new(1.8589, &shape);
        let xs = vec![i1];

        let comps = i1.prepare_computations(r, Some(&xs)).unwrap();

        let reflectance = schlick(comps);

        assert_eq!(is_equal(reflectance, 0.48873), true);
    }
}
