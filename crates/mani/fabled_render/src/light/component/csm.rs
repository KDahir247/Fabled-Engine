use fabled_component::{Component, Modification, Unique};
use fabled_math::{Matrix4x4, Vector3, Vector4};


// There will only be one Cascade map that can be active at any given time.
#[derive(Copy, Clone, PartialEq)]
pub struct CascadeSplit {
    pub splits: Vector4,
    pub lambda: f32,
}

impl Unique for CascadeSplit {
    type Tracking = Modification;
}


#[derive(Copy, Clone, PartialEq)]
pub struct CascadeFrustum {
    pub center: [Vector3; 4],
    pub min_extent: [Vector3; 4],
    pub max_extent: [Vector3; 4],
}

impl Unique for CascadeFrustum {
    type Tracking = Modification;
}
