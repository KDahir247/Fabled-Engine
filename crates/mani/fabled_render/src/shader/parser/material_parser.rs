use crate::material::*;
use crate::shader;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialParser {
    material: MaterialTree,
    //This will have a number of values for each type variable of a material maybe.
    //It will Have an Arena of Type and the Material Tree Attribute Handle will have the handles for the type.
    map: slotmap::SlotMap<MaterialKey, MaterialNode>,
}

impl Default for MaterialParser {
    fn default() -> Self {
        Self {
            material: MaterialTree::new("".to_string()),
            map: Default::default(),
        }
    }
}

impl MaterialParser {
    pub fn create_material_hierarchy<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> anyhow::Result<String> {
        let name = match path.as_ref().file_name() {
            None => "shader",
            Some(file_name) => file_name.to_str().unwrap_or("shader"),
        };

        self.material.shader = name.to_string();

        let shader = shader::parser::ShaderParser::parse(path)?;

        let naga::Module {
            types,
            global_variables,
            ..
        } = shader.module;

        let globals = global_variables.into_inner();

        for global in globals.iter().filter(|glob_var| glob_var.binding.is_some()) {
            let type_var = types.try_get(global.ty).unwrap();

            if let naga::TypeInner::Struct {
                top_level,
                members,
                span,
            } = &type_var.inner
            {
                if *top_level {
                    assert!(global.binding.is_some(), "Global Binding for variable has previous pass check of not being None and and has been evaluated as None after the check.");

                    let res_binding = global.binding.as_ref().unwrap();
                    let (group, binding) = (res_binding.group, res_binding.binding);

                    for member in members {
                        if let Some(ty) = types.try_get(member.ty) {
                            //we will append it the the key node.
                            let material_node = MaterialNode {
                                value_group: Some(group),
                                value_binding: Some(binding),
                                value: MaterialTarget::from(&ty.inner),
                            };

                            let id = self.material.get(material_node.value.into());

                            if let Some(id) = id {
                                let material_key = self.map.insert(material_node);
                                self.material.branch[id].keys.push(material_key);
                            }
                        }
                    }
                } else {
                    // we want to maybe print something to the user. for eg. "skipped struct name due to having nested struct types as member variable/s".
                }
            }

            // Handle case where the binding variable in the shader is a sampler or a texture.
            let node = Self::create_node(global.to_owned(), &type_var.inner);
            let id = self.material.get(node.value.into());

            if let Some(id) = id {
                let material_key = self.map.insert(node);
                self.material.branch[id].keys.push(material_key);
            }
        }

        let pretty = ron::ser::PrettyConfig::new()
            .with_separate_tuple_members(true)
            .with_decimal_floats(true)
            .with_depth_limit(5)
            .with_enumerate_arrays(true);

        let ron_material = ron::ser::to_string_pretty(&self, pretty).expect("failed to serialize shader data into a material representation.\n failed on writing out the data to ron format.");

        Ok(ron_material)
    }

    fn create_node(global_var: naga::GlobalVariable, inner: &naga::TypeInner) -> MaterialNode {
        let node_type = inner;

        let resource = global_var.binding;
        let (group, binding) = match resource {
            None => (None, None),
            Some(resource) => (Some(resource.group), Some(resource.binding)),
        };

        MaterialNode {
            value: MaterialTarget::from(node_type),
            value_group: group,
            value_binding: binding,
        }
    }
}

// ------------- Test --------------

#[cfg(test)]
mod material_test {

    use crate::shader::init_shader_test_env;
    use crate::shader::parser::*;

    #[test]
    fn display_material() {
        init_shader_test_env();

        let wgsl_path = std::env::var("WGSL_FILE").unwrap();
        let wgsl_path = std::path::Path::new(wgsl_path.as_str());

        let mut material_wgsl_parser = MaterialParser::default();
        let wgsl_tree = material_wgsl_parser
            .create_material_hierarchy(wgsl_path)
            .unwrap();

        println!("WGSL TREE:\n{}\n\n", wgsl_tree);

        let spv_path = std::env::var("SPV_FILE").unwrap();
        let spv_path = std::path::Path::new(spv_path.as_str());

        let mut material_spv_parser = MaterialParser::default();

        let spv_tree = material_spv_parser
            .create_material_hierarchy(spv_path)
            .unwrap();
        println!("SPV TREE:\n{}\n\n", spv_tree);

        let vertex_path = std::env::var("VERT_FILE").unwrap();
        let vertex_path = std::path::Path::new(vertex_path.as_str());

        let mut material_vert_parser = MaterialParser::default();
        let vertex_tree = material_vert_parser
            .create_material_hierarchy(vertex_path)
            .unwrap();

        println!("GLSL VERTEX TREE:\n{}\n\n", vertex_tree);

        let frag_path = std::env::var("FRAG_FILE").unwrap();
        let frag_path = std::path::Path::new(frag_path.as_str());

        let mut material_frag_parser = MaterialParser::default();

        let fragment_tree = material_frag_parser
            .create_material_hierarchy(frag_path)
            .unwrap();

        println!("GLSL FRAGMENT TREE:\n{}\n\n", fragment_tree);

        let compute_path = std::env::var("COMP_FILE").unwrap();
        let compute_path = std::path::Path::new(compute_path.as_str());

        let mut material_comp_parser = MaterialParser::default();

        let compute_tree = material_comp_parser
            .create_material_hierarchy(compute_path)
            .unwrap();

        println!("GLSL COMPUTE TREE:\n{}\n\n", compute_tree);
    }
}
