// I think this is where raytracing utils will live
// like lighting and shit that doesn't clearly belong to a struct
// We'll see what happens
use super::color::Color;
use super::point_light::PointLight;
use super::tuple::Tuple;
use super::patterns::Pattern;
use super::generics::{Drawables, Drawable};

pub const EPSILON: f64 = 0.00001;

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
