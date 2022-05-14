use std::sync::{Arc, Mutex};
use std::{time::Duration,thread::sleep};

use crate::{traits::{object::Object, projector::Projector}, modules::vector::Vector2Color};

use super::{display::Display, vector::Vector2};

use crate::projectors::paralel_simple::ParaleleSimpleProjector;

pub struct Engine {
    pub display: Display,
    pub projector: Box<dyn Projector>,
    pub objects: Vec<Arc<Mutex<dyn Object>>>,
    pub projection_scale: isize,
    pub fps: u32,
}

impl Engine {
    pub fn new() -> Self {
        let display = Display::new(40, 40);
        let defaul_projector = Box::new(ParaleleSimpleProjector {});
        let objects: Vec<Arc<Mutex<dyn Object>>> = Vec::new();
        let projection_scale = 5;
        let fps = 10;
        Engine {
            display,
            objects,
            projector: defaul_projector,
            projection_scale,
            fps
        }
    }

    pub fn add_object(&mut self, object: Arc<Mutex<dyn Object>>) {
        self.objects.push(object);
    }

    pub fn set_projection_scale(&mut self, scale: isize) {
        self.projection_scale = scale;
    }

    pub fn set_projector(&mut self, projector: Box<dyn Projector>) {
        self.projector = projector;
    }

    pub fn count_objects(&self) -> usize {
        self.objects.len()
    }
    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
    }

    pub fn process_scene_to_display(&mut self) {
        self.display.clear();
        for object in self.objects.iter() {
            for vertex in object.lock().unwrap().get_vertices() {
                let colored_vector = self.projector.project(vertex, self.projection_scale);
                self.display.draw_point(colored_vector);
            }
        }
    }

    pub fn frame(&mut self) {
        self.process_scene_to_display();
        self.display.draw_to_terminal();

        let sleep_duration = 1000 / self.fps;
        sleep(Duration::from_millis(sleep_duration as u64));

    }
}
