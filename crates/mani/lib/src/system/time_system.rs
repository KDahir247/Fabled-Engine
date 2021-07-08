use crate::component::time_component::Time;

pub fn calculate_delta_time_system(mut delta_time: shipyard::UniqueViewMut<Time>) {
    let elapsed_time = delta_time.application.time.elapsed().as_secs_f64();
    delta_time.delta = elapsed_time - delta_time.last_frame;
    delta_time.last_frame = elapsed_time;
}
