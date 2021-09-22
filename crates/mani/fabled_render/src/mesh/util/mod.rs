mod mesh_util;

pub use mesh_util::*;

pub enum NormalInstruction {
    Flat,
    Smooth,
}

impl Default for NormalInstruction {
    fn default() -> Self {
        NormalInstruction::Smooth
    }
}

#[cfg(test)]
mod data_test {
    use crate::mesh::util::NormalInstruction;

    #[test]
    fn data_size_test() {
        let normal_instruction_size = std::mem::size_of::<NormalInstruction>();
        assert_eq!(normal_instruction_size & (normal_instruction_size - 1), 0);
    }

    #[test]
    fn align_size_test() {
        let normal_instruction_align = std::mem::align_of::<NormalInstruction>();
        assert_eq!(normal_instruction_align & (normal_instruction_align - 1), 0);
    }
}
