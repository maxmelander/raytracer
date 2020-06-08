#[cfg(test)]

mod world_tests {
    use crate::world::World;
    use crate::point_light::PointLight;
    use crate::tuple::Tuple;
    use crate::color::Color;
    use crate::sphere::Sphere;
    use crate::plane::Plane;
    use crate::material::Material;
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::intersection::Intersection;
    use crate::generics::Drawables;
    use crate::patterns::Patterns;
    use crate::utils::EPSILON;

    #[test]
    fn default_world() {
        let light = PointLight::new(Tuple::new_point(-10., 10., -10.), Color::new(1., 1., 1.)).unwrap();
        let mut s1 = Sphere::new();
        s1.shape.material = Material{
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };

        let s2 = Sphere::new_with_transform(Matrix4::new_scaling(0.5, 0.5, 0.5));
        let w: World = Default::default();

        assert_eq!(w.lights[0], light);
        assert_eq!(w.objects.contains(&Drawables::Sphere(s1)), true);
        assert_eq!(w.objects.contains(&Drawables::Sphere(s2)), true);
    }

    #[test]
    fn shade_intersection() {
        let w: World = Default::default();
        let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

        let shape = w.objects[0];
        let i = Intersection {
            t: 4.0,
            object: &shape
        };

        let comps = i.prepare_computations(r, None).unwrap();
        let c = w.shade_hit(comps, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_intersection_inside() {
        let w = World{
            lights: vec![PointLight::new(Tuple::new_point(0., 0.25, 0.), Color::new(1., 1., 1.)).unwrap()],
            ..Default::default()
        };

        let r = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.)).unwrap();

        let shape = w.objects[1];
        let i = Intersection {
            t: 0.5,
            object: &shape
        };

        let comps = i.prepare_computations(r, None).unwrap();
        let c = w.shade_hit(comps, 1);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_ray_miss() {
        let w: World = Default::default();
        let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 1., 0.)).unwrap();

        let c = w.color_at(r, 1);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn color_ray_hit() {
        let w: World = Default::default();
        let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();

        let c = w.color_at(r, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_ray_intersect_behind() {
        let material = Material {
            ambient: 1.0,
            ..Default::default()
        };

        let mut outer = Sphere::new();
        outer.shape.material = material;

        let mut inner = Sphere::new();
        inner.shape.material = material;


        let w = World {
            objects: vec![Drawables::Sphere(outer), Drawables::Sphere(inner)],
            ..Default::default()
        };

        let r = Ray::new(Tuple::new_point(0., 0., 0.75), Tuple::new_vector(0., 0., -1.)).unwrap();

        let c = w.color_at(r, 1);
        assert_eq!(c, inner.shape.material.color);
    }

    #[test]
    fn no_shadow() {
        let w: World = Default::default();
        let p = Tuple::new_point(0., 10., 0.);

        assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
    }

    #[test]
    fn shadow_object_between_point_and_light() {
        let w: World = Default::default();
        let p = Tuple::new_point(10., -10., 10.);

        assert_eq!(w.is_shadowed(p, &w.lights[0]), true);
    }

    #[test]
    fn no_shadow_object_behing_light() {
        let w: World = Default::default();
        let p = Tuple::new_point(-20., 20., -20.);

        assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
    }

    #[test]
    fn no_shadow_object_behind_point() {
        let w: World = Default::default();
        let p = Tuple::new_point(-2., 2., -2.);

        assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
    }

    #[test]
    fn shade_hit_intersection_in_shadow() {
        let s1 = Drawables::Sphere(Sphere::new());
        let s2 = Drawables::Sphere(Sphere::new_with_transform(Matrix4::new_translation(0., 0., 10.)));
        let w = World {
            lights: vec![PointLight::new(Tuple::new_point(0., 0., -10.), Color::new(1., 1., 1.)).unwrap()],
            objects: vec![s1, s2]
        };

        let r = Ray::new(Tuple::new_point(0., 0., 5.), Tuple::new_vector(0., 0., 1.)).unwrap();
        let i = Intersection::new(4., &s2);
        let comps = i.prepare_computations(r, None).unwrap();

        let c = w.shade_hit(comps, 1);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn hit_should_offset() {
        let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.)).unwrap();
        let s = Drawables::Sphere(Sphere::new_with_transform(Matrix4::new_translation(0., 0., 1.)));
        let i = Intersection::new(5., &s);
        let comps = i.prepare_computations(r, None).unwrap();

        assert_eq!(comps.over_point.z < -EPSILON / 2.0, true);
        assert_eq!(comps.point.z > comps.over_point.z, true);


    }

    #[test]
    fn color_of_non_reflective_mat() {
        let w: World = Default::default();
        let r = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.)).unwrap();
        let mut sphere = w.objects[0];

        let i = Intersection::new(1., &sphere);

        let comps = i.prepare_computations(r, None).unwrap();
        let color = w.reflected_color(comps, 1);
        assert_eq!(color, Ok(Color::new(0., 0., 0.)));
    }

    #[test]
    fn color_of_reflective_mat() {
        let mut w: World = Default::default();

        let mut plane = Plane::new_with_transform(Matrix4::new_translation(0., -1., 0.));
        plane.shape.material = Material {
            reflective: 0.5,
            ..Default::default()
        };
        let mut shape = Drawables::Plane(plane);

        w.objects.push(shape);

        let r = Ray::new(
            Tuple::new_point(0., 0., -3.),
            Tuple::new_vector(0., -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        ).unwrap();

        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(r, None).unwrap();
        let color = w.reflected_color(comps, 1);
        assert_eq!(color, Ok(Color::new(0.1903322, 0.23791525, 0.142749151)));
    }

    #[test]
    fn shade_hit_with_reflective_mat() {
        let mut w: World = Default::default();

        let mut plane = Plane::new_with_transform(Matrix4::new_translation(0., -1., 0.));
        plane.shape.material = Material {
            reflective: 0.5,
            ..Default::default()
        };
        let mut shape = Drawables::Plane(plane);

        w.objects.push(shape);

        let r = Ray::new(
            Tuple::new_point(0., 0., -3.),
            Tuple::new_vector(0., -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        ).unwrap();

        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(r, None).unwrap();
        let color = w.shade_hit(comps, 1);
        assert_eq!(color, Color::new(0.876757, 0.9243403, 0.8291742));
    }

    #[test]
    fn shade_hit_with_mutually_reflective_surfaces() {
        let mut w: World = Default::default();

        let light = PointLight::new(Tuple::new_point(0., 0., 0.), Color::new(1., 1., 1.)).unwrap();
        w.lights = vec![light];

        let mut lower = Plane::new_with_transform(Matrix4::new_translation(0., -1., 0.));
        lower.shape.material = Material {
            reflective: 1.0,
            ..Default::default()
        };
        let mut lower_shape = Drawables::Plane(lower);

        let mut upper = Plane::new_with_transform(Matrix4::new_translation(0., 1., 0.));
        upper.shape.material = Material {
            reflective: 1.0,
            ..Default::default()
        };
        let mut upper_shape = Drawables::Plane(upper);

        w.objects = vec![upper_shape, lower_shape];

        let r = Ray::new(
            Tuple::new_point(0., 0., 0.),
            Tuple::new_vector(0., 1., 0.)
        ).unwrap();

        let color = w.color_at(r, 4);
    }

    #[test]
    fn limit_recursion() {
        let mut w: World = Default::default();

        let mut plane = Plane::new_with_transform(Matrix4::new_translation(0., -1., 0.));
        plane.shape.material = Material {
            reflective: 0.5,
            ..Default::default()
        };
        let mut shape = Drawables::Plane(plane);

        w.objects.push(shape);

        let r = Ray::new(
            Tuple::new_point(0., 0., -3.),
            Tuple::new_vector(0., -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        ).unwrap();

        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(r, None).unwrap();
        let color = w.reflected_color(comps, 0);
        assert_eq!(color, Ok(Color::new(0., 0., 0.)));
    }

    #[test]
    fn n1_n2_at_intersections() {
        let mut a = Sphere::new();
        a.shape.material.transparency = 1.0;
        a.shape.material.refractive_index = 1.5;
        a.shape.transform = Matrix4::new_scaling(2., 2., 2.);

        let mut b = Sphere::new();
        b.shape.material.transparency = 1.0;
        b.shape.material.refractive_index = 2.0;
        b.shape.transform = Matrix4::new_translation(0., 0., -0.25);

        let mut c = Sphere::new();
        c.shape.material.transparency = 1.0;
        c.shape.material.refractive_index = 2.5;
        c.shape.transform = Matrix4::new_translation(0., 0., 0.25);

        let s1 = Drawables::Sphere(a);
        let s2 = Drawables::Sphere(b);
        let s3 = Drawables::Sphere(c);

        let r = Ray::new(
            Tuple::new_point(0., 0., -4.),
            Tuple::new_vector(0., 0., 1.)
        ).unwrap();

        let xs = vec![
            Intersection::new(2., &s1),
            Intersection::new(2.75, &s2),
            Intersection::new(3.25, &s3),
            Intersection::new(4.75, &s2),
            Intersection::new(5.25, &s3),
            Intersection::new(6., &s1),
        ];

        let n1s = vec![1.0, 1.5, 2.0, 2.5, 2.5, 1.5];
        let n2s = vec![1.5, 2.0, 2.5, 2.5, 1.5, 1.0];

        for (i, item) in xs.iter().enumerate() {
            let comps = item.prepare_computations(r, Some(&xs)).unwrap();
            assert_eq!(comps.n1, n1s[i]);
            assert_eq!(comps.n2, n2s[i]);
        }
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let mut w: World = Default::default();

        let shape = w.objects[0];

        let r = Ray::new(
            Tuple::new_point(0., 0., -5.),
            Tuple::new_vector(0., 0., 1.)
        ).unwrap();

        let i1 = Intersection::new(4., &shape);
        let i2 = Intersection::new(6., &shape);

        let xs = vec![i1, i2];

        let comps = i1.prepare_computations(r, Some(&xs)).unwrap();

        let color = w.refracted_color(comps, 5);

        assert_eq!(color, Ok(Color::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn refracted_color_max_recursion() {
        let mut w: World = Default::default();

        let mut shape = w.objects[0];
        if let Drawables::Sphere(mut s) = shape {
            s.shape.material.transparency = 1.0;
            s.shape.material.refractive_index = 1.5;
        }

        let r = Ray::new(
            Tuple::new_point(0., 0., -5.),
            Tuple::new_vector(0., 0., 1.)
        ).unwrap();

        let i1 = Intersection::new(4., &shape);
        let i2 = Intersection::new(6., &shape);

        let xs = vec![i1, i2];

        let comps = i1.prepare_computations(r, Some(&xs)).unwrap();
        let color = w.refracted_color(comps, 0);

        assert_eq!(color, Ok(Color::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn refracted_color_total_internal_reflection() {
        let mut w: World = Default::default();

        let mut shape = w.objects[0];
        if let Drawables::Sphere(mut s) = shape {
            s.shape.material.transparency = 1.0;
            s.shape.material.refractive_index = 1.5;
            shape = Drawables::Sphere(s);
        }

        let r = Ray::new(
            Tuple::new_point(0., 0., -2.0_f64.sqrt()/2.0),
            Tuple::new_vector(0., 1., 0.)
        ).unwrap();

        let i1 = Intersection::new(-2.0_f64.sqrt()/2.0, &shape);
        let i2 = Intersection::new(2.0_f64.sqrt()/2.0, &shape);

        let xs = vec![i1, i2];

        let comps = i2.prepare_computations(r, Some(&xs)).unwrap();
        let color = w.refracted_color(comps, 5);

        assert_eq!(color, Ok(Color::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w: World = Default::default();

        let mut o1 = w.objects[0];
        if let Drawables::Sphere(mut s) = o1 {
            s.shape.material.ambient = 1.0;
            s.shape.material.pattern = Some(Patterns::new_test());
            w.objects[0] = Drawables::Sphere(s);
        }
        let s1 = w.objects[0];

        let mut o2 = w.objects[1];
        if let Drawables::Sphere(mut s) = o2 {
            s.shape.material.transparency = 1.0;
            s.shape.material.refractive_index = 1.5;
            w.objects[1] = Drawables::Sphere(s);
        }
        let s2 = w.objects[1];

        let r = Ray::new(
            Tuple::new_point(0., 0., 0.1),
            Tuple::new_vector(0., 1., 0.)
        ).unwrap();

        let i1 = Intersection::new(-0.9899, &s1);
        let i2 = Intersection::new(-0.4899, &s2);
        let i3 = Intersection::new(0.4899, &s2);
        let i4 = Intersection::new(0.9899, &s1);

        let xs = vec![i1, i2, i3, i4];

        let comps = i3.prepare_computations(r, Some(&xs)).unwrap();

        let c = w.refracted_color(comps, 5);

        assert_eq!(c, Ok(Color::new(0., 0.9988745, 0.0472189)));
    }

    #[test]
    fn shade_hit_with_transparent_mat() {
        let mut w: World = Default::default();

        let mut floor = Plane::new_with_transform(Matrix4::new_translation(0., -1., 0.));
        floor.shape.material.transparency = 0.5;
        floor.shape.material.refractive_index = 1.5;

        w.objects.push(Drawables::Plane(floor));

        let mut ball = Sphere::new_with_transform(Matrix4::new_translation(0., -3.5, -0.5));
        ball.shape.material.color = Color::new(1., 0., 0.);
        ball.shape.material.ambient = 0.5;

        w.objects.push(Drawables::Sphere(ball));

        let r = Ray::new(
            Tuple::new_point(0., 0., -3.),
            Tuple::new_vector(0., -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        ).unwrap();

        let xs = vec![Intersection::new(2.0_f64.sqrt(), &w.objects[2])];
        let comps = xs[0].prepare_computations(r, Some(&xs)).unwrap();

        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_mat() {
        let mut w: World = Default::default();

        let mut floor = Plane::new_with_transform(Matrix4::new_translation(0., -1., 0.));
        floor.shape.material.reflective = 0.5;
        floor.shape.material.transparency = 0.5;
        floor.shape.material.refractive_index = 1.5;

        w.objects.push(Drawables::Plane(floor));

        let mut ball = Sphere::new_with_transform(Matrix4::new_translation(0., -3.5, -0.5));
        ball.shape.material.color = Color::new(1., 0., 0.);
        ball.shape.material.ambient = 0.5;

        w.objects.push(Drawables::Sphere(ball));

        let r = Ray::new(
            Tuple::new_point(0., 0., -3.),
            Tuple::new_vector(0., -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0)
        ).unwrap();

        let xs = vec![Intersection::new(2.0_f64.sqrt(), &w.objects[2])];
        let comps = xs[0].prepare_computations(r, Some(&xs)).unwrap();

        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
