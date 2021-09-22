use crate::material::*;
use crate::shader::parser::*;
use std::ops::IndexMut;

#[derive(Debug, Serialize, Deserialize)]
#[repr(align(16))]
pub struct MaterialParser {
    material: MaterialTree,
    map: slotmap::SlotMap<MaterialKey, MaterialNode>,
}

impl Default for MaterialParser {
    fn default() -> Self {
        Self {
            material: MaterialTree::new(),
            map: Default::default(),
        }
    }
}

impl MaterialParser {
    pub fn parse_material<P: AsRef<std::path::Path>>(&mut self, path: P) -> String {
        // handle case here where it a user error (invalid path specified) and not a
        // internal bug.
        let (module, _) = if let Ok(val) = parse_shader(path, None) {
            val
        } else {
            return String::new();
        };


        let naga::Module {
            types,
            global_variables,
            ..
        } = module;

        let globals = global_variables.into_inner();

        for global_with_binding in globals.iter().filter(|glob_var| glob_var.binding.is_some()) {
            let type_var = types
                .try_get(global_with_binding.ty)
                .expect("type arena len is less then the handle of type index specified and is out of bounds");

            let res_binding = global_with_binding.binding.as_ref().expect("Global Binding for variable has previous pass check of not being None and and has been evaluated as None after the check.");

            if let naga::TypeInner::Struct {
                top_level, members, ..
            } = &type_var.inner
            {
                if *top_level {
                    let (group, binding) = (res_binding.group, res_binding.binding);

                    for member in members {
                        if let Some(member_ty) = types.try_get(member.ty) {
                            let material_node = MaterialNode {
                                value_group: group,
                                value_binding: binding,
                                ty: MaterialTarget::from(&member_ty.inner),
                            };

                            let mat_attr: Option<MaterialAttributes> = material_node.ty.into();

                            if let Some(attr) = mat_attr {
                                let index = attr as usize;
                                let material_key = self.map.insert(material_node);
                                self.material.index_mut(index).add_to_keys(material_key);
                            }
                        }
                    }
                } else {
                    let struct_name = type_var
                        .name
                        .as_ref()
                        .expect("One of the shader struct name is consider empty.");

                    println!("Skipped struct name due to having nested struct types as member variable/s.\n The struct in questioning is named : {}", struct_name);
                }
            }

            // Handle case where the binding variable in the shader is a sampler or a
            // texture.
            let (group, binding) = (res_binding.group, res_binding.binding);

            let material_node = MaterialNode {
                value_group: group,
                value_binding: binding,
                ty: MaterialTarget::from(&type_var.inner),
            };

            let mat_attr: Option<MaterialAttributes> = material_node.ty.into();

            if let Some(attr) = mat_attr {
                let index = attr as usize;
                let material_key = self.map.insert(material_node);
                self.material.index_mut(index).add_to_keys(material_key);
            }
        }

        let pretty = ron::ser::PrettyConfig::new()
            .with_separate_tuple_members(true)
            .with_decimal_floats(true)
            .with_depth_limit(7)
            .with_enumerate_arrays(true);

        ron::ser::to_string_pretty(&self, pretty).expect("failed to serialize shader data into a material representation.\nMaterial file should be utf-8")
    }

    pub fn get_group(&self, index: u32) -> Vec<(MaterialKey, MaterialNode)> {
        self.map
            .to_owned()
            .into_iter()
            .filter(|key_value| key_value.1.value_group.eq(&index))
            .collect::<Vec<_>>()
    }

    pub fn get_binding(&self, index: u32) -> Vec<(MaterialKey, MaterialNode)> {
        self.map
            .to_owned()
            .into_iter()
            .filter(|key_value| key_value.1.value_binding.eq(&index))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod material_test {

    use crate::shader::init_shader_test_env;
    use crate::shader::parser::*;

    #[test]
    fn display_material() {
        init_shader_test_env();

        // ----------------------- WEB GPU -----------------------

        let wgsl_path = std::env::var("WGSL_FILE").unwrap();
        let wgsl_path = std::path::Path::new(wgsl_path.as_str());

        let mut material_wgsl_parser = MaterialParser::default();

        let wgsl_tree = material_wgsl_parser.parse_material(wgsl_path);

        println!("WGSL TREE:\n{}\n\n", wgsl_tree);

        // ----------------------- SPIR-V -----------------------

        let spv_path = std::env::var("SPV_FILE").unwrap();
        let spv_path = std::path::Path::new(spv_path.as_str());

        let mut material_spv_parser = MaterialParser::default();

        let spv_tree = material_spv_parser.parse_material(spv_path);
        println!("SPV TREE:\n{}\n\n", spv_tree);

        // ----------------------- GLSL Vertex -----------------------

        let vertex_path = std::env::var("VERT_FILE").unwrap();
        let vertex_path = std::path::Path::new(vertex_path.as_str());

        let mut material_vert_parser = MaterialParser::default();

        let vertex_tree = material_vert_parser.parse_material(vertex_path);

        println!("GLSL VERTEX TREE:\n{}\n\n", vertex_tree);

        // ----------------------- GLSL Fragment -----------------------

        let frag_path = std::env::var("FRAG_FILE").unwrap();
        let frag_path = std::path::Path::new(frag_path.as_str());

        let mut material_frag_parser = MaterialParser::default();

        let fragment_tree = material_frag_parser.parse_material(frag_path);

        println!("GLSL FRAGMENT TREE:\n{}\n\n", fragment_tree);

        // ----------------------- GLSL Compute -----------------------

        let compute_path = std::env::var("COMP_FILE").unwrap();
        let compute_path = std::path::Path::new(compute_path.as_str());

        let mut material_comp_parser = MaterialParser::default();

        let compute_tree = material_comp_parser.parse_material(compute_path);

        println!("GLSL COMPUTE TREE:\n{}\n\n", compute_tree);
    }

    #[test]
    fn display_group() {
        init_shader_test_env();

        let vertex_path = std::env::var("VERT_FILE").unwrap();
        let vertex_path = std::path::Path::new(vertex_path.as_str());

        let mut material_parser = MaterialParser::default();

        let vertex_tree = material_parser.parse_material(vertex_path);

        println!("GLSL VERTEX TREE:\n{}\n\n", vertex_tree);

        let struct_members = material_parser.get_group(0);

        assert!(!struct_members.is_empty());

        for (index, member) in struct_members.iter().enumerate() {
            println!("{} variable in struct is {:?}", index, member.1);
        }
    }

    #[test]
    fn display_binding() {
        init_shader_test_env();

        let vertex_path = std::env::var("VERT_FILE").unwrap();
        let vertex_path = std::path::Path::new(vertex_path.as_str());

        let mut material_parser = MaterialParser::default();

        let vertex_tree = material_parser.parse_material(vertex_path);

        println!("GLSL VERTEX TREE:\n{}\n\n", vertex_tree);

        let binding_members = material_parser.get_binding(0);

        assert!(!binding_members.is_empty());

        for binding in binding_members.iter() {
            println!("{:?}", binding.1);
        }
    }
}
