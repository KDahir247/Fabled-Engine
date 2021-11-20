#[derive(Copy, Clone, Debug)]
pub enum SpaceType {
    Local,
    World,
}

impl Default for SpaceType {
    fn default() -> Self {
        Self::World
    }
}
