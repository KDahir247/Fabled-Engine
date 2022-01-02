mod core;
mod test;

fn main() {
    superluminal_perf::set_current_thread_name("Main_Thread");

    superluminal_perf::begin_event("Staring");

    fabled_fbx::load("C:/Users/kdahi/Downloads/goultard-chibi/source/GoultardtoDecimated.fbx");
    superluminal_perf::end_event();

    // core::State::run();
}
