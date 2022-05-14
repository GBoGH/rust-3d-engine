use crate::modules::vector::{Vector2Color, Vector3};
use crate::traits::projector::Projector;
pub struct ParalelSkewProjector {}

impl Projector for ParalelSkewProjector {
    fn project(&self, point: &Vector3, scale: isize) -> Vector2Color {
        let Vector3(x, y, z) = *point;
        let typed_scale = scale as f32;
        Vector2Color(
            x * typed_scale + z * typed_scale,
            y * typed_scale - z * typed_scale,
            1,
        )
    }
}
