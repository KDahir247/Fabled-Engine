use fabled_transform::{LocalToWorld, Position, Rotation, Scale};
use shipyard::*;
pub fn local_world_system(
    position: shipyard::View<Position>,
    rotation: shipyard::View<Rotation>,
    scale: shipyard::View<Scale>,
    local_to_world: shipyard::View<LocalToWorld>,
) {
    (&position, &rotation, &scale, !&local_to_world)
        .fast_iter()
        .for_each(|_| {});

    // first system.
    // If has transform, rotation, scale. Add both LocalToWorld and WorldToLocal
    // component.
}
