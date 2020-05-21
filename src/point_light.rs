use super::tuple::Tuple;
use super::color::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple
}

#[allow(dead_code)]
impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Result<Self, &'static str> {
        if position.is_vector() {
            Err("Position can't be a vector")
        } else {
            Ok(Self{position, intensity})
        }
    }
}
