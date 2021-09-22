#[rustfmt::skip]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::const_mat4!(
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.5, 1.0]
);

pub const CLEAR_COLOR: f64 = 0.09;

pub fn invalid_map_path() -> String {
    let mut cargo_directory = env!("CARGO_MANIFEST_DIR").to_string();
    cargo_directory.push_str("/resource/map/Default_Map.png");
    cargo_directory
}
