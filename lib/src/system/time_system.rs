use crate::component::time_component::DeltaTime;

pub fn calculate_delta_time_system(mut delta_time: shipyard::UniqueViewMut<DeltaTime>) {
    let now = std::time::Instant::now();
    let dt = now - delta_time.last_render_time;
    delta_time.last_render_time = now;

    delta_time.delta = dt;
}
