// Feature lists for shader:
// Maybe add a shader query system where you can pass a generic and it will return all the data for that generic from the shader.
// Maybe add a shader cache for the shader module??, but definitely add a common with will hold all the directory path to the common shader for the game engine.
// Maybe add benching for the shader scripts.

pub mod common;
pub mod converter;
pub mod parser;
pub mod reflection;
pub mod shader_validator;
mod validation_rule;

pub use validation_rule::*;
