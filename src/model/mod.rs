use Transform;
use super::{Vector, Point};
use super::geometry::{Ray,Collision,Primitive};

#[derive(Debug,Clone)]
pub enum Material {
    PhongMaterial {
        kd: Vector,
        ks: Vector,
        shininess: Vector
    },
    None
}

#[derive(Debug,Clone)]
pub struct SceneNode {
    pub id: u32,
    pub children: Vec<SceneNode>,
    pub transform: Transform,
    pub inv_transform: Transform,
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
            transform: Transform::identity(),
            inv_transform: Transform::identity(),
            name: name,
            material: Material::None,
            primitive: Primitive::None,
        }
    }
}

impl Collidable for SceneNode {
    fn collides(&self, ray: &Ray) -> Option<Collision> {
        let self_collides = self.primitive.collides(ray);

        let min = self.children.iter()
            .map(|child| child.collides(ray))
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
}

pub trait Collidable {
    fn collides(&self, ray: &Ray) -> Option<Collision>;
}
