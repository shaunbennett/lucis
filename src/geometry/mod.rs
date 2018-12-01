mod mesh;
mod primitive;
mod ray;
pub mod volume;

pub use self::mesh::Mesh;
pub use self::primitive::Primitive;
pub use self::ray::Ray;

use nalgebra::Vector3;
use roots::Roots;

const CUBE_EPS: f32 = 0.0001;

// TODO: This should be very easy to use generics for
pub fn aabb_collision(ray: &Ray, pos: &Vector3<f32>, size: &Vector3<f32>) -> Roots<f32> {
    let inv_dir = Vector3::new(1.0 / ray.dir[0], 1.0 / ray.dir[1], 1.0 / ray.dir[2]);
    let mut tmp;

    let mut tmin = (pos.x - ray.src.x) * inv_dir.x;
    let mut tmax = ((pos.x + size.x) - ray.src.x) * inv_dir.x;
    let mut tymin = (pos.y - ray.src.y) * inv_dir.y;
    let mut tymax = ((pos.y + size.y) - ray.src.y) * inv_dir.y;

    if tmin > tmax {
        tmp = tmin;
        tmin = tmax;
        tmax = tmp;
    }

    if tymin > tymax {
        tmp = tymin;
        tymin = tymax;
        tymax = tmp;
    }

    if (tmin > tymax) || (tymin > tmax) {
        return Roots::No([]);
    }
    if tymin > tmin {
        tmin = tymin;
    }
    if tymax < tmax {
        tmax = tymax;
    }

    let mut tzmin = (pos.z - ray.src.z) * inv_dir.z;
    let mut tzmax = ((pos.z + size.z) - ray.src.z) * inv_dir.z;
    if tzmin > tzmax {
        tmp = tzmin;
        tzmin = tzmax;
        tzmax = tmp;
    }

    if (tmin > tzmax) || (tzmin > tmax) {
        return Roots::No([]);
    }
    if tzmin > tmin {
        tmin = tzmin;
    }
    if tzmax < tmax {
        tmax = tzmax;
    }

    if tmin <= CUBE_EPS {
        if tmax <= CUBE_EPS {
            return Roots::No([]);
        }
        Roots::One([tmax])
    } else {
        Roots::Two([tmin, tmax])
    }
}
