use fabled_render::mesh::{Cone, Model};

mod core;
mod test;

fn main() {
    superluminal_perf::set_current_thread_name("Main_Thread");
    superluminal_perf::begin_event("Staring");

    let cone = Cone::new(1.0, 64, 2., [0.0, 1.0, 0.0]);
    let cone_model: Model = cone.into();

    println!("{:?}", cone_model);

    superluminal_perf::end_event();

    // core::State::run();
}
