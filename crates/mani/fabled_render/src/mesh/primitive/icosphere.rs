use crate::mesh::{Mesh, Model, Vertex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IcoSphere {
    pub radius: f32,
    pub tessellation: u32,
}

impl Default for IcoSphere {
    fn default() -> Self {
        Self::new(1.0, 2)
    }
}

impl IcoSphere {
    pub fn new(radius: f32, mut tessellation: u32) -> IcoSphere {
        tessellation = tessellation.min(3);

        Self {
            radius,
            tessellation,
        }
    }

    fn middle_point(
        p1: usize,
        p2: usize,
        vertices: &mut Vec<f32>,
        cache: &mut rustc_hash::FxHashMap<usize, usize>,
    ) -> usize {
        assert!(vertices.len() > 3 * p1 + 2);
        assert!(vertices.len() > 3 * p2 + 2);

        let cantor_encode = p1 + p2;
        let key = ((cantor_encode * (cantor_encode + 1)) << 1) + std::cmp::min(p1, p2); // Cantor's pairing function

        match cache.remove(&key) {
            Some(value) => {
                // Already Cached
                value
            }
            None => {
                // Cache it
                let len = vertices.len() / 3;
                cache.insert(key, len);

                let x = (vertices[3 * p1] + vertices[3 * p2]) * 0.5; // X
                let y = (vertices[3 * p1 + 1] + vertices[3 * p2 + 1]) * 0.5; // Y
                let z = (vertices[3 * p1 + 2] + vertices[3 * p2 + 2]) * 0.5; // Z

                // Normalize it
                let vec = glam::const_vec3a!([x, y, z]).normalize();

                vertices.extend(vec.to_array());

                len
            }
        }
    }
}

#[rustfmt::skip] 
impl From<IcoSphere> for Model {
    fn from(ico_sphere: IcoSphere) -> Self {
        let vertex_size = 4usize.pow(ico_sphere.tessellation) * 10 * 3 + 2;
        
        //truncated Golden_Ratio (Phi) = 1.618_034
        //Normalization of (1, Phi)
        //(0.52573, 0.85065)
        //let modified_vert = (0.525_73 * ico_sphere.radius, 0.850_65 * ico_sphere.radius);
        let mut normalized_vertices = vec![
            -0.525_73, 0.850_65, 0.0, //0
            0.525_73, 0.850_65, 0.0, //1,
            -0.525_73, -0.850_65, 0.0, //2 
            0.525_73, -0.850_65, 0.0, //3
            
            0.0, -0.525_73, 0.850_65, //4
            0.0, 0.525_73, 0.850_65, //5
            0.0, -0.525_73, -0.850_65, //6
            0.0, 0.525_73, -0.850_65, //7

            0.850_65, 0.0, -0.525_73, //8
            0.850_65, 0.0, 0.525_73, //9
            -0.850_65, 0.0, -0.525_73, //10
            -0.850_65, 0.0, 0.525_73,// 11
        ];
        
        let mut indices: Vec<usize> = vec![
            //5 faces around point 0
            0, 11, 5, //0
            0, 5, 1, //1
            0, 1, 7, //2
            0, 7, 10, //3
            0, 10, 11, //4

            // 5 adjacent faces
            1, 5, 9, //0
            5, 11, 4, //1
            11, 10, 2, //2
            10, 7, 6, //3
            7, 1, 8, //4

            //5 faces around point 3
            3, 9, 4, //0
            3, 4, 2, //1
            3, 2, 6, //2
            3, 6, 8, //3
            3, 8, 9, //4


            // 5 adjacent faces
            4, 9, 5, //0
            2, 4, 11, //1
            6, 2, 10, //2
            8, 6, 7, //3
            9, 8, 1, //4
        ];

        // Very fast hash algorithm we don't care or are worried of Dos Attack so we will use a fast non-cryptographic algorithm
        let mut middle_index_cache : rustc_hash::FxHashMap<usize, usize>  = rustc_hash::FxHashMap::default();

        for index in 0..ico_sphere.tessellation{
            let target_len = 60_usize * 4usize.pow(index + 1);
            let mut face_2: Vec<usize> = vec![0; target_len];


            for  (index, tri) in indices.chunks_exact(3).enumerate(){
                // X, Y, Z
                let v1 : usize = tri[0];
                let v2 : usize = tri[1];
                let v3 : usize = tri[2];
                
                let a = self::IcoSphere::middle_point(v1,v2, &mut normalized_vertices, &mut middle_index_cache);
                let b = self::IcoSphere::middle_point(v2, v3, &mut normalized_vertices, &mut middle_index_cache);
                let c = self::IcoSphere::middle_point(v3, v1, &mut normalized_vertices, &mut middle_index_cache);

                
                let sub_divide_triangle_chunk = [v1, a, c, v2, b, a, v3, c, b, a, b, c];
                
                {
                    //memcpy fast copying
                    let offset = index * 12;
                    let (target_left, _) = face_2[offset..].split_at_mut(12);
                    target_left.copy_from_slice(&sub_divide_triangle_chunk);
                }
                
            }

            indices = face_2;
        }


        //UV map
        let mut uvs = vec![0.0; vertex_size];
        for (index, chunk) in normalized_vertices.chunks_exact(3).enumerate(){

            let inv_pi = 1.0 / std::f32::consts::PI;
            
            // The Last batch of triangles loop back from high uv to zero 
            // A temporary hack is for the Coordinate (U or V) is to use the absolute value of chunk[2] (z).atan2(chunk[0](x)) .
            // chunk[2].abs().atan2(chunk[0]) * (inv_pi * 0.5) + 0.5, // U
            let uv = [
                chunk[2].atan2(chunk[0]) * (inv_pi * 0.5) + 0.5, // U
                chunk[1].asin() * inv_pi + 0.5 // V
            ];

            {
                let offset = index * 2;
                let (target_left, _) = uvs[offset..].split_at_mut(2);
                target_left.copy_from_slice(&uv);
            }
        }
        

        let mut vertices = Vec::with_capacity(vertex_size / 3);

        for (vertex,i) in normalized_vertices.chunks_exact_mut(3).zip(uvs.chunks_exact(2)){
            let pos_x = vertex[0];
            let pos_y = vertex[1];
            let pos_z = vertex[2];

            let u = i[0];
            let v = i[1];

            vertices.push(Vertex{
                position: [pos_x * ico_sphere.radius, pos_y * ico_sphere.radius, pos_z * ico_sphere.radius],
                tex_coord: [u, v],
                normal: [pos_x, pos_y, pos_z],
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4]
            });
        
        }

        let mesh = Mesh{
            vertices,
            material_id: 0,
            indices : indices.into()
        };


        Model{
            meshes: vec![mesh]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::icosphere::IcoSphere;
    use crate::mesh::Model;

    #[test]
    fn test() {
        let ico_sphere = IcoSphere::new(1.0, 2);
        let ico_model: Model = ico_sphere.into();
        for vertex in &ico_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.normal[0], vertex.normal[1], vertex.normal[2]
            );
        }
        println!("{:?}", ico_model.meshes[0].indices);
    }
}
