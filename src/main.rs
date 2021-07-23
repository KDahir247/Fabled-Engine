mod core;
mod test;

fn main() {
    superluminal_perf::set_current_thread_name("Main_Thread"); //todo create profiler feature.

    test::run();
    core::State::run();
}
