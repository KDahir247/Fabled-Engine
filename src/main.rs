use fabled_render::mesh::{Cone, Model};

mod core;
mod test;

fn main() {
    // superluminal_perf::set_current_thread_name("Main_Thread");
    // superluminal_perf::begin_event("Staring");
    //
    // let a=  fabled_render::shader::MaterialParser::default().parse_material("D:\\
    // Study\\Fabled
    // Engine\\crates\\mani\\fabled_render\\src\\shader\\shader\\wgsl\\test\\
    // parse_test.wgsl" );
    //
    // println!("WGSL TREE:\n {}", a);
    //
    // superluminal_perf::end_event();

    core::State::run();
}
