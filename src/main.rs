mod core;
mod test;

fn main() {
    superluminal_perf::set_current_thread_name("Main_Thread");
    // superluminal_perf::begin_event("Staring");
    //
    // fabled_fbx::V7400Loader::loads(
    // "C:/Users/kdahi/Downloads/jun-goto-tenshi-no-3p/source/Jun Goto/Jun
    // Goto.fbx", );
    // superluminal_perf::end_event();

    core::State::run();
}
