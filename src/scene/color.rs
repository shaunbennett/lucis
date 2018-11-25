use image::Rgb;
use nalgebra::Vector3;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }.clamp()
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }.clamp()
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }.clamp()
    }
}

impl<'a, 'b> Mul<&'b Vector3<f32>> for &'a Color {
    type Output = Color;

    fn mul(self, rhs: &'b Vector3<f32>) -> Color {
        Color {
            r: self.r * rhs.x,
            g: self.g * rhs.y,
            b: self.b * rhs.z,
        }.clamp()
    }
}

impl<'a> Mul<f32> for &'a Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }.clamp()
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Color {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }.clamp()
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn as_rgb(&self) -> Rgb<u8> {
        Rgb([
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
        ])
    }

    fn clamp(mut self) -> Color {
        self.r = self.r.max(0.0).min(1.0);
        self.g = self.g.max(0.0).min(1.0);
        self.b = self.b.max(0.0).min(1.0);
        self
    }
}
