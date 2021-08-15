use crate::mesh::Model;

/*
    Planning Phase.

*/
pub struct Sphere {
    pub radius: f32,
    pub tessellation: i32,
}

impl Default for Sphere {
    /*
        Assumption on the
    */
    fn default() -> Self {
        Self::new(0.5, 16)
    }
}

impl Sphere {
    pub fn new(radius: f32, tessellation: i32) -> Sphere {
        Self {
            radius,
            tessellation,
        }
    }
}

impl From<Sphere> for Model {
    fn from(sphere: Sphere) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
