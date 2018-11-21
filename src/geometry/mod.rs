mod primitive;
mod ray;

pub use self::primitive::Primitive;
pub use self::ray::Ray;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Collision {
    t_value: f32
}

impl Collision {
    pub fn new(t_value: f32) -> Collision {
        Collision { t_value }
    }
}
