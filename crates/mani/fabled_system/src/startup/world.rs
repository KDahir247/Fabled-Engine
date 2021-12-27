use crate::WorldFlag;


pub struct World {
    pub value: shipyard::World,
    pub command: u32, /* this will change to a command struct to add entity or remove or
                       * modify the world. */
    flags: WorldFlag,
}
