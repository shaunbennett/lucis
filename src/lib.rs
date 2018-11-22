extern crate image;
extern crate nalgebra;
extern crate png;
extern crate rand;
extern crate rlua;
extern crate roots;

pub mod geometry;
pub mod scene;

mod raytrace;
pub use raytrace::{Raytracer, TracingOptions};

use nalgebra::{Point3, Transform3, Vector3};

pub type Point = Point3<f32>;
pub type Vector = Vector3<f32>;
pub type Transform = Transform3<f32>;
