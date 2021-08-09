mod core;
mod test;

fn main() {
    superluminal_perf::set_current_thread_name("Main_Thread");

    superluminal_perf::begin_event("Creating Cube");
    let cube = fabled_render::mesh::Cone::new(5., 30, 2., [0., 2., 0.]);
    println!("{:?}", cube);
    superluminal_perf::end_event();
    /*test::run();
    core::State::run();*/
}
