use bitflags::*;

bitflags! {
    pub struct WorldFlag : u8{
        const NONE = 1;
        const EDITOR = 2;
        const GAME = 4;
        const SIMULATION = 8;
        const LIVE = 16;
    }
}
