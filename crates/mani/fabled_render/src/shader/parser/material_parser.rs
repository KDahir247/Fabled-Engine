use crate::material::*;
use crate::shader;

pub struct MaterialParser {
    material_head: MaterialTree,
}

impl Default for MaterialParser {
    fn default() -> Self {
        Self::new("Material")
    }
}

impl MaterialParser {
    pub fn new(head: &str) -> Self {
        let parser_leaf: [MaterialLeaf; 9] = [
            MaterialLeaf {
                type_name: "Scalar".to_string(),
                types: vec![],
            },
            MaterialLeaf {
                type_name: "Vector".to_string(),
                types: vec![],
            },
            MaterialLeaf {
                type_name: "Matrix".to_string(),
                types: vec![],
            },
            MaterialLeaf {
                type_name: "Pointer".to_string(),
                types: vec![],
            },
            MaterialLeaf {
                type_name: "ValuePointer".to_string(),
                types: vec![],
            },
            MaterialLeaf {
                type_name: "Array".to_string(),
                types: vec![],
            },
            MaterialLeaf {
                type_name: "Struct".to_string(),
                types: vec![],
            },
            MaterialLeaf {
                type_name: "Image".to_string(),
                types: vec![],
            },
            MaterialLeaf {
                type_name: "Sampler".to_string(),
                types: vec![],
            },
        ];

        Self {
            material_head: MaterialTree {
                material_name: head.to_string(),
                shader: "Material".to_string(),
                attributes: parser_leaf,
            },
        }
    }

    pub fn create_material_hierarchy<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> anyhow::Result<String> {
        let shader = shader::parser::ShaderParser::parse(path)?;

        let naga::Module {
            types,
            global_variables,
            ..
        } = shader.module;

        let globals = global_variables.into_inner();

        let mut types = types.into_inner();

        for (offset, global) in globals
            .iter()
            .filter(|has_name| has_name.name.is_some())
            .enumerate()
        {
            let type_var: naga::Type;

            //todo don't like this nested condition. use another method. bitwise operators?.
            if global.ty.index() == 0 {
                type_var = types.remove(0);
            } else if global.ty.index() >= offset {
                type_var = types.remove(global.ty.index() - offset);
            } else {
                type_var = types.remove(global.ty.index());
            }

            let node = Self::create_node(global.to_owned(), type_var.inner);

            //todo find a cleaner solution then this.
            let index = MaterialTarget::index(&node.value_type);

            self.material_head.attributes[index].types.push(node);
        }

        let pretty = ron::ser::PrettyConfig::new()
            .with_separate_tuple_members(true)
            .with_decimal_floats(true)
            .with_depth_limit(5)
            .with_enumerate_arrays(true);

        let ron_material = ron::ser::to_string_pretty(&self.material_head, pretty).expect("failed to serialize shader data into a material representation.\n failed on writing out the data to ron format.");

        Ok(ron_material)
    }

    fn create_node(global_var: naga::GlobalVariable, inner: naga::TypeInner) -> MaterialNode {
        let node_type = inner;

        let resource = global_var.binding;

        let (group, binding) = match resource {
            None => (None, None),
            Some(resource) => (Some(resource.group), Some(resource.binding)),
        };

        MaterialNode {
            value_name: global_var.name.unwrap(),
            value: MaterialTarget::from(&node_type),
            value_type: node_type,
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

        let mut material_wgsl_parser = MaterialParser::new("Wgsl Material");
        let wgsl_tree = material_wgsl_parser
            .create_material_hierarchy(wgsl_path)
            .unwrap();

        println!("WGSL TREE:\n{}\n\n", wgsl_tree);

        let spv_path = std::env::var("SPV_FILE").unwrap();
        let spv_path = std::path::Path::new(spv_path.as_str());

        let mut material_spv_parser = MaterialParser::new("SPV Material");

        let spv_tree = material_spv_parser
            .create_material_hierarchy(spv_path)
            .unwrap();
        println!("SPV TREE:\n{}\n\n", spv_tree);

        let vertex_path = std::env::var("VERT_FILE").unwrap();
        let vertex_path = std::path::Path::new(vertex_path.as_str());

        let mut material_vert_parser = MaterialParser::new("GLSL Vertex Material");
        let vertex_tree = material_vert_parser
            .create_material_hierarchy(vertex_path)
            .unwrap();

        println!("GLSL VERTEX TREE:\n{}\n\n", vertex_tree);

        let frag_path = std::env::var("FRAG_FILE").unwrap();
        let frag_path = std::path::Path::new(frag_path.as_str());

        let mut material_frag_parser = MaterialParser::new("GLSL Fragment Material");

        let fragment_tree = material_frag_parser
            .create_material_hierarchy(frag_path)
            .unwrap();

        println!("GLSL FRAGMENT TREE:\n{}\n\n", fragment_tree);

        let compute_path = std::env::var("COMP_FILE").unwrap();
        let compute_path = std::path::Path::new(compute_path.as_str());

        let mut material_comp_parser = MaterialParser::new("GLSL Compute Material");

        let compute_tree = material_comp_parser
            .create_material_hierarchy(compute_path)
            .unwrap();

        println!("GLSL COMPUTE TREE:\n{}\n\n", compute_tree);
    }
}
