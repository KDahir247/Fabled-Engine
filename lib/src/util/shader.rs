//todo need to work on.
/*use wgpu::ShaderModuleDescriptor;

pub enum ShaderType {
    SPIRV,
    WGSL,
}

//got to remember to get CARGO_MANIFEST_DIR

//todo it want to parse all the full file data.
pub fn create_shader_module<'a>(
    name: std::option::Option<&'a str>,
    path: &'a str,
    shader_type: ShaderType,
    flag: wgpu::ShaderFlags,
) -> wgpu::ShaderModuleDescriptor<'a> {
    let current_directory = env!("CARGO_MANIFEST_DIR")
        .parse::<String>()
        .expect("parsing failed : CARGO_MANIFEST_DIR failed to parse!!");

    let shader_path = format!("{}/{}", current_directory, path);
    let shader = match shader_type {
        WGSL => Some(wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(path))),
        _ => None,
    }
    .unwrap();

    wgpu::ShaderModuleDescriptor {
        label: name,
        source: shader,
        flags: flag,
    }
}
*/