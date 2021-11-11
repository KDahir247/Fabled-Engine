#[allow(dead_code)]
pub fn load_test_obj(extension: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut path = String::new();

    path.push_str(env!("CARGO_MANIFEST_DIR"));
    path.push_str("/src/obj/test/");
    path.push('*');
    path.push_str(extension);
    let retrieved_paths = glob::glob(path.as_str()).unwrap();
    for path_result in retrieved_paths {
        paths.push(path_result.unwrap().to_str().unwrap().to_string());
    }
    paths
}
