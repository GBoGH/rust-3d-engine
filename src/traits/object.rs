use crate::modules::vector::Vector3;
use crate::misc::axis::Axis;
pub trait Object{

    fn get_vertices(&self) -> &Vec<Vector3>;
    fn rotate_around_origin(&mut self, angle: f32, axis: Axis);


    fn move_global(&mut self, vector: &Vector3) ;
}