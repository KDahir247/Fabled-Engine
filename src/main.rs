mod core;
mod test;

fn main() {
    /*    superluminal_perf::set_current_thread_name("Main_Thread");

    superluminal_perf::begin_event("Creating Primitive");
    let plane = fabled_render::mesh::Plane::new(
        100.,
        100.,
        100,
        100,
        fabled_render::mesh::PlaneInstruction::SingleSided,
    );
    let plane_model: fabled_render::mesh::Model = plane.into();

    println!("{}", plane_model.meshes[0].material_id);
    superluminal_perf::end_event();*/
    test::run();
    core::State::run();
}
