mod common;
mod converter;
mod parser;
mod reflection;
mod shader_validator;
mod validation_rule;

use crate::shader;
pub use validation_rule::*;

pub fn init_shader_test_env() {
    std::env::set_var("WGSL_FILE", shader::common::PARSE_TEST_WGSL_SHADER);
    std::env::set_var("SPV_FILE", shader::common::PARSE_TEST_SPV_SHADER);

    std::env::set_var("VERT_FILE", shader::common::PARSE_TEST_VERTEX_SHADER);
    std::env::set_var("FRAG_FILE", shader::common::PARSE_TEST_FRAGMENT_SHADER);
    std::env::set_var("COMP_FILE", shader::common::PARSE_TEST_COMPUTE_SHADER);
}
