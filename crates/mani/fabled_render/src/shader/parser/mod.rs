mod material_parser;
mod shader_parser;

pub use material_parser::*;
pub use shader_parser::*;

#[cfg(test)]
mod data_alignment_test {
    use crate::shader::parser::*;

    #[test]
    fn data_alignment() {
        let shader_parser = std::mem::size_of::<ShaderParser>();
        assert_eq!(shader_parser & (shader_parser - 1), 0);

        let material_parser = std::mem::size_of::<MaterialParser>();
        println!("material_parser is {} bytes", material_parser);
    }
}
