mod core;
mod test;

fn main() {
    /*    superluminal_perf::set_current_thread_name("Main_Thread");

    superluminal_perf::begin_event("Creating Primitive");

    let plane = fabled_render::mesh::Plane::new(
        1.0,
        1.0,
        10,
        10,
        fabled_render::mesh::PlaneInstruction::SingleSided,
    );
    let plane_model: fabled_render::mesh::Model = plane.into();

    println!("a");
    superluminal_perf::end_event();*/
    test::run();
    core::State::run();
}
