use super::color::Color;
use super::patterns::Patterns;

#[allow(dead_code)]
pub const VACUUM: f64 = 1.0;
#[allow(dead_code)]
pub const AIR: f64 = 1.00029;
#[allow(dead_code)]
pub const WATER: f64 = 1.333;
#[allow(dead_code)]
pub const GLASS: f64 = 1.52;
#[allow(dead_code)]
pub const DIAMOND: f64 = 2.417;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub reflective: f64,
    pub shininess: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Option<Patterns>
}

impl Default for Material {
    fn default() -> Self{
        Self {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            reflective: 0.0,
            shininess: 200.,
            transparency: 0.0,
            refractive_index: VACUUM,
            pattern: None
        }
    }
}
