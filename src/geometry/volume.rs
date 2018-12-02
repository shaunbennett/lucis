use geometry::Ray;
use nalgebra::{Vector3};
use roots::Roots;
use scene::{Intersection, Color};
use super::aabb_collision;

// Volumes that can be passed through

pub struct VolumetricSolid {
    volume: Volume,
    effect: VolumeEffect, }

impl VolumetricSolid {
    pub fn new(volume: Volume, effect: VolumeEffect) -> VolumetricSolid {
        VolumetricSolid {
            volume, effect
        }
    }

    pub fn apply(&self, ray: &Ray, ri: &Option<Intersection>, curr_color: Color) -> Color {
        match self.volume.passes_through(ray) {
            Some(intersection) => self.effect.apply(ray, ri, &intersection, curr_color),
            None => curr_color
        }
    }
}

// Represents a volume that can be passed through by a ray
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Volume {
    Box(BoxParams),
}

impl Volume {
    fn passes_through(&self, ray: &Ray) -> Option<VolumeIntersection> {
        match self {
            Volume::Box(b) => box_passes_through(b, ray),
            _ => None
        }
    }
}


// Represents an effect on the resulting pixel a volume has while being passed through
pub enum VolumeEffect {
    // Fog color
    Fog(Color)
}

impl VolumeEffect {
    fn apply(&self, ray: &Ray, ri: &Option<Intersection>, vi: &VolumeIntersection, curr_color: Color) -> Color {
        match self {
            VolumeEffect::Fog(color) => fog_apply(*color, ray, ri, vi, curr_color),
            _ => curr_color
        }
    }
}

fn fog_apply(fog_color: Color, ray: &Ray, ri: &Option<Intersection>, vi: &VolumeIntersection, curr_color: Color) -> Color {
    // Do calculation to apply fog effect
    let fog_amount = match ri {
        Some(ray_i) => {
            // Caluclate time in fog
            let enter = vi.t_enter.max(0.0);
            let leave = vi.t_leave;
            let t_intersect = (ray_i.point - ray.src).x / ray.dir.x;

            if enter >= t_intersect {
                return curr_color
            }

            if leave < t_intersect {
                let i1 = ray.src + (vi.t_enter.max(0.0) * ray.dir);
                let i2 = ray.src + (vi.t_leave * ray.dir);
                let distance = nalgebra::distance(&i1, &i2);
                distance * 0.03
            } else {
                let i1 = ray.src + (vi.t_enter.max(0.0) * ray.dir);
                let i2 = ray_i.point;
                let distance = nalgebra::distance(&i1, &i2);
                distance * 0.03
            }

        },
        None => {
            let i1 = ray.src + (vi.t_enter.max(0.0) * ray.dir);
            let i2 = ray.src + (vi.t_leave * ray.dir);
            let distance = nalgebra::distance(&i1, &i2);
            distance * 0.03
        }
    }.max(0.0).min(1.0);

    (fog_amount * fog_color) + ((1.0-fog_amount) * curr_color)
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
pub struct BoxParams {
    pub pos: Vector3<f32>,
    pub size: Vector3<f32>,
}

fn box_passes_through(b: &BoxParams, ray: &Ray) -> Option<VolumeIntersection> {
    let roots = aabb_collision(ray, &b.pos, &b.size);

    match roots {
        Roots::Two([t1, t2]) => Some(VolumeIntersection::new(t1, t2)),
        Roots::One([t1]) => Some(VolumeIntersection::new(0.0f32, t1)),
        _ => None
    }
}
