use crate::material::*;
use crate::shader::parser::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::{BitAnd, IndexMut};

// todo size issue now.
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialParserMetadata {
    material: MaterialTree,
    map: slotmap::DenseSlotMap<MaterialKey, MaterialNode>,
}
impl Default for MaterialParserMetadata {
    fn default() -> Self {
        Self {
            material: MaterialTree::new(),
            map: Default::default(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MaterialParser {
    metadata: parking_lot::RwLock<MaterialParserMetadata>,
}


impl MaterialParser {
    pub fn new() -> Self {
        Self {
            metadata: parking_lot::const_rwlock(Default::default()),
        }
    }

    pub fn parse_material<P: AsRef<std::path::Path>>(&mut self, path: P) -> String {
        // handle case here where it a user error (invalid path specified) and not a
        // internal bug.
        let shader_module =
            parse_shader(path, None).map_or(naga::Module::default(), |(module, _)| module);

        let naga::Module {
            types,
            global_variables,
            ..
        } = shader_module;

        let globals_variables = global_variables.into_inner();

        globals_variables
            .into_par_iter()
            .filter(|module_glob_var| {
                let material_val = MaterialValue::from(&types[module_glob_var.ty].inner);

                let a = material_val.get_primitive_ty().is_some();
                module_glob_var
                    .binding
                    .is_some()
                    .bitand(material_val.get_primitive_ty().is_some())
            })
            .for_each(|resource_bounded_var| unsafe {
                let resource_binding_type_var = &types[resource_bounded_var.ty];


                // We can guarantee that unwrap will succeed, since we filter for
                // binding.is_some()
                let res_binding = resource_bounded_var.binding.unwrap();

                let (value_group, value_binding) = (res_binding.group, res_binding.binding);

                let mut material_nodes = Vec::with_capacity(20);

                if let naga::TypeInner::Struct {
                    members: struct_members,
                    ..
                } = &resource_binding_type_var.inner
                {
                    for struct_member in struct_members {
                        let struct_member_material_node = MaterialNode {
                            value_group,
                            value_binding,
                            value_detail: MaterialValue::from(&types[struct_member.ty].inner),
                        };


                        material_nodes.push(struct_member_material_node);
                    }
                }

                // Handle case where the binding variable in the shader is a sampler or a
                // texture.
                let material_node = MaterialNode {
                    value_group,
                    value_binding,
                    value_detail: MaterialValue::from(&resource_binding_type_var.inner),
                };

                material_nodes.push(material_node);

                for node in material_nodes
                    .into_iter()
                    .filter(|x| x.value_detail.value_ty.ne(&MaterialValueType::None))
                {
                    // We can guarantee that get_primitive_ty will never return a None since we
                    // filtered out all None.

                    let material_ty = node.value_detail.get_primitive_ty().unwrap();

                    let primitive_ty_index = material_ty as usize;

                    {
                        // blocking.
                        let mut write_rw_guard = self.metadata.write();

                        let material_key = write_rw_guard.map.insert(node);

                        write_rw_guard
                            .material
                            .index_mut(primitive_ty_index)
                            .add_to_keys(material_key);
                    }
                }
            });

        let pretty = ron::ser::PrettyConfig::new()
            .decimal_floats(true)
            .depth_limit(5)
            .enumerate_arrays(true);

        ron::ser::to_string_pretty(&self, pretty).expect("failed to serialize shader data into a material representation.\nMaterial file should be utf-8")
    }

    pub fn get_group(&self, index: u32) -> Vec<(MaterialKey, MaterialNode)> {
        let reader_rw_guard = self.metadata.read();

        reader_rw_guard
            .map
            .to_owned()
            .into_iter()
            .filter(|key_value| key_value.1.value_group.eq(&index))
            .collect::<Vec<_>>()
    }

    pub fn get_binding(&self, index: u32) -> Vec<(MaterialKey, MaterialNode)> {
        let reader_rw_guard = self.metadata.read();

        reader_rw_guard
            .map
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

        // let wgsl_path = std::env::var("WGSL_FILE").unwrap();
        // let wgsl_path = std::path::Path::new(wgsl_path.as_str());
        //
        // println!("{:?}", wgsl_path);
        // let mut material_wgsl_parser = MaterialParser::default();
        //
        // let wgsl_tree = material_wgsl_parser.parse_material(wgsl_path);
        //
        // println!("WGSL TREE:\n {}", wgsl_tree);

        // // ----------------------- SPIR-V -----------------------

        // let spv_path = std::env::var("SPV_FILE").unwrap();
        // let spv_path = std::path::Path::new(spv_path.as_str());
        //
        // let mut material_spv_parser = MaterialParser::default();
        //
        // let spv_tree = material_spv_parser.parse_material(spv_path);
        // println!("SPV TREE:\n {}", spv_tree);

        // println!();
        // // ----------------------- GLSL Vertex -----------------------

        let vertex_path = std::env::var("VERT_FILE").unwrap();
        let vertex_path = std::path::Path::new(vertex_path.as_str());

        let mut material_vert_parser = MaterialParser::default();

        let vertex_tree = material_vert_parser.parse_material(vertex_path);

        println!("GLSL VERTEX TREE:\n {}", vertex_tree);

        println!();
        // // ----------------------- GLSL Fragment -----------------------
        //
        // let frag_path = std::env::var("FRAG_FILE").unwrap();
        // let frag_path = std::path::Path::new(frag_path.as_str());
        //
        // let mut material_frag_parser = MaterialParser::default();
        //
        // let fragment_tree = material_frag_parser.parse_material(frag_path);
        //
        // println!("GLSL FRAGMENT TREE:\n {}", fragment_tree);
        //
        // // ----------------------- GLSL Compute -----------------------
        //
        // let compute_path = std::env::var("COMP_FILE").unwrap();
        // let compute_path = std::path::Path::new(compute_path.as_str());
        //
        // let mut material_comp_parser = MaterialParser::default();
        //
        // let compute_tree = material_comp_parser.parse_material(compute_path);
        //
        // println!("GLSL COMPUTE TREE:\n {}", compute_tree);
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
