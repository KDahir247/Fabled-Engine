use crate::WorldFlag;

// todo a growable read collection of World
pub struct World {
    value: shipyard::World,
    flags: WorldFlag,
}

pub struct Worlds {
    worlds: Vec<World>,
}

impl Default for Worlds {
    fn default() -> Self {
        let default_injected_world = World {
            value: Default::default(),
            flags: WorldFlag::all(),
        };
        Self {
            worlds: vec![default_injected_world],
        }
    }
}
