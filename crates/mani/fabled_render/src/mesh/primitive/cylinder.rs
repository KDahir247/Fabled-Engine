use crate::mesh::Model;

/*
    Thought of Variables.
    Planning Phase
    Top radius
    bottom radius
    height
    radial segment
    rings
    Axis,
    Center,
    Radius,

    Sanity check for Top radius and bottom radius to prevent less than or equal to zero.
*/
pub struct Cylinder {}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new()
    }
}

impl Cylinder {
    pub fn new() -> Cylinder {
        Self {}
    }
}

impl From<Cylinder> for Model {
    fn from(cylinder: Cylinder) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
