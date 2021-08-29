#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Orientation {
    //todo this will go in math module later.
    pub transformation_matrix: [f32; 16],
    pub forward: [f32; 4],
    pub right: [f32; 4],
}

//Can be decided if the look at is lh or rh in this script.
