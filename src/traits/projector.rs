use crate::modules::vector::{Vector3, Vector2Color};

pub trait Projector {
    fn project(&self, point: &Vector3, scale: isize) -> Vector2Color;
}
