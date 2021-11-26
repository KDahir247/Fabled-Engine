use fabled_transform::{Rotation, Scale, Space, Translation};
use shipyard::ViewMut;

pub fn transformation_track_system(
    mut translation: ViewMut<Translation>,
    mut rotation: ViewMut<Rotation>,
    mut scale: ViewMut<Scale>,
    mut space: ViewMut<Space>,
) {
    translation.track_insertion().track_modification();
    rotation.track_insertion().track_modification();
    scale.track_insertion().track_modification();
    space.track_insertion().track_modification();
}


pub fn deleted_track_system(
    mut translation: ViewMut<Translation>,
    mut rotation: ViewMut<Rotation>,
    mut scale: ViewMut<Scale>,
    mut space: ViewMut<Space>,
) {
    translation.track_removal().track_deletion();
    rotation.track_removal().track_deletion();
    scale.track_removal().track_deletion();
    space.track_removal().track_deletion();
}
