mod core;

fn main() {
    superluminal_perf::set_current_thread_name("Main_Thread"); //todo create profiler feature.

    core::State::run();
}
