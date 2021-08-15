use crate::mesh::util::min_ss;
use crate::mesh::{Mesh, Model, Vertex};

pub struct IcoSphere {
    pub radius: f32,
    pub tessellation: u32,
}

impl Default for IcoSphere {
    fn default() -> Self {
        Self::new(1.0, 3)
    }
}

impl IcoSphere {
    pub fn new(radius: f32, tessellation: u32) -> IcoSphere {
        //clamp tessellation to a value.

        Self {
            radius,
            tessellation,
        }
    }

    //todo maybe radius * vec
    fn middle_point(
        p1: usize,
        p2: usize,
        vertices: &mut Vec<f32>,
        cache: &mut std::collections::HashMap<usize, usize>,
    ) -> usize {
        //((a + b) * (a + b + 1) / 2) + Math.min(a, b)
        let key = (((p1 + p2) * (p1 + p2 + 1)) << 1) + min_ss(p1 as f32, p2 as f32) as usize; //todo create a min for usize type

        match cache.remove(&key) {
            Some(value) => {
                //Already Cached
                value
            }
            None => {
                //Cache it
                let len = vertices.len() / 3;
                cache.insert(key, len);

                let x = (vertices[3 * p1] + vertices[3 * p2]) * 0.5; // X
                let y = (vertices[3 * p1 + 1] + vertices[3 * p2 + 1]) * 0.5; // Y
                let z = (vertices[3 * p1 + 2] + vertices[3 * p2 + 2]) * 0.5; // Z

                //Normalize it
                let vec = glam::const_vec3a!([x, y, z]).normalize();

                vertices.push(vec.x); // X
                vertices.push(vec.y); // Y
                vertices.push(vec.z); // Z

                len
            }
        }
    }
}

#[rustfmt::skip] //todo temporary to see indention on data for implementation
impl From<IcoSphere> for Model {
    fn from(ico_sphere: IcoSphere) -> Self {
        let vertex_size = 4i32.pow(ico_sphere.tessellation) * 10 + 2;

        //println!("{}", vertex_size);
        //Normalization of (1, Phi)
        //truncated Golden_Ratio (Phi) = 1.618_034
        //(0.52573, 0.85065)
        // vertex_size
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
        
        let mut triangles: Vec<usize> = vec![
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

        //100 pre allocated space for cache
        let mut middle_index_cache: std::collections::HashMap<usize,usize> = std::collections::HashMap::with_capacity(100);
        
        for index in 0..ico_sphere.tessellation{
            let target_len = 60_usize * 4usize.pow(index + 1);
            let mut face_2: Vec<usize> = vec![0; target_len];


            for  (index, tri) in triangles.chunks_exact(3).enumerate(){
                // X, Y, Z
                let v1 : usize = tri[0];
                let v2 : usize = tri[1];
                let v3 : usize = tri[2];

                let a = self::IcoSphere::middle_point(v1,v2,&mut normalized_vertices, &mut middle_index_cache);
                let b = self::IcoSphere::middle_point(v2, v3, &mut normalized_vertices, &mut middle_index_cache);
                let c = self::IcoSphere::middle_point(v3, v1, &mut normalized_vertices, &mut middle_index_cache);

                let sub_triangles = [v1, a, c, v2, b, a, v3, c, b, a, b, c];
                
                {
                    //memcpy fast copying
                    let target = index * 12;
                    let (left, _) = face_2[target..].split_at_mut(12);
                    left.copy_from_slice(&sub_triangles);
                }
            }

            triangles = face_2;
        }

        let mut vertex_data = Vec::new();
        //Correct
        for vertex in normalized_vertices.chunks_exact_mut(3){
            vertex[0] *= ico_sphere.radius;
            vertex[1] *=  ico_sphere.radius;
            vertex[2] *= ico_sphere.radius;
        
            
            vertex_data.push(Vertex{
                position: [vertex[0], vertex[1], vertex[2]],
                tex_coord: [0.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                tangent: [0.0, 0.0, 0.0, 0.0],
                bi_tangent: [0.0, 0.0, 0.0, 0.0]
            });
        
        }
        
        let mesh = Mesh{
            vertices: vertex_data,
            material_id: 0,
            indices: triangles
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
        let ico_sphere = IcoSphere::new(5.0, 3);
        let ico_model: Model = ico_sphere.into();
        println!("{:?}", ico_model.meshes[0].indices);
    }
}
