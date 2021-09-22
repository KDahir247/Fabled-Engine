mod core;
mod test;

fn main() {
    superluminal_perf::set_current_thread_name("Main_Thread");

    //  superluminal_perf::begin_event("Staring");
    //
    // superluminal_perf::end_event();

    core::State::run();
}
