use nalgebra::{Point3,Vector3,Transform3,norm};
use std::ops::Mul;
use image::{RgbImage, Rgb};

#[derive(Clone,Copy,Debug,PartialEq, PartialOrd)]
pub struct Ray {
    // The src position that the ray is coming from
    pub src: Point3<f32>,
    // The direction the ray is moving, assume it is not already normalized
    pub dir: Vector3<f32>,
}

impl Mul<Ray> for Transform3<f32> {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Ray {
        Ray {
            src: self * rhs.src,
            dir: self * rhs.dir,
        }
    }
}

impl Ray {
    pub fn new(a: Point3<f32>, b: Point3<f32>) -> Ray {
        Ray{ src: a, dir: (b - a).normalize() }
    }
}
