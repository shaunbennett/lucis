use geometry::Ray;
use nalgebra::{Vector3};
use roots::Roots;
use scene::Color;
use super::aabb_collision;

// Volumes that can be passed through

// Represents a volume that can be passed through by a ray
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Volume {
    Box(Box),
}

impl Volume {
    pub fn passes_through(&self, ray: &Ray) -> Option<VolumeIntersection> {
        match self {
            Volume::Box(b) => box_passes_through(b, ray),
            _ => None
        }
    }
}


// Represents an effect on the resulting pixel a volume has while being passed through
pub enum VolumeEffect {
    Fog
}

impl VolumeEffect {
    fn apply(&self, ray: &Ray, vi: &VolumeIntersection, curr_color: Color) -> Color {
        match self {
            Fog => fog_apply(ray, vi, curr_color),
            _ => curr_color
        }
    }
}

fn fog_apply(ray: &Ray, vi: &VolumeIntersection, curr_color: Color) -> Color {
    // Do calculation to apply fog effect

    curr_color
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct VolumeIntersection {
    // When the ray entered the volume
    pub t_enter: f32,
    // When the ray left the volume
    pub t_leave: f32,
}

impl VolumeIntersection {
    fn new(t_enter: f32, t_leave: f32) -> VolumeIntersection {
        VolumeIntersection {
            t_enter,
            t_leave,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Box {
    pos: Vector3<f32>,
    size: Vector3<f32>,
}

fn box_passes_through(b: &Box, ray: &Ray) -> Option<VolumeIntersection> {
    let roots = aabb_collision(ray, &b.pos, &b.size);

    match roots {
        Roots::Two([t1, t2]) => Some(VolumeIntersection::new(t1, t2)),
        _ => None
    }
}
