use crate::mesh::Model;

pub struct Torus {}

impl Default for Torus {
    fn default() -> Self {
        Self::new()
    }
}

impl Torus {
    pub fn new() -> Torus {
        Self {}
    }
}

impl From<Torus> for Model {
    fn from(torus: Torus) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
