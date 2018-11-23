use geometry::Ray;
use nalgebra::{dot, Unit, Vector3};
use roots::find_roots_quadratic;
use roots::Roots;

const SPHERE_EPS: f32 = 0.0001;
const CUBE_EPS: f32 = 0.0001;
const CLOSE_EPS: f32 = 0.001;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Sphere,
    Cube,
    None,
}

impl Primitive {
    pub fn collides(&self, ray: &Ray, t_value: &mut f32, normal: &mut Vector3<f32>) -> bool {
        match self {
            Primitive::Sphere => sphere_collides(ray, t_value, normal),
            Primitive::Cube => cube_collides(ray, t_value, normal),
            _ => false,
        }
    }
}

fn aabb_collides(ray: &Ray, pos: &Vector3<f32>, size: &Vector3<f32>, t_value: &mut f32) -> bool {
    let inv_dir = Vector3::new(1.0 / ray.dir[0], 1.0 / ray.dir[1], 1.0 / ray.dir[2]);
    let (mut tmin, mut tmax, mut tymin, mut tymax, mut tzmin, mut tzmax, mut tmp) = (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);

    tmin = (pos.x - ray.src.x) * inv_dir.x;
    tmax = ((pos.x + size.x) - ray.src.x) * inv_dir.x;
    tymin = (pos.y - ray.src.y) * inv_dir.y;
    tymax = ((pos.y + size.y) - ray.src.y) * inv_dir.y;

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
        return false;
    }
    if tymin > tmin {
        tmin = tymin;
    }
    if tymax < tmax {
        tmax = tymax;
    }

    tzmin = (pos.z - ray.src.z) * inv_dir.z;
    tzmax = ((pos.z + size.z) - ray.src.z) * inv_dir.z;
    if tzmin > tzmax {
        tmp = tzmin;
        tzmin = tzmax;
        tzmax = tmp;
    }

    if (tmin > tzmax) || (tzmin > tmax) {
        return false;
    }
    if tzmin > tmin {
        tmin = tzmin;
    }
    if tzmax < tmax {
        tmax = tzmax;
    }

    if tmin <= CUBE_EPS {
        if tmax <= CUBE_EPS {
            return false;
        }
        tmin = tmax;
    }
    *t_value = tmin;
    true
}

fn close(a: f32, b: f32) -> bool {
    let diff = (a - b).abs();
    diff < CLOSE_EPS
}

fn cube_collides(ray: &Ray, t_value: &mut f32, normal: &mut Vector3<f32>) -> bool {
    if aabb_collides(ray, &Vector3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 1.0, 1.0), t_value) {
        let collision_point = ray.src + (*t_value * ray.dir);
        // decide which side the point is on
        if close(collision_point.x, 0.0) {
            *normal = Vector3::new(-1.0, 0.0, 0.0);
        } else if close(collision_point.x, 1.0) {
            *normal = Vector3::new(1.0, 0.0, 0.0);
        } else if close(collision_point.y, 0.0) {
            *normal = Vector3::new(0.0, -1.0, 0.0);
        } else if close(collision_point.y, 1.0) {
            *normal = Vector3::new(0.0, 1.0, 0.0);
        } else if close(collision_point.z, 0.0) {
            *normal = Vector3::new(0.0, 0.0, -1.0);
        } else {
            *normal = Vector3::new(0.0, 0.0, 1.0);
        }

        return true;
    }

    false
}

fn sphere_collides(ray: &Ray, t_value: &mut f32, normal: &mut Vector3<f32>) -> bool {
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
        *normal = (ray.src + (closest_root * ray.dir)).coords;
        true
    } else {
        false
    }
}
