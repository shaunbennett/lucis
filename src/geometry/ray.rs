use image::{Rgb, RgbImage};
use nalgebra::{Point3, Affine3, Vector3, Unit, Transform3};
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Ray {
    // The src position that the ray is coming from
    pub src: Point3<f32>,
    // The direction the ray is moving, assume it is not already normalized
    pub dir: Vector3<f32>,
}

impl Mul<Ray> for Affine3<f32> {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Ray {
        Ray {
            src: self * rhs.src,
            dir: self * rhs.dir,
        }
    }
}

impl Ray {
    pub fn new(src: Point3<f32>, dir: Vector3<f32>) -> Ray {
        Ray {
            src: src,
            dir: dir.normalize(),
        }
    }

    pub fn unit_dir(&self) -> Unit<Vector3<f32>> {
        Unit::new_normalize(self.dir)
    }
}
