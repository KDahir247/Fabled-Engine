mod material_parser;
mod shader_parser;

pub use material_parser::*;
pub use shader_parser::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseOption {
    SPV {
        /// The IR coordinate space matches all the APIs except SPIR-V,
        /// so by default we flip the Y coordinate of the `BuiltIn::Position`.
        /// This flag can be used to avoid this.
        adjust_coordinate_space: bool,
        /// Only allow shaders with the known set of capabilities.
        strict_capabilities: bool,
    },
    Glsl {
        entry_point: String,
    },
}

#[cfg(test)]
mod data_test {
    use crate::shader::parser::*;

    #[test]
    fn data_size() {
        let parse_option_size = std::mem::size_of::<ParseOption>();
        assert_eq!(parse_option_size & (parse_option_size - 1), 0);

        let material_parser_size = std::mem::size_of::<MaterialParser>();
        assert_eq!(material_parser_size & (material_parser_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let parse_option_alignment = std::mem::align_of::<ParseOption>();
        assert_eq!(parse_option_alignment & (parse_option_alignment - 1), 0);

        let material_parser_alignment = std::mem::align_of::<MaterialParser>();
        assert_eq!(
            material_parser_alignment & (material_parser_alignment - 1),
            0
        );
    }
}
