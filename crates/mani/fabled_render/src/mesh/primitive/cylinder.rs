use crate::mesh::Model;

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
