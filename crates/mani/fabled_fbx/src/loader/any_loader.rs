use crate::{triangulate, FbxLoadError, Material, MaterialData};

use fbxcel_dom::v7400::data::material::ShadingModel;
use fbxcel_dom::v7400::object::material::MaterialProperties;
use fbxcel_dom::v7400::object::model::TypedModelHandle;
use fbxcel_dom::v7400::object::TypedObjectHandle;

use crate::FbxLoadError::{FBXLayerMaterialError, FBXLayerNormalError, FBXLayerUVError};

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

                // todo we will not load the video clip from the diffuse or transparent texture,
                // since the path to the  texture will be invalid which will
                // cause a error.
                println!("{:?}", mesh.name());

                let materials = mesh.materials().map(|material_handle| {
                    let material_properties = material_handle.properties();


                    let shading_data = match material_properties.shading_model().unwrap() {
                        None => {
                            panic!("none???")
                        }
                        Some(shading_model) => match shading_model {
                            ShadingModel::Unknown => get_material_property(material_properties),
                            _ => get_material_property(material_properties),
                        },
                    };

                    Material {
                        name: material_handle.name().unwrap().into(),
                        data: shading_data,
                    }
                    // todo get material index
                }).collect::<Vec<_>>();
                for a in &materials {
                    println!("{} {:?}", a.name, a.data)
                }

                let geometry_obj = mesh.geometry().unwrap();

                let polygon_vertices = geometry_obj.polygon_vertices().unwrap();

                let triangle_pvi_indices = polygon_vertices.triangulate_each(triangulate).unwrap();

                let position = triangle_pvi_indices
                    .iter_control_point_indices()
                    .map(|cpi| {
                        cpi.ok_or(FbxLoadError::FbxHeaderError)
                            .and_then(|valid_cpi| {
                                polygon_vertices
                                    .control_point(valid_cpi)
                                    .map(|point| [point.x as f32, point.y as f32, point.z as f32])
                                    .ok_or(FbxLoadError::ControlPointError(valid_cpi))
                            })
                    })
                    .collect::<Result<Vec<_>, _>>();

                for layer in geometry_obj.layers() {

                    let normals = {
                        let normal_handle = layer.layer_element_entries()
                            .filter_map(|layer_entry_handle| match layer_entry_handle.typed_layer_element(){
                                Ok(fbxcel_dom::v7400::data::mesh::layer::TypedLayerElementHandle::Normal(normal_layer_handle)) => Some(normal_layer_handle),
                                _ => None
                            }).next();

                        let normals= if let Some(valid_normal_handle) = normal_handle {
                             //todo switch unwrap with ? to prevent hard panic
                            let normals = valid_normal_handle.normals().map_err(|err|FBXLayerNormalError).unwrap();

                             let normal : Vec<[f32; 3]>= triangle_pvi_indices.triangle_vertex_indices().map(|tri_vert_index|{
                                 let normal = normals.normal(&triangle_pvi_indices, tri_vert_index).unwrap();
                                 [normal.x as f32, normal.y as f32, normal.z as f32]
                             }).collect::<Vec<_>>();

                            normal
                         }else {
                             Vec::new()
                         };

                        normals
                    };

                    let uv = {
                        let uv_handle = layer.layer_element_entries()
                            .filter_map(|layer_entry_handle| match layer_entry_handle.typed_layer_element(){
                                Ok(fbxcel_dom::v7400::data::mesh::layer::TypedLayerElementHandle::Uv(uv_layer_handle)) => Some(uv_layer_handle),
                                _ => None,
                            }).next();

                        let uvs = if let Some(valid_uv_handle) = uv_handle{

                            let uvs = valid_uv_handle.uv().map_err(|_err| FBXLayerUVError).unwrap();

                            let uv : Vec<[f32; 2]> = triangle_pvi_indices.triangle_vertex_indices().map(|tri_vert_index|{
                                let uv = uvs.uv(&triangle_pvi_indices, tri_vert_index).unwrap();
                                [uv.x as f32, uv.y as f32]
                            }).collect::<Vec<_>>();

                            uv
                        }else{
                            Vec::new()
                        };

                        uvs
                    };

                    let indices_per_material = {
                        let mut indices_per_material = vec![Vec::new(); materials.len()];

                        let material_handle = layer.layer_element_entries()
                            .filter_map(|layer_entry_handle| match layer_entry_handle.typed_layer_element(){
                                Ok(fbxcel_dom::v7400::data::mesh::layer::TypedLayerElementHandle::Material(material_layer_handle)) => Some(material_layer_handle),
                                _ => None,
                            }).next();

                        if let Some(valid_material_handle) = material_handle{

                            let materials = valid_material_handle.materials().map_err(|_err|FBXLayerMaterialError).unwrap();

                            for tri_vert_index in triangle_pvi_indices.triangle_vertex_indices(){
                                let local_material_index = materials.material_index(&triangle_pvi_indices, tri_vert_index)
                                    .unwrap().to_u32();

                                indices_per_material.get_mut(local_material_index as usize)
                                    .unwrap().push(tri_vert_index.to_usize() as u32);


                            }
                            indices_per_material
                        }else{
                            Vec::new()
                        }
                    };

                    println!("mesh {:?}, position {}, normals {}, uvs {}, material indices {}",geometry_obj.name(),position.as_ref().unwrap().len(), normals.len(), uv.len(), indices_per_material.len());
                    // get normal if the layer is a normal layer
                    // get uv if the layer is a uv layer
                    // get material if the layer is a material layer
                    // get color if the layer is a color channel layer
                }

                // todo why are we only getting one layer
                for b in geometry_obj.layers() {
                    for (index, a) in b.layer_element_entries().enumerate() {
                        println!("{} {:?}", index, a.type_str());
                    }
                    println!()
                }
            };
        });
    }
}

fn get_material_property(material_properties: MaterialProperties) -> MaterialData {
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

    MaterialData {
        ambient_color: ambient_color.map(|f_64| f_64 as f32),
        diffuse_color: diffuse_color_coe.map(|f_64| f_64 as f32),
        specular_color: specular_color_coe.map(|f_64| f_64 as f32),
        emissive_color: emissive_color_coe.map(|f_64| f_64 as f32),
        shininess: shininess as f32,
        bump_vector,
        normal_map: normal_map.map(|f_64| f_64 as f32),
        transparent_color: transparent_color_coe.map(|f_64| f_64 as f32),
        displacement_color: displacement_color_coe.map(|f_64| f_64 as f32),
        vector_displacement_color: vector_displacement_color_coe.map(|f_64| f_64 as f32),
        reflection: reflection_coe.map(|f_64| f_64 as f32),
    }
}
