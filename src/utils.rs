// I think this is where raytracing utils will live
// like lighting and shit that doesn't clearly belong to a struct
// We'll see what happens
use super::color::Color;
use super::point_light::PointLight;
use super::tuple::Tuple;
use super::generics::{Drawables, Drawable};
use super::intersection::Comps;

pub const EPSILON: f64 = 0.00001;
pub const RECURSION_DEPTH: usize = 5;

pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[allow(dead_code, unused_variables)]
// Phong lighting
pub fn lighting(
    object: &Drawables,
    point: Tuple,
    light: PointLight,
    eye_v: Tuple,
    normal_v: Tuple,
    in_shadow: bool,
) -> Result<Color, &'static str> {
    if point.is_vector() || eye_v.is_point() || normal_v.is_point() {
        return Err("point or vectors not correct format");
    }
    let material = object.get_shape().material;
    let color: Color;

    if let Some(p) = material.pattern {
        match p.color_at_object(object, point) {
            Some(c) => color = c,
            None => return Err("Could not get pattern color at object")
        }
    } else {
        color = material.color;
    }

    let effective_color = color * light.intensity;
    let ambient = effective_color * material.ambient;
    if in_shadow {
        return Ok(ambient);
    }

    let light_v = (light.position - point).normalize();
    let mut diffuse = Color::new(0.0, 0.0, 0.0);
    let mut specular = Color::new(0.0, 0.0, 0.0);

    let light_dot_normal = light_v.dot(normal_v);
    if light_dot_normal >= 0.0 {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflect_v = -light_v.reflect(normal_v);
        let reflect_dot_eye = reflect_v.dot(eye_v);

        if reflect_dot_eye >= 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    Ok(ambient + diffuse + specular)
}

pub fn schlick(comps: Comps) -> f64 {
    let mut cos = comps.eye_v.dot(comps.normal_v);

    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powf(2.0) * (1.0 - cos.powf(2.0));
        if sin2_t > 1.0 {
            return 1.0;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        cos = cos_t;
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powf(2.0);

    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}
