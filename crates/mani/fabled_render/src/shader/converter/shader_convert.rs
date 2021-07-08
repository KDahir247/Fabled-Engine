use crate::shader::converter::*;
use crate::shader::validation_rule::ValidationLayer;

pub fn convert_shader(
    conversion: ShaderConvertOption,
    shader_module: &naga::Module,
) -> anyhow::Result<ShaderConvertResult> {
    let shader_info = shader_module.validate(naga::valid::ValidationFlags::all())?;

    let conversion_res = match conversion {
        ShaderConvertOption::Wgsl => {
            let wgsl_buffer = naga::back::wgsl::write_string(shader_module, &shader_info)?;

            ShaderConvertResult::Wgsl(wgsl_buffer)
        }
        ShaderConvertOption::Spv { option } => {
            let mut spv_option = naga::back::spv::Options::default();

            if let SpvOptions::Custom {
                maj_min,
                //capabilities,
            } = option
            {
                //spv_option.capabilities = capabilities;
                spv_option.lang_version = maj_min;
            };

            let spv = naga::back::spv::write_vec(shader_module, &shader_info, &spv_option)?;
            let bytes = spv
                .iter()
                .fold(Vec::with_capacity(spv.len() * 4), |mut acc, index| {
                    acc.extend_from_slice(&index.to_le_bytes());
                    acc
                });

            ShaderConvertResult::Spv(bytes)
        }
        ShaderConvertOption::Glsl { version } => {
            match version {
                Version::Desktop(desk_prep) => {
                    assert!(
                        desk_prep <= *naga::back::glsl::SUPPORTED_CORE_VERSIONS.last().unwrap() //todo better error
                    );
                }
                Version::Embedded(embed_prep) => {
                    assert!(embed_prep <= *naga::back::glsl::SUPPORTED_ES_VERSIONS.last().unwrap());
                    //todo better error
                }
            };

            let mut glsl_buffer = String::new();

            for entry in shader_module.entry_points.iter() {
                let glsl_option = naga::back::glsl::Options {
                    version: version.into(),
                    shader_stage: entry.stage,
                    entry_point: entry.name.to_owned(),
                };

                let mut writer = naga::back::glsl::Writer::new(
                    &mut glsl_buffer,
                    shader_module,
                    &shader_info,
                    &glsl_option,
                )?;

                writer.write()?;
            }

            ShaderConvertResult::Glsl(glsl_buffer)
        }
    };

    Ok(conversion_res)
}
