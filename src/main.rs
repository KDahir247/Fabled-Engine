mod core;
mod test;

fn main() {
    /*    superluminal_perf::set_current_thread_name("Main_Thread");

    superluminal_perf::begin_event("Creating Primitive");

    let quad = fabled_render::mesh::Quad::new(1000., 1000., 1000.);
    let quad_model: fabled_render::mesh::Model = quad.into();

    println!("{:?}", quad_model.meshes[0].material_id);

    superluminal_perf::end_event();*/
    test::run();
    core::State::run();
}
