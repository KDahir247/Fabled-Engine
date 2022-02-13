use crate::WorldFlag;

pub struct World {
    pub value: shipyard::World,
    pub flags: WorldFlag,
}

impl Default for World {
    fn default() -> Self {
        Self {
            value: Default::default(),
            flags: WorldFlag::all(),
        }
    }
}


pub fn create_new_world() -> World {
    World {
        value: Default::default(),
        flags: WorldFlag::all(),
    }
}
