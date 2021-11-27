use crate::SpaceType;

// used in future (editor) for local or world translation, scale and rotation.
#[derive(Copy, Clone, Debug, Default)]
pub struct Space {
    pub value: SpaceType,
}
