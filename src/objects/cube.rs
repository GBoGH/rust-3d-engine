use std::sync::{Arc, Mutex};

use crate::modules::vector::Vector3;
use crate::traits::object::Object;

pub struct Cube {
    pub vertices: Vec<Vector3>,
}

impl Cube {
    pub fn new(center: Vector3, side_length: f32) -> Arc<Mutex<Self>> {
        let Vector3(center_x, center_y, center_z) = center;

        let mut vertices: Vec<Vector3> = Vec::new();
        vertices.push(Vector3(
            center_x - side_length / 2.0,
            center_y - side_length / 2.0,
            center_z - side_length / 2.0,
        ));
        vertices.push(Vector3(
            center_x + side_length / 2.0,
            center_y - side_length / 2.0,
            center_z - side_length / 2.0,
        ));
        vertices.push(Vector3(
            center_x + side_length / 2.0,
            center_y + side_length / 2.0,
            center_z - side_length / 2.0,
        ));
        vertices.push(Vector3(
            center_x - side_length / 2.0,
            center_y + side_length / 2.0,
            center_z - side_length / 2.0,
        ));
        vertices.push(Vector3(
            center_x - side_length / 2.0,
            center_y - side_length / 2.0,
            center_z + side_length / 2.0,
        ));
        vertices.push(Vector3(
            center_x + side_length / 2.0,
            center_y - side_length / 2.0,
            center_z + side_length / 2.0,
        ));
        vertices.push(Vector3(
            center_x + side_length / 2.0,
            center_y + side_length / 2.0,
            center_z + side_length / 2.0,
        ));
        vertices.push(Vector3(
            center_x - side_length / 2.0,
            center_y + side_length / 2.0,
            center_z + side_length / 2.0,
        ));

        Arc::new(Mutex::new(Cube { vertices }))
    }
}

impl Object for Cube {
    fn get_vertices(&self) -> &Vec<Vector3> {
        &self.vertices
    }

    fn rotate_around_origin(&mut self, angle: f32, axis: crate::misc::axis::Axis) {
        for vertex in self.vertices.iter_mut() {
            vertex.rotate(angle, &axis);
        }
    }
    fn move_global(&mut self, displacement: &Vector3) {
        for vertex in self.vertices.iter_mut() {
            vertex.add(displacement);
        }
    }
}
