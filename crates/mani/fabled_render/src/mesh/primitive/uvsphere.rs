use crate::mesh::Model;

/*
    Planning Phase.
    Longitude and Latitude?.
    Center?
    rings?
    height?
    width?

*/
pub struct UvSphere {
    pub radius: f32,
    pub tessellation: i32,
}

impl Default for UvSphere {
    fn default() -> Self {
        Self::new(0.5, 16)
    }
}

impl UvSphere {
    pub fn new(radius: f32, tessellation: i32) -> UvSphere {
        Self {
            radius,
            tessellation,
        }
    }
}

impl From<UvSphere> for Model {
    fn from(uv_sphere: UvSphere) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
