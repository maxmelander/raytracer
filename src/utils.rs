// I think this is where raytracing utils will live
// like lighting and shit that doesn't clearly belong to a struct
// We'll see what happens
use super::material::Material;
use super::point_light::PointLight;
use super::tuple::Tuple;
use super::color::Color;

#[allow(dead_code, unused_variables)]
// Phong lighting
pub fn lighting(material: Material, point: Tuple, light: PointLight, eye_v: Tuple, normal_v: Tuple) -> Result<Color, &'static str> {
    if point.is_vector() || eye_v.is_point() || normal_v.is_point() {
        return Err("point or vectors not correct format");
    }

    let effective_color = material.color * light.intensity;
    let light_v = (light.position - point).normalize();
    let ambient = effective_color * material.ambient;
    let mut diffuse = Color::new(0.0, 0.0, 0.0);
    let mut specular = Color::new(0.0, 0.0, 0.0);

    let light_dot_normal = light_v.dot(&normal_v);
    if light_dot_normal >= 0.0 {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflect_v = -light_v.reflect(normal_v);
        let reflect_dot_eye = reflect_v.dot(&eye_v);

        if reflect_dot_eye >= 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    Ok(ambient + diffuse + specular)
}
