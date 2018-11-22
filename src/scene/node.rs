use geometry::{Primitive, Ray};
use nalgebra::{Affine3, Matrix4, Vector3};
use scene::{Color, Intersection};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Material {
    PhongMaterial {
        kd: Color,
        ks: Color,
        shininess: f32,
    },
    None,
}

impl Material {
    pub fn phong(kd: Color, ks: Color, shininess: f32) -> Material {
        Material::PhongMaterial { kd, ks, shininess }
    }

    pub fn get_color(&self, intersect: &Intersection) -> Color {
        match self {
            Material::PhongMaterial { kd, ks, shininess } => *kd,
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
        let self_collides = if self.primitive.collides(&transformed_ray, &mut t_value) {
            Some(Intersection::new(t_value, &self))
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
            "x" => Vector3::x_axis(),
            "y" => Vector3::y_axis(),
            "z" => Vector3::z_axis(),
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
