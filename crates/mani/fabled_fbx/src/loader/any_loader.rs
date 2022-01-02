use crate::{FullData, ShaderModel, StandardData};
use fbxcel_dom::v7400::data::material::ShadingModel;
use fbxcel_dom::v7400::object::material::MaterialProperties;
use fbxcel_dom::v7400::object::model::TypedModelHandle;
use fbxcel_dom::v7400::object::TypedObjectHandle;
use std::io::Read;

use std::ops::Mul;

pub fn load<P: AsRef<std::path::Path>>(path: P) {
    let file = std::fs::File::open(path).unwrap();
    let file_bytes_size = file.metadata().unwrap().len() as usize;

    let mut buff_reader = std::io::BufReader::with_capacity(file_bytes_size, file);

    let fbx_header = fbxcel::low::FbxHeader::load(buff_reader.by_ref()).unwrap();

    let mut fbx_any_parser =
        fbxcel::pull_parser::v7400::from_seekable_reader(fbx_header, buff_reader).unwrap();

    let fbx_v7400_tree = fbxcel::tree::v7400::Loader::new()
        .load(&mut fbx_any_parser)
        .unwrap()
        .0;

    if let Ok(document) = fbxcel_dom::v7400::Loader::default().load_from_tree(fbx_v7400_tree) {
        document.objects().for_each(|obj_handle| {
            if let TypedObjectHandle::Model(TypedModelHandle::Mesh(mesh)) = obj_handle.get_typed() {
                // if the mesh is already loaded then return the mesh index
                // to get the mesh. otherwise load.

                let geometry_obj = mesh.geometry().unwrap();

                let materials = mesh.materials().map(|material_handle| {
                    //  let transparent_texture = material_handle.transparent_texture().unwrap();
                    //
                    // let diffuse_texture = material_handle.diffuse_texture();

                    let material_properties = material_handle.properties();

                    let standard_data = get_standard_material_property(material_properties);

                    let shading_data = match material_properties.shading_model().unwrap() {
                        None => {
                            panic!("none???")
                        }
                        Some(a) => match a {
                            ShadingModel::Unknown => {
                                get_full_material_property(standard_data, material_properties)
                            }
                            _ => ShaderModel::Standard(standard_data),
                        },
                    };

                    println!("{:?} {:?}", mesh.name(), standard_data);

                    2
                });
                println!("{:?}", materials.count());
            };
        });
    }
}

fn get_full_material_property(
    standard_model: StandardData,
    material_properties: MaterialProperties,
) -> ShaderModel {
    let bump_factor = material_properties.bump_factor_or_default().unwrap();
    let bump_vector = material_properties
        .bump_or_default()
        .unwrap()
        .map(|vec| (vec * bump_factor) as f32);

    let normal_map = material_properties.normal_map_or_default().unwrap();

    let transparent_color_coefficient = material_properties
        .transparency_factor_or_default()
        .unwrap();

    let transparent_color_coe: [f64; 3] = material_properties
        .transparent_color_or_default()
        .unwrap()
        .mul(transparent_color_coefficient)
        .into();

    let displacement_coefficient = material_properties
        .displacement_factor_or_default()
        .unwrap();

    let displacement_color_coe: [f64; 3] = material_properties
        .vector_displacement_color_or_default()
        .unwrap()
        .mul(displacement_coefficient)
        .into();

    let vector_displacement_color_coefficient = material_properties
        .vector_displacement_factor_or_default()
        .unwrap();

    let vector_displacement_color_coe: [f64; 3] = material_properties
        .vector_displacement_color_or_default()
        .unwrap()
        .mul(vector_displacement_color_coefficient)
        .into();


    let reflection_coefficient = material_properties.reflection_factor_or_default().unwrap();

    let reflection_coe: [f64; 3] = material_properties
        .reflection_or_default()
        .unwrap()
        .mul(reflection_coefficient)
        .into();

    ShaderModel::Full(FullData {
        standard_param: standard_model,
        bump_vector_coe: bump_vector,
        normal_map: normal_map.map(|f_64| f_64 as f32),
        transparent_color_coe: transparent_color_coe.map(|f_64| f_64 as f32),
        displacement_color_coe: displacement_color_coe.map(|f_64| f_64 as f32),
        vector_displacement_color_coe: vector_displacement_color_coe.map(|f_64| f_64 as f32),
        reflection_coe: reflection_coe.map(|f_64| f_64 as f32),
    })
}

fn get_standard_material_property(material_properties: MaterialProperties) -> StandardData {
    let diffuse_coefficient = material_properties.diffuse_factor_or_default().unwrap();

    let diffuse_color_coe: [f64; 3] = material_properties
        .diffuse_color_or_default()
        .unwrap()
        .mul(diffuse_coefficient)
        .into();


    let ambient_coefficient = material_properties.ambient_factor_or_default().unwrap();

    let ambient_color: [f64; 3] = material_properties
        .ambient_color_or_default()
        .unwrap()
        .mul(ambient_coefficient)
        .into();

    let specular_coefficient = material_properties.specular_factor_or_default().unwrap();

    let specular_color_coe: [f64; 3] = material_properties
        .specular_or_default()
        .unwrap()
        .mul(specular_coefficient)
        .into();

    let emissive_factor = material_properties.emissive_factor_or_default().unwrap();

    let emissive_color_coe: [f64; 3] = material_properties
        .emissive_color_or_default()
        .unwrap()
        .mul(emissive_factor)
        .into();

    let shininess = material_properties.shininess_or_default().unwrap();


    StandardData {
        ambient_color_coe: ambient_color.map(|f_64| f_64 as f32),
        diffuse_color_coe: diffuse_color_coe.map(|f_64| f_64 as f32),
        specular_color_coe: specular_color_coe.map(|f_64| f_64 as f32),
        emissive_color_factor: emissive_color_coe.map(|f_64| f_64 as f32),
        shininess: shininess as f32,
    }
}
