use bitflags::*;

bitflags! {
    pub struct MissingTy : u8{
        const TRANSLATION = 1;
        const ROTATION = 2;
        const SCALE = 3;
        const SPACE = 4;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Missing {
    pub ty: MissingTy,
}

impl Missing {
    pub fn new(ty: MissingTy) -> Self {
        Self { ty }
    }
}
