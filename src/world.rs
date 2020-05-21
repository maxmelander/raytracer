use super::point_light::PointLight;
use super::sphere::Sphere;
use super::material::Material;
use super::color::Color;
use super::matrix::Matrix4;
use super::tuple::Tuple;
use super::intersection::Comps;
use super::ray::Ray;
use super::utils::lighting;
use super::intersection::hit;

#[allow(dead_code)]
pub struct World {
    pub lights: Vec<PointLight>,
    pub objects: Vec<Sphere>
}

#[allow(dead_code)]
impl World {
    pub fn shade_hit(&self, comps: Comps) -> Color {
        let mut color = Color::new(0., 0., 0.);

        for light in self.lights.iter() {
            if let Ok(result) = lighting(comps.object.material, comps.point, *light, comps.eye_v, comps.normal_v) {
                color = color + result;
            }
        }

        color
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = ray.intersect_world(self);
        if let Some(hit) = hit(&xs) {
            if let Some(comps) = hit.prepare_computations(ray) {
                return self.shade_hit(comps);
            }
        }
        Color::new(0., 0., 0.)
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Tuple::new_point(-10., 10., -10.), Color::new(1., 1., 1.)).unwrap();
        let mut s1 = Sphere::new();
        s1.material = Material{
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        let s2 = Sphere::new_with_transform(Matrix4::new_scaling(0.5, 0.5, 0.5));

        let objects = vec!(s1, s2);

        Self {lights: vec![light], objects}
    }
}
