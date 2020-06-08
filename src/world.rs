use super::point_light::PointLight;
use super::sphere::Sphere;
use super::material::Material;
use super::color::Color;
use super::matrix::Matrix4;
use super::tuple::Tuple;
use super::intersection::Comps;
use super::ray::Ray;
use super::utils::{lighting, schlick};
use super::intersection::hit;
use super::generics::{Drawables, Drawable};

#[allow(dead_code)]
pub struct World {
    pub lights: Vec<PointLight>,
    pub objects: Vec<Drawables>,
}

#[allow(dead_code)]
impl World {
    pub fn shade_hit(&self, comps: Comps, remaining: usize) -> Color {
        let mut color = Color::new(0., 0., 0.);

        for light in self.lights.iter() {
            let in_shadow = self.is_shadowed(comps.over_point, light);

            if let Ok(result) = lighting(
                comps.object,
                comps.over_point,
                *light,
                comps.eye_v,
                comps.normal_v,
                in_shadow
            ) {
                color = color + result;

            }
        }
        let reflected = match self.reflected_color(comps, remaining) {
            Ok(r) => r,
            Err(_) => Color::new(0., 0., 0.)
        };

        let refracted = match self.refracted_color(comps, remaining) {
            Ok(r) => r,
            Err(_) => Color::new(0., 0., 0.)
        };

        let material = comps.object.get_shape().material;
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = schlick(comps);
            color = color + (reflected * reflectance) + (refracted * (1.0 - reflectance));
        } else {
            color = color + reflected + refracted;
        }

        color
    }

    pub fn color_at(&self, ray: Ray, remaining: usize) -> Color {
        let xs = ray.intersect_world(self);
        if let Some(hit) = hit(&xs) {
            if let Some(comps) = hit.prepare_computations(ray, None) {
                return self.shade_hit(comps, remaining);
            }
        }
        Color::new(0., 0., 0.)
    }

    pub fn is_shadowed(&self, point: Tuple, light: &PointLight) -> bool {
        let v = light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        if let Ok(shadow_ray) = Ray::new(point, direction) {
            let xs = shadow_ray.intersect_world(&self);
            if let Some(hit) = hit(&xs) {
                if hit.t < distance {
                    return true;
                }
            }
        }
        false
    }

    pub fn reflected_color(&self, comps: Comps, remaining: usize) -> Result<Color, &'static str> {
        if remaining < 1 {
            return Ok(Color::new(0., 0., 0.));
        }

        let material = comps.object.get_shape().material;
        if material.reflective == 0.0 {
            return Ok(Color::new(0., 0., 0.));
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflect_v)?;
        let color = self.color_at(reflect_ray, remaining - 1);

        Ok(color * material.reflective)
    }

    pub fn refracted_color(&self, comps: Comps, remaining: usize) -> Result<Color, &'static str> {
        if remaining < 1 {
            return Ok(Color::new(0., 0., 0.));
        }

        let material = comps.object.get_shape().material;
        if material.transparency == 0.0 {
            return Ok(Color::new(0., 0., 0.));
        }

        // Find total internal reflection using Snell's Law
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_v.dot(comps.normal_v);
        let sin2_t = n_ratio.powf(2.0) * (1.0 - cos_i.powf(2.0));

        if sin2_t > 1.0 {
            return Ok(Color::new(0., 0., 0.));
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normal_v * (n_ratio * cos_i - cos_t) - comps.eye_v * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction)?;

        Ok(self.color_at(refract_ray, remaining - 1) * comps.object.get_shape().material.transparency)
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Tuple::new_point(-10., 10., -10.), Color::new(1., 1., 1.)).unwrap();
        let mut s1 = Sphere::new();
        s1.shape.material = Material{
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        let s2 = Sphere::new_with_transform(Matrix4::new_scaling(0.5, 0.5, 0.5));

        let objects = vec!(Drawables::Sphere(s1), Drawables::Sphere(s2));

        Self {lights: vec![light], objects}
    }
}
