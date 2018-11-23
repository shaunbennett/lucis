use nalgebra::Point3;
use scene::Color;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Light {
    pub color: Color,
    pub position: Point3<f32>,
    pub falloff: [f32; 3],
}

impl Light {
    pub fn new(color: Color, position: Point3<f32>, falloff: [f32; 3]) -> Light {
        Light {
            color,
            position,
            falloff,
        }
    }
}
