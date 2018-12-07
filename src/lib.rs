pub mod geometry;
pub mod scene;

mod raytrace;
pub use crate::raytrace::Raytracer;

use nalgebra::{Point3, Transform3, Vector3};

pub type Point = Point3<f32>;
pub type Vector = Vector3<f32>;
pub type Transform = Transform3<f32>;
