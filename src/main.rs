mod core;

fn main() {
    #[cfg(feature = "profiler")]
    superluminal_perf::set_current_thread_name("Main_Thread"); //todo create profiler feature.

    core::State::run();
}
