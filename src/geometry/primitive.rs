use geometry::Ray;
use nalgebra::{dot, Unit, Vector3};
use geometry::Mesh;
use roots::find_roots_quadratic;
use roots::Roots;

const SPHERE_EPS: f32 = 0.0001;
const CYLINDER_EPS: f32 = 0.0001;
const CUBE_EPS: f32 = 0.0001;
const CLOSE_EPS: f32 = 0.001;
const TRIANGLE_EPS: f32 = 0.0000001;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Sphere,
    Cube,
    Cylinder,
    Mesh(Mesh),
    None,
}

impl Primitive {
    pub fn collides(&self, ray: &Ray, t_value: &mut f32, normal: &mut Vector3<f32>) -> bool {
        match self {
            Primitive::Sphere => sphere_collides(ray, t_value, normal),
            Primitive::Cylinder => cylinder_collides(ray, t_value, normal),
            Primitive::Cube => cube_collides(ray, t_value, normal),
            Primitive::Mesh(mesh) => mesh_collides(ray, mesh, t_value, normal),
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

fn triangle_collides(ray: &Ray, triangle: &[Vector3<f32>; 3], t_value: &mut f32, normal: &mut Vector3<f32>) -> bool {
    let edge1 = triangle[1] - triangle[0];
    let edge2 = triangle[2] - triangle[0];

    let face_normal = edge1.cross(&edge2).normalize();

    let q = ray.dir.cross(&edge2);
    let a = edge1.dot(&q);

    if (a.abs() <= TRIANGLE_EPS) || face_normal.dot(&ray.dir) >= 0.0 {
        return false;
    }

    let s = (ray.src - triangle[0]).coords / a;
    let r = s.cross(&edge1);

    let x = s.dot(&q);
    let y = r.dot(&ray.dir);
    let z = 1.0f32 - x - y;

    if x < 0.0 || y < 0.0 || z < 0.0 {
        return false;
    }

    *t_value = edge2.dot(&r);

    if *t_value < TRIANGLE_EPS {
        return false
    }

    *normal = face_normal;
    true
}

fn cylinder_collides(ray: &Ray, t_value: &mut f32, normal: &mut Vector3<f32>) -> bool {
    // ray = src + t*dir
    // cylinder: x^2 + y^2 = 1

    let src = &ray.src;
    let dir = &ray.dir;

    let a = (dir.x * dir.x) + (dir.y * dir.y);
    let b = 2.0f32 * ((src.x * dir.x) + (src.y * dir.y));
    let c = (src.x * src.x) + (src.y * src.y) - 1.0f32;

    let mut intercept_cap = false;
    // closest cap
    let mut cap_normal = Vector3::new(0.0, 0.0, 1.0);
    let closest_root = match find_roots_quadratic(a, b, c) {
        Roots::One([r1]) => r1,
        Roots::Two([r1, r2]) => {
            let i_1 = &ray.src + (r1 * &ray.dir);
            let i_2 = &ray.src + (r2 * &ray.dir);
            if i_1.z > 1.0 && i_2.z > 1.0 {
                return false
            } else if i_1.z < -1.0 && i_2.z < -1.0 {
                return false
            } else if i_1.z.abs() < 1.0 && i_2.z.abs() < 1.0 {
                r1
            } else {
                // check cap intercepts
                r1
//                println!("z1: {}, z2: {}", i_1.z, i_2.z);
//                if i_1.z < -1.0 && -1.0 < i_2.z {
//                    // first cap
//                    let t_value = (-1.0f32 - src.z) / dir.z;
//                    if t_value < r1 {
//                        intercept_cap = true;
//                        t_value
//                    } else {
//                        r1
//                    }
//                } else if i_1.z < 1.0 && 1.0 < i_2.z {
//                    // second cap
//                    let t_value = (1.0f32 - src.z) / dir.z;
//                    if t_value < r2 {
//                        cap_normal = Vector3::new(0.0, 0.0, -1.0);
//                        intercept_cap = true;
//                        t_value
//                    } else {
//                        r2
//                    }
//                } else {
//                    intercept_cap = true;
//                    let t1 = (-1.0f32 - src.z) / dir.z;
//                    let t2 = (1.0f32 - src.z) / dir.z;
//                    if t1 < t2 {
//                        t1
//                    } else {
//                        cap_normal = Vector3::new(0.0, 0.0, -1.0);
//                        t2
//                    }
//                }
            }
        },
        _ => return false
    };

    if closest_root > CYLINDER_EPS {
        let intersection_point = &ray.src + (closest_root * &ray.dir);
        *t_value = closest_root;
        if intercept_cap {
            *normal = cap_normal;
        } else {
            *normal = Vector3::new(intersection_point.x, intersection_point.y, 0.0f32);
        }
        true
    } else {
        false
    }
}

fn mesh_collides(ray: &Ray, mesh: &Mesh, t_value: &mut f32, normal: &mut Vector3<f32>) -> bool {
    if !aabb_collides(ray, &mesh.aabb_corner, &mesh.aabb_size, t_value) {
        return false
    }

    let mut smallest_t = -1.0f32;
    let mut smallest_normal = Vector3::new(0.0f32, 0.0f32, 0.0f32);
    let mut triangle = [smallest_normal, smallest_normal, smallest_normal];

    for face in mesh.faces.iter() {
        triangle[0] = mesh.vertices[face[0]];
        triangle[1] = mesh.vertices[face[1]];
        triangle[2] = mesh.vertices[face[2]];

        if triangle_collides(ray, &triangle, t_value, normal) {
            if smallest_t == -1.0 || *t_value < smallest_t {
                smallest_t = *t_value;
                smallest_normal = *normal;
            }
        }
    }

    *normal = smallest_normal;
    *t_value = smallest_t;
    smallest_t != -1.0
}