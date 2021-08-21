mod capsule;
mod cone;
mod cube;
mod icosphere;
mod plane;
mod quad;
mod torus;
mod uvsphere;

pub use capsule::*;
pub use cone::*;
pub use cube::*;
pub use icosphere::*;
pub use plane::*;
pub use quad::*;
pub use torus::*;
pub use uvsphere::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderInstruction {
    SingleSided = 0,
    DoubleSided = 1,
}

#[cfg(test)]
mod struct_test {
    use crate::mesh::primitive::plane::Plane;
    use crate::mesh::{Capsule, Cone, Cube, IcoSphere, Quad, Torus};

    #[test]
    fn data_size_test() {
        //----------------- Cube ------------------------------
        let cube_size = std::mem::size_of::<Cube>();
        assert_eq!(cube_size & (cube_size - 1), 0);
        let cube_alignment = std::mem::align_of::<Cube>();
        assert_eq!(cube_alignment & (cube_alignment - 1), 0);
        println!("cube is aligned to {} bytes", cube_alignment);

        //------------------ Cone -----------------------------

        let cone_size = std::mem::size_of::<Cone>();
        assert_eq!(cone_size & (cube_size - 1), 0);
        let cone_alignment = std::mem::align_of::<Cone>();
        assert_eq!(cone_alignment & (cube_alignment - 1), 0);
        println!("cone is aligned to {} bytes", cone_alignment);

        //-------------------- Capsule --------------------------

        let capsule_size = std::mem::size_of::<Capsule>();
        assert_eq!(capsule_size & (capsule_size - 1), 0);
        let capsule_alignment = std::mem::align_of::<Capsule>();
        assert_eq!(capsule_alignment & (capsule_alignment - 1), 0);
        println!("capsule is aligned to {} bytes", capsule_alignment);

        //------------------- Plane ------------------------------

        let plane_size = std::mem::size_of::<Plane>();
        assert_eq!(plane_size & (plane_size - 1), 0);
        let plane_alignment = std::mem::align_of::<Plane>();
        assert_eq!(plane_alignment & (plane_alignment - 1), 0);
        println!("plane is aligned to {} bytes", plane_alignment);

        //-------------------- Quad ------------------------------

        let quad_size = std::mem::size_of::<Quad>();
        assert_eq!(quad_size & (quad_size - 1), 0);
        let quad_alignment = std::mem::align_of::<Quad>();
        assert_eq!(quad_alignment & (quad_alignment - 1), 0);
        println!("Quad is aligned to {} bytes", quad_alignment);

        //---------------- Ico Sphere ---------------------------

        let ico_size = std::mem::size_of::<IcoSphere>();
        assert_eq!(ico_size & (ico_size - 1), 0);
        let ico_alignment = std::mem::align_of::<IcoSphere>();
        assert_eq!(ico_alignment & (ico_alignment - 1), 0);
        println!("Ico Sphere is aligned to {} bytes", ico_alignment);

        //-------------------- Torus ------------------------------

        let torus_size = std::mem::size_of::<Torus>();
        assert_eq!(torus_size & (torus_size - 1), 0);
        let torus_alignment = std::mem::align_of::<Torus>();
        assert_eq!(torus_alignment & (torus_alignment - 1), 0);
        println!("Torus is aligned to {} bytes", torus_alignment);
    }
}
