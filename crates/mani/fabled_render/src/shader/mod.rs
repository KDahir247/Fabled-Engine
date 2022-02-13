mod common;
mod converter;
mod error;
mod parser;
mod shader_validator;
mod validation_rule;

use crate::shader::common::{
    retrieve_test_glsl_shader, retrieve_test_spv_shader, retrieve_test_wgsl_shader,
};
pub use converter::*;
pub use error::*;
pub use parser::*;
pub use validation_rule::*;


pub fn init_shader_test_env() {
    let wgsl_path = retrieve_test_wgsl_shader();
    let glsl_path = retrieve_test_glsl_shader();
    let spv_path = retrieve_test_spv_shader();

    std::env::set_var("WGSL_FILE", wgsl_path[0].as_str());

    std::env::set_var("SPV_FILE", spv_path[0].as_str());

    std::env::set_var("VERT_FILE", glsl_path[2].as_str());
    std::env::set_var("FRAG_FILE", glsl_path[1].as_str());
    std::env::set_var("COMP_FILE", glsl_path[0].as_str());
}
