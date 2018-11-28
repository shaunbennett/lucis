use nalgebra::{Affine3, Point3, Vector3, U3};
use scene::SceneNode;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    // The t value for the ray where this collision occured. Can be used to calculate the intersection point
    pub t_value: f32,
    pub point: Point3<f32>,
    pub node: &'a SceneNode,
    pub normal: Vector3<f32>,
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        self.t_value.partial_cmp(&other.t_value)
    }
}

impl<'a> Intersection<'a> {
    pub fn new(
        t_value: f32,
        point: Point3<f32>,
        node: &'a SceneNode,
        normal: Vector3<f32>,
    ) -> Intersection {
        Intersection {
            t_value,
            point,
            node,
            normal,
        }
    }

    pub fn apply_transform(
        self,
        transform: &Affine3<f32>,
        inv_transform: &Affine3<f32>,
    ) -> Intersection<'a> {
        let inv_mat3_transpose = inv_transform
            .matrix()
            .fixed_resize::<U3, U3>(0.0f32)
            .transpose();
        let transformed_point = transform * self.point;
        let transformed_normal = (inv_mat3_transpose * self.normal).normalize();
        Intersection {
            t_value: self.t_value,
            point: transformed_point,
            node: self.node,
            normal: transformed_normal,
        }
    }
}
