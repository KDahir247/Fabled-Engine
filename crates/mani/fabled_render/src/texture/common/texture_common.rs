pub fn load_test_textures(extension: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut path = String::new();

    path.push_str(env!("CARGO_MANIFEST_DIR"));
    path.push_str("/src/texture/texture/test/albedo/");
    path.push('*');
    path.push_str(extension);
    let a = glob::glob(path.as_str()).unwrap();
    for path_result in a {
        paths.push(path_result.unwrap().to_str().unwrap().to_string());
    }
    paths
}

// todo work here
pub fn save_test_texture(relative: &str) -> String {
    let mut path = String::new();

    path.push_str(env!("CARGO_MANIFEST_DIR"));
    path.push_str("/src/texture/texture/");
    path.push_str(relative);
    path
}
