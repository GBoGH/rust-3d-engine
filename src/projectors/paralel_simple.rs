use crate::modules::vector::{ Vector3, Vector2Color};
use crate::traits::projector::Projector;
pub struct ParaleleSimpleProjector {}

impl Projector for ParaleleSimpleProjector {
    fn project(&self, point: &Vector3, scale: isize) -> Vector2Color {
        let Vector3(x, y, _) = *point;
        let typed_scale = scale as f32;
        Vector2Color(x*typed_scale, y*typed_scale, 50)
    }
}
