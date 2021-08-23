mod material_parser;
mod shader_parser;

pub use material_parser::*;
pub use shader_parser::*;

#[cfg(test)]
mod data_test {
    use crate::shader::parser::*;

    #[test]
    fn data_size() {
        let shader_parser_size = std::mem::size_of::<ShaderParser>();
        println!("shader_parser is {} bytes", shader_parser_size);

        let material_parser_size = std::mem::size_of::<MaterialParser>();
        println!("material_parser is {} bytes", material_parser_size);
    }

    #[test]
    fn data_alignment() {
        let shader_parser_alignment = std::mem::align_of::<ShaderParser>();
        println!(
            "shader_parser is aligned to {} bytes",
            shader_parser_alignment
        );

        let material_parser_alignment = std::mem::align_of::<MaterialParser>();
        println!(
            "material_parser is aligned to {} bytes",
            material_parser_alignment
        );
    }
}
