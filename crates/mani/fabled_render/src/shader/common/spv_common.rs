// Directory of required glsl shader for the game engine.
// maybe use glob to get all the files that match a pattern.

//  Test Shader
pub const PARSE_TEST_SPV_SHADER: &str = ".\\src\\shader\\compiled\\test\\parse_test.spv";

//  Core Shader
pub fn retrieve_test_spv_shader() -> Vec<String> {
    let mut paths = Vec::new();
    let mut path = String::new();

    path.push_str(env!("CARGO_MANIFEST_DIR"));
    path.push_str("/src/shader/compiled/test/**/*.spv");
    let a = glob::glob(path.as_str()).unwrap();
    for path_result in a {
        paths.push(path_result.unwrap().to_str().unwrap().to_string());
    }
    paths
}
