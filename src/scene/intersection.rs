use scene::SceneNode;
use std::cmp::Ordering;
use nalgebra::Vector3;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    // The t value for the ray where this collision occured. Can be used to calculate the intersection point
    pub t_value: f32,
    pub node: &'a SceneNode,
    pub normal: Vector3<f32>,
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        return self.t_value.partial_cmp(&other.t_value);
    }
}

impl<'a> Intersection<'a> {
    pub fn new(t_value: f32, node: &'a SceneNode, normal: Vector3<f32>) -> Intersection {
        Intersection { t_value, node, normal }
    }
}
