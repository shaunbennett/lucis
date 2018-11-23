use geometry::{Primitive, Ray};
use nalgebra::{clamp, distance_squared, Affine3, Matrix3, Matrix4, Vector3, U3};
use scene::{Color, Intersection, Light};
use Raytracer;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Material {
    PhongMaterial {
        kd: Color,
        ks: Color,
        shininess: f32,
    },
    None,
}

// glm::vec3 calculatePhongIllumination(const SceneNode &root, const glm::vec3 &eye, const Collision &collision, const glm::vec3 &ambient, const std::list<Light *> &lights) {
// 	PhongMaterial *mat = static_cast<PhongMaterial*>(collision.geometryNode->m_material);

// 	glm::vec3 nVec = glm::normalize(collision.normal);
// 	glm::vec3 vVec = glm::normalize(eye - collision.collisionPoint);
// 	// Db("---");
// 	// Db("n: " << glm::to_string(nVec));
// 	// Db("v: " << glm::to_string(vVec));

// 	glm::vec3 diffuseColor;
// 	if (collision.textureColor != glm::vec3(-1)) {
// 		// Db("Custom color: " << glm::to_string(collision.textureColor));
// 		diffuseColor = collision.textureColor;
// 	} else {
// 		diffuseColor = mat->m_kd;
// 	}
// 	glm::vec3 finalColor = diffuseColor * ambient;

// 	for (Light *light : lights) {

// 		// first we should check if the path from collision point to light is open
// 		Ray shadowRay{collision.collisionPoint, light->position};
// 		std::list<Collision> collisions;
// 		if (root.collides(shadowRay, collisions)) continue;

// 		glm::vec3 lVec = light->position - collision.collisionPoint;
// 		float lLength = glm::length(lVec);
// 		lVec = glm::normalize(lVec);

// 		// Db("l: " << glm::to_string(lVec));
// 		float ldotn = glm::clamp(glm::dot(lVec, nVec), 0.0f, 1.0f);
// 		glm::vec3 rVec = glm::normalize((2 * ldotn * nVec) - lVec);
// 		float rdotv = glm::clamp(glm::dot(rVec, vVec), 0.0f, 1.0f);
// 		float attenuation = light->falloff[0] + (light->falloff[1] * lLength) + (light->falloff[2] * lLength * lLength);
// 		glm::vec3 lightSum = (diffuseColor * ldotn * light->colour) + (mat->m_ks * glm::pow(rdotv, mat->m_shininess) * light->colour);
// 		lightSum = lightSum / attenuation;
// 		finalColor += lightSum;
// 	}

// 	// Db("---");
// 	// Db(glm::to_string(finalColor));

// 	return finalColor;
// }

fn calculate_phong_lighting(
    kd: &Color,
    ks: &Color,
    shininess: &f32,
    ray: &Ray,
    raytracer: &Raytracer,
    intersect: &Intersection,
) -> Color {
    let intersect_point = intersect.point;
    let n = intersect.normal.normalize();
    let v = (raytracer.eye - intersect_point).normalize();

    let mut final_color = *kd * raytracer.ambient;

    for light in raytracer.lights.iter() {
        let shadow_ray = Ray::new_from_points(intersect_point, light.position);
        if raytracer.root_node.intersects(&shadow_ray).is_some() {
            continue;
        }
        let mut L = light.position - intersect_point;
        let L_norm = L.norm();
        L = L.normalize();

        let Ldotn = clamp(L.dot(&n), 0.0f32, 1.0f32);
        let r = ((2.0f32 * Ldotn * n) - L).normalize();
        let rdotv = clamp(r.dot(&v), 0.0f32, 1.0f32);
        let attenuation =
            light.falloff[0] + (light.falloff[1] * L_norm) + (light.falloff[2] * L_norm * L_norm);
        let light_sum = (kd * Ldotn * light.color) + (ks * rdotv.powf(*shininess) * light.color);
        final_color = final_color + (light_sum / attenuation);
    }

    return final_color;
}

impl Material {
    pub fn phong(kd: Color, ks: Color, shininess: f32) -> Material {
        Material::PhongMaterial { kd, ks, shininess }
    }

    pub fn get_color(&self, ray: &Ray, raytracer: &Raytracer, intersect: &Intersection) -> Color {
        match self {
            Material::PhongMaterial { kd, ks, shininess } => {
                calculate_phong_lighting(kd, ks, shininess, ray, raytracer, intersect)
            }
            Material::None => Color::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneNode {
    pub id: u32,
    pub children: Vec<SceneNode>,
    pub transform: Affine3<f32>,
    pub inv_transform: Affine3<f32>,
    pub name: String,

    // Material and Primitive
    pub material: Material,
    pub primitive: Primitive,
}

impl SceneNode {
    pub fn new(id: u32, name: String) -> SceneNode {
        SceneNode {
            id: id,
            children: Vec::new(),
            transform: Affine3::identity(),
            inv_transform: Affine3::identity(),
            name: name,
            material: Material::None,
            primitive: Primitive::None,
        }
    }
}

impl Intersect for SceneNode {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let transformed_ray = self.inv_transform * *ray;

        let mut t_value: f32 = 0.0;
        let mut normal = Vector3::new(0.0f32, 0.0, 0.0);
        let self_collides = if self
            .primitive
            .collides(&transformed_ray, &mut t_value, &mut normal)
        {
            Some(Intersection::new(
                t_value,
                transformed_ray.src + (t_value * transformed_ray.dir.normalize()),
                &self,
                normal,
            ))
        } else {
            None
        };

        let min = self
            .children
            .iter()
            .map(|child| child.intersects(&transformed_ray))
            .filter(|child| child.is_some())
            .map(|child| child.unwrap())
            .fold(None, |min, child| match min {
                None => Some(child),
                Some(cmin) => Some(if distance_squared(&cmin.point, &transformed_ray.src)
                    < distance_squared(&child.point, &transformed_ray.src)
                {
                    cmin
                } else {
                    child
                }),
            });

        match (self_collides, min) {
            (None, None) => None,
            (Some(a), None) => Some(a.apply_transform(&self.transform, &self.inv_transform)),
            (None, Some(a)) => Some(a.apply_transform(&self.transform, &self.inv_transform)),
            (Some(a), Some(b)) => Some(
                (if distance_squared(&a.point, &transformed_ray.src)
                    < distance_squared(&b.point, &transformed_ray.src)
                {
                    a
                } else {
                    b
                }).apply_transform(&self.transform, &self.inv_transform),
            ),
        }
    }
}

impl SceneNode {
    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        println!("Applying scaling to {} of ({}, {}, {})", self.name, x, y, z);
        self.apply_transform(Matrix4::new_nonuniform_scaling(&Vector3::new(x, y, z)));
    }
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        println!(
            "Applying translation to {} of ({}, {}, {})",
            self.name, x, y, z
        );
        self.apply_transform(Matrix4::new_translation(&Vector3::new(x, y, z)));
    }
    pub fn rotate(&mut self, axis: &str, angle: f32) {
        println!(
            "Applying rotation to {} of ({}, {})",
            self.name, axis, angle
        );
        let axis = match axis {
            "x" | "X" => Vector3::x_axis(),
            "y" | "Y" => Vector3::y_axis(),
            "z" | "Z" => Vector3::z_axis(),
            _ => panic!(
                "Got unexpected axis: \'{}\' while trying to apply rotation to node \'{}\'",
                axis, self.name
            ),
        };
        self.apply_transform(Matrix4::from_axis_angle(&axis, angle.to_radians()));
    }
    fn apply_transform(&mut self, t: Matrix4<f32>) {
        let ta: Affine3<f32> = Affine3::from_matrix_unchecked(t);
        self.transform = ta * self.transform;
        self.inv_transform = self.transform.inverse();
    }
}

pub trait Intersect {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}
