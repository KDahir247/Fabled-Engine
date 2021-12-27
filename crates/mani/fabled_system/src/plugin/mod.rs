use crate::{
    calculate_local_world_parent_system, calculate_local_world_system,
    removed_deleted_transform_system,
};

use shipyard_app::*;


#[derive(Default)]
struct TransformInternalPlugin;

impl Plugin for TransformInternalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(removed_deleted_transform_system);
    }
}


#[derive(Default)]
pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TransformInternalPlugin::default())
            .add_system(calculate_local_world_system)
            .add_system(calculate_local_world_parent_system);
    }
}
