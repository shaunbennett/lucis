use super::geometry::{Collision, Primitive, Ray};
use super::{Point, Vector};
use nalgebra::{Projective3, Matrix4, Affine3};
use Transform;

#[derive(Debug, Clone)]
pub enum Material {
    PhongMaterial {
        kd: Vector,
        ks: Vector,
        shininess: f32,
    },
    None,
}

impl Material {
    pub fn phong(kd: Vector, ks: Vector, shininess: f32) -> Material {
        Material::PhongMaterial {
            kd,
            ks,
            shininess
        }
    }
}

#[derive(Debug, Clone)]
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

impl Collidable for SceneNode {
    fn collides(&self, ray: &Ray) -> Option<Collision> {
        let transformed_ray = self.inv_transform * *ray;
        let self_collides = self.primitive.collides(&transformed_ray);

        let min = self
            .children
            .iter()
            .map(|child| child.collides(&transformed_ray))
            .filter(|child| child.is_some())
            .map(|child| child.unwrap())
            .fold(None, |min, child| match min {
                None => Some(child),
                Some(cmin) => Some(if cmin < child { cmin } else { child }),
            });

        match (self_collides, min) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(a)) => Some(a),
            (Some(a), Some(b)) => Some(if a < b { a } else { b }),
        }
    }
}

impl SceneNode {
    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        println!("Applying scaling to {} of ({}, {}, {})", self.name, x, y, z);
        self.apply_transform(Matrix4::new_nonuniform_scaling(&Vector::new(x, y, z)));
    }
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        println!("Applying translation to {} of ({}, {}, {})", self.name, x, y, z);
        self.apply_transform(Matrix4::new_translation(&Vector::new(x, y, z)));
    }
    pub fn rotate(&mut self, axis: &str, angle: f32) {
        println!("Applying rotation to {} of ({}, {})", self.name, axis, angle);
        let axis = match axis {
            "x" => Vector::x_axis(),
            "y" => Vector::y_axis(),
            "z" => Vector::z_axis(),
            _ => panic!("Got unexpected axis: \'{}\' while trying to apply rotation to node \'{}\'", axis, self.name),
        };
        self.apply_transform(Matrix4::from_axis_angle(&axis, angle.to_radians()));
    }
    fn apply_transform(&mut self, t: Matrix4<f32>) {
        let ta: Affine3<f32> = Affine3::from_matrix_unchecked(t);
        self.transform = ta * self.transform;
        self.inv_transform = self.transform.inverse();
    }
}

pub trait Collidable {
    fn collides(&self, ray: &Ray) -> Option<Collision>;
}
