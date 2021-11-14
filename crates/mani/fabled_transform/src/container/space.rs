#[derive(Copy, Clone, Debug)]
pub enum Space {
    Local,
    World,
}

impl Default for Space {
    fn default() -> Self {
        Self::Local
    }
}
