mod misc;
mod modules;
mod objects;
mod projectors;
mod traits;

use modules::engine::Engine;
use modules::vector::Vector3;
use objects::cube::Cube;
use projectors::paralel_simple::ParaleleSimpleProjector;
use projectors::paralel_skew::ParalelSkewProjector;
use traits::object::Object;

fn main() {
    let mut engine = Engine::new();

    // let projector = Box::new(ParalelSkewProjector {});
    let projector = Box::new(ParaleleSimpleProjector {});
    engine.set_projector(projector);
    engine.set_fps(30);
    engine.set_projection_scale(30);

    let cube1 = Cube::new(Vector3(0.0, 0.0, 0.0), 1.0);
    engine.add_object(cube1.clone());
    engine.process_scene_to_display();

    loop {
        cube1
            .lock()
            .unwrap()
            .rotate_around_origin(0.1, misc::axis::Axis::Z);
        cube1
            .lock()
            .unwrap()
            .rotate_around_origin(0.1, misc::axis::Axis::X);
        cube1
            .lock()
            .unwrap()
            .rotate_around_origin(0.1, misc::axis::Axis::Y);
        engine.frame();
    }
}
