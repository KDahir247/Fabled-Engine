mod capsule;
mod cone;
mod cube;
mod cylinder;
mod icosphere;
mod plane;
mod quad;
mod sphere;
mod torus;
mod uvsphere;

pub use capsule::*;
pub use cone::*;
pub use cube::*;

#[cfg(test)]
mod struct_test {
    use crate::mesh::{Capsule, Cone, Cube};

    #[test]
    fn data_size_test() {
        let cube_size = std::mem::size_of::<Cube>();
        assert_eq!(cube_size & (cube_size - 1), 0);
        let cube_alignment = std::mem::align_of::<Cube>();
        assert_eq!(cube_alignment & (cube_alignment - 1), 0);
        println!("cube is aligned to {} bytes", cube_alignment);

        let cone_size = std::mem::size_of::<Cone>();
        assert_eq!(cone_size & (cube_size - 1), 0);
        let cone_alignment = std::mem::align_of::<Cone>();
        assert_eq!(cone_alignment & (cube_alignment - 1), 0);
        println!("cone is aligned to {} bytes", cone_alignment);

        let capsule_size = std::mem::size_of::<Capsule>();
        assert_eq!(capsule_size & (capsule_size - 1), 0);
        let capsule_alignment = std::mem::align_of::<Capsule>();
        assert_eq!(capsule_alignment & (capsule_alignment - 1), 0);
        println!("capsule is aligned to {} bytes", capsule_alignment);
    }
}
