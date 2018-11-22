use geometry::Ray;
use nalgebra::{dot, Unit, Vector3};
use roots::find_roots_quadratic;
use roots::Roots;

const SPHERE_EPS: f32 = 0.1;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Sphere,
    None,
}

impl Primitive {
    pub fn collides(&self, ray: &Ray, t_value: &mut f32) -> bool {
        match self {
            Primitive::Sphere => sphere_collides(ray, t_value),
            _ => false,
        }
    }
}

fn sphere_collides(ray: &Ray, t_value: &mut f32) -> bool {
    // Check if circle collides with unit sphere
    let L = &ray.src.coords;
    let udir: Unit<Vector3<f32>> = ray.unit_dir();
    let dir = udir.as_ref();
    let a = dot(dir, dir);
    let b = 2.0f32 * dot(L, dir);
    let c = dot(L, L) - 1.0f32;

    let closest_root = match find_roots_quadratic(a, b, c) {
        Roots::One([r1]) => r1,
        Roots::Two([r1, r2]) => r1,
        _ => return false,
    };

    if closest_root > SPHERE_EPS {
        *t_value = closest_root;
        true
    } else {
        false
    }
}
