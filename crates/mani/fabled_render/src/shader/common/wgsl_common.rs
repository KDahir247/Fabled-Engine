pub fn retrieve_test_wgsl_shader() -> Vec<String> {
    let mut paths = Vec::new();
    let mut path = String::new();

    path.push_str(env!("CARGO_MANIFEST_DIR"));
    path.push_str("/src/shader/shader/wgsl/test/**/*.wgsl");
    let a = glob::glob(path.as_str()).unwrap();
    for path_result in a {
        paths.push(path_result.unwrap().to_str().unwrap().to_string());
    }
    paths
}
