use super::ray::Ray;
use super::Collision;
use nalgebra::{Vector3, Point3, dot, Unit};
use roots::find_roots_quadratic;
use roots::Roots;

const SPHERE_EPS: f32 = 0.1;

#[derive(Debug, Clone)]
pub enum Primitive {
    Sphere,
    None,
}

impl Primitive {
    pub fn collides(&self, ray: &Ray) -> Option<Collision> {
        match self {
            Primitive::Sphere => sphere_collides(ray),
            _ => None,
        }
    }
}

fn sphere_collides(ray: &Ray) -> Option<Collision> {
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
        _ => return None,
    };

    if closest_root > SPHERE_EPS {
        Some(Collision::new(closest_root))
    } else {
        None
    }
}

//   const float eps = 0.1;
//  glm::vec3 L = ray.eyePoint;
//  double a = glm::dot(ray.direction, ray.direction);
//  double b = 2 * glm::dot(L, ray.direction);
//  double c = glm::dot(L, L) - 1;
//
//  double roots[2];
//  size_t numRoots = quadraticRoots(a, b, c, roots);
//  if (numRoots == 0)
//    return false;
//  else if (numRoots == 1) {
//    if (roots[0] < eps)
//      return false;
//    collisionPoint = ray.eyePoint + (ray.direction * float(roots[0]));
//  } else if (numRoots == 2) {
//    if (roots[0] < roots[1] && roots[0] >= eps) {
//      collisionPoint = ray.eyePoint + (ray.direction * float(roots[0]));
//    } else if (roots[1] >= eps) {
//      collisionPoint = ray.eyePoint + (ray.direction * float(roots[1]));
//    } else {
//      return false;
//    }
//  }
//
//  glm::vec3 normalDirection = collisionPoint;
//  normal = glm::normalize(normalDirection);
//
//  return true;
