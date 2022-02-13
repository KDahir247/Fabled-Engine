// This type will not be returning the scene but will use the asset manager to
// make it into a meta file so it can be convert to the struct type.

use crate::loader::{v7400_camera_loader as camera_impl, v7400_light_loader as light_impl};
use crate::{triangulate, FbxLoadError, GeometryMesh, Material, MaterialData};
use std::io::Read;
use std::ops::Mul;

pub struct V7400Loader;

impl V7400Loader {
    pub fn loads<P: AsRef<std::path::Path>>(path: P) -> Result<(), FbxLoadError> {
        let file = std::fs::File::open(path)?;
        let file_bytes_size = file.metadata()?.len() as usize;

        let mut buff_reader = std::io::BufReader::with_capacity(file_bytes_size, file);

        let fbx_header = fbxcel::low::FbxHeader::load(buff_reader.by_ref())
            .map_err(|_err| FbxLoadError::FbxHeaderError)?;

        let mut fbx_v7400_parser =
            fbxcel::pull_parser::v7400::from_seekable_reader(fbx_header, buff_reader)
                .map_err(|_| FbxLoadError::FBXInvalidVersionError)?;

        fbx_v7400_parser.set_warning_handler(|fbx_warning, syn_pos| {
            eprintln!("Warning : {} = {:?}", fbx_warning, syn_pos);

            Ok(())
        });

        // bottle here.
        let fbx_v7400_tree = fbxcel::tree::v7400::Loader::new()
            .load(&mut fbx_v7400_parser)
            .map_err(|_| FbxLoadError::FBXParseError)?
            .0;

        let fbx_v7400_document = fbxcel_dom::v7400::Loader::default()
            .load_from_tree(fbx_v7400_tree)
            .map_err(|_| FbxLoadError::FBXParseError)?;

        fbx_v7400_document.objects().for_each(|obj_handle| {
            if let fbxcel_dom::v7400::object::TypedObjectHandle::Model(
                fbxcel_dom::v7400::object::model::TypedModelHandle::Mesh(mesh),
            ) = obj_handle.get_typed()
            {
                let materials = collect_materials(mesh);
                let geometry_mesh = collect_geometry(mesh, materials.len()).unwrap();
            }

            #[cfg(feature = "light")]
            light_impl::load_light_handle(&obj_handle);

            #[cfg(feature = "camera")]
            camera_impl::load_camera_handle(&obj_handle);
        });

        Ok(())
    }
}

fn collect_geometry(
    mesh_handle: fbxcel_dom::v7400::object::model::MeshHandle,
    material_len: usize,
) -> anyhow::Result<GeometryMesh> {
    let geometry_mesh_handle = mesh_handle.geometry()?;

    let polygon_vertices = geometry_mesh_handle.polygon_vertices()?;

    let triangle_pvi_indices = polygon_vertices.triangulate_each(triangulate)?;

    let positions = triangle_pvi_indices
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
        .flat_map(|position_vector| position_vector.unwrap_or_default())
        .collect::<Vec<_>>();

    let fbx_layers = geometry_mesh_handle.layers();

    let mut normals = Vec::new();

    let mut uvs = Vec::new();

    let mut indices_per_material = Vec::with_capacity(material_len);

    for layer in fbx_layers {
        normals = {
            let normal_handle = layer
                .layer_element_entries()
                .filter_map(
                    |layer_entry_handle| match layer_entry_handle.typed_layer_element() {
                        Ok(
                            fbxcel_dom::v7400::data::mesh::layer::TypedLayerElementHandle::Normal(
                                normal_layer_handle,
                            ),
                        ) => Some(normal_layer_handle),
                        _ => None,
                    },
                )
                .next();

            let normal_collection = if let Some(valid_normal_handle) = normal_handle {
                let normals_detail = valid_normal_handle
                    .normals()
                    .map_err(|_| FbxLoadError::FBXLayerNormalError)?;

                let normals: Vec<f32> = triangle_pvi_indices
                    .triangle_vertex_indices()
                    .map(|tri_vert_index| {
                        let normal_vector = normals_detail
                            .normal(&triangle_pvi_indices, tri_vert_index)
                            .unwrap_or_else(|_| [0.0; 3].into());

                        [
                            normal_vector.x as f32,
                            normal_vector.y as f32,
                            normal_vector.z as f32,
                        ]
                    })
                    .flatten()
                    .collect::<Vec<_>>();

                normals
            } else {
                vec![]
            };

            normal_collection
        };

        uvs = {
            let uv_handle = layer
                .layer_element_entries()
                .filter_map(
                    |layer_entry_handle| match layer_entry_handle.typed_layer_element() {
                        Ok(fbxcel_dom::v7400::data::mesh::layer::TypedLayerElementHandle::Uv(
                            uv_layer_handle,
                        )) => Some(uv_layer_handle),
                        _ => None,
                    },
                )
                .next();

            let uv_collection = if let Some(valid_uv_handle) = uv_handle {
                let uv_details = valid_uv_handle
                    .uv()
                    .map_err(|_| FbxLoadError::FBXLayerUVError)?;

                let uvs: Vec<f32> = triangle_pvi_indices
                    .triangle_vertex_indices()
                    .map(|tri_vert_index| {
                        let uv_vector = uv_details
                            .uv(&triangle_pvi_indices, tri_vert_index)
                            .unwrap_or_else(|_| [0.0; 2].into());

                        [uv_vector.x as f32, uv_vector.y as f32]
                    })
                    .flatten()
                    .collect::<Vec<_>>();

                uvs
            } else {
                vec![]
            };

            uv_collection
        };

        indices_per_material = {
            let mut indices_per_material: Vec<Vec<u32>> = vec![Vec::new(); material_len];

            let material_handle = layer
                .layer_element_entries()
                .filter_map(
                    |layer_entry_handle| match layer_entry_handle.typed_layer_element() {
                        Ok(
                            fbxcel_dom::v7400::data::mesh::layer::TypedLayerElementHandle::Material(
                                material_layer_handle,
                            ),
                        ) => Some(material_layer_handle),
                        _ => None,
                    },
                )
                .next();

            if let Some(valid_material_handle) = material_handle {
                let material_detail = valid_material_handle
                    .materials()
                    .map_err(|_| FbxLoadError::FBXLayerMaterialError)?;

                for tri_vert_index in triangle_pvi_indices.triangle_vertex_indices() {
                    let local_material_index = material_detail
                        .material_index(&triangle_pvi_indices, tri_vert_index)
                        .ok()
                        .map(|material_index| material_index.to_u32());


                    if let Some(material_index) = local_material_index {
                        indices_per_material
                            .get_mut(material_index as usize)
                            .unwrap()
                            .push(tri_vert_index.to_usize() as u32);
                    }
                }
            }
            indices_per_material
        };
    }

    Ok(GeometryMesh {
        name: geometry_mesh_handle.name().unwrap_or_default().into(),
        positions,
        normals,
        uvs,
        indices_per_material,
    })
}

fn collect_materials(mesh_handle: fbxcel_dom::v7400::object::model::MeshHandle) -> Vec<Material> {
    let materials = mesh_handle
        .materials()
        .map(|material_handle| {
            let material_property = material_handle.properties();

            let material_detail = match material_property.shading_model().unwrap_or(None) {
                None => MaterialData::default(),
                Some(_) => {
                    let diffuse_coefficient =
                        material_property.diffuse_factor_or_default().unwrap_or(0.0);

                    let diffuse_color: [f64; 3] = material_property
                        .diffuse_color_or_default()
                        .unwrap_or_default()
                        .mul(diffuse_coefficient)
                        .into();

                    let ambient_coefficient =
                        material_property.ambient_factor_or_default().unwrap_or(0.0);

                    let ambient_color: [f64; 3] = material_property
                        .ambient_color_or_default()
                        .unwrap_or_default()
                        .mul(ambient_coefficient)
                        .into();

                    let specular_coefficient = material_property
                        .specular_factor_or_default()
                        .unwrap_or(0.0);

                    let specular_color: [f64; 3] = material_property
                        .specular_or_default()
                        .unwrap_or_default()
                        .mul(specular_coefficient)
                        .into();

                    let emissive_factor = material_property
                        .emissive_factor_or_default()
                        .unwrap_or(0.0);

                    let emissive_color: [f64; 3] = material_property
                        .emissive_color_or_default()
                        .unwrap_or_default()
                        .mul(emissive_factor)
                        .into();

                    let shininess = material_property.shininess_or_default().unwrap_or(0.0);

                    let bump_factor = material_property.bump_factor_or_default().unwrap_or(0.0);

                    let bump_vector = material_property
                        .bump_or_default()
                        .unwrap_or_default()
                        .map(|vec| (vec * bump_factor) as f32);

                    let normal_map = material_property
                        .normal_map_or_default()
                        .unwrap_or_default();

                    let transparent_color_coefficient = material_property
                        .transparency_factor_or_default()
                        .unwrap_or(0.0);

                    let transparent_color: [f64; 3] = material_property
                        .transparent_color_or_default()
                        .unwrap_or_default()
                        .mul(transparent_color_coefficient)
                        .into();

                    let displacement_coefficient = material_property
                        .displacement_factor_or_default()
                        .unwrap_or(0.0);

                    let displacement_color: [f64; 3] = material_property
                        .displacement_color_or_default()
                        .unwrap_or_default()
                        .mul(displacement_coefficient)
                        .into();

                    let vector_displacement_factor = material_property
                        .vector_displacement_factor_or_default()
                        .unwrap_or(0.0);

                    let vector_displacement_color: [f64; 3] = material_property
                        .vector_displacement_color_or_default()
                        .unwrap_or_default()
                        .mul(vector_displacement_factor)
                        .into();

                    let reflection_factor = material_property
                        .reflection_factor_or_default()
                        .unwrap_or(0.0);

                    let reflection: [f64; 3] = material_property
                        .reflection_or_default()
                        .unwrap_or_default()
                        .mul(reflection_factor)
                        .into();

                    let material_data = MaterialData {
                        ambient_color: [
                            ambient_color[0] as f32,
                            ambient_color[1] as f32,
                            ambient_color[2] as f32,
                        ],
                        diffuse_color: [
                            diffuse_color[0] as f32,
                            diffuse_color[1] as f32,
                            diffuse_color[2] as f32,
                        ],
                        specular_color: [
                            specular_color[0] as f32,
                            specular_color[1] as f32,
                            specular_color[2] as f32,
                        ],
                        emissive_color: [
                            emissive_color[0] as f32,
                            emissive_color[1] as f32,
                            emissive_color[2] as f32,
                        ],
                        shininess: shininess as f32,
                        bump_vector,
                        normal_map: [
                            normal_map[0] as f32,
                            normal_map[1] as f32,
                            normal_map[2] as f32,
                        ],
                        transparent_color: [
                            transparent_color[0] as f32,
                            transparent_color[1] as f32,
                            transparent_color[2] as f32,
                        ],
                        displacement_color: [
                            displacement_color[0] as f32,
                            displacement_color[1] as f32,
                            displacement_color[2] as f32,
                        ],
                        vector_displacement_color: [
                            vector_displacement_color[0] as f32,
                            vector_displacement_color[1] as f32,
                            vector_displacement_color[2] as f32,
                        ],
                        reflection: [
                            reflection[0] as f32,
                            reflection[1] as f32,
                            reflection[2] as f32,
                        ],
                    };

                    material_data
                }
            };

            Material {
                name: material_handle.name().unwrap_or_default().into(),
                data: material_detail,
            }
        })
        .collect::<Vec<_>>();

    materials
}

#[cfg(test)]
mod v7400_loader_test {
    use crate::V7400Loader;
    use std::slice::from_raw_parts;

    #[test]
    fn load() {
        V7400Loader::loads("C:/Users/kdahi/Downloads/untitled.fbx");
    }
}
