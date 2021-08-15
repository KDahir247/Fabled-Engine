use crate::mesh::util::min_ss;
use crate::mesh::{Mesh, Model, Vertex};

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
    pub fn new(radius: f32, tessellation: u32) -> IcoSphere {
        //clamp tessellation to a value.

        Self {
            radius,
            tessellation,
        }
    }

    //todo optimize this function.
    fn middle_point(
        p1: usize,
        p2: usize,
        radius: f32,
        vertices: &mut Vec<f32>,
        cache: &mut rustc_hash::FxHashMap<usize, usize>,
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

                //Normalize it and scale by radius
                let vec = glam::const_vec3a!([x, y, z]).normalize() * radius;

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
        //let vertex_size = 4i32.pow(ico_sphere.tessellation) * 10 + 2;
        
        //truncated Golden_Ratio (Phi) = 1.618_034
        //Normalization of (1, Phi)
        //(0.52573, 0.85065)
        let modified_vert = (0.525_73 * ico_sphere.radius, 0.850_65 * ico_sphere.radius);

        let mut normalized_vertices = vec![
            -modified_vert.0, modified_vert.1, 0.0, //0
            modified_vert.0, modified_vert.1, 0.0, //1,
            -modified_vert.0, -modified_vert.1, 0.0, //2 
            modified_vert.0, -modified_vert.1, 0.0, //3
            
            0.0, -modified_vert.0, modified_vert.1, //4
            0.0, modified_vert.0, modified_vert.1, //5
            0.0, -modified_vert.0, -modified_vert.1, //6
            0.0, modified_vert.0, -modified_vert.1, //7

            modified_vert.1, 0.0, -modified_vert.0, //8
            modified_vert.1, 0.0, modified_vert.0, //9
            -modified_vert.1, 0.0, -modified_vert.0, //10
            -modified_vert.1, 0.0, modified_vert.0,// 11
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

        // Very fast hash algorithm we don't care or are worried of Dos Attack so we will use a fast non-cryptographic algorithm
        // todo maybe reduce it to u32, u32
        let mut middle_index_cache : rustc_hash::FxHashMap<usize, usize>  = rustc_hash::FxHashMap::default();
        
        for index in 0..ico_sphere.tessellation{
            let target_len = 60_usize * 4usize.pow(index + 1);
            let mut face_2: Vec<usize> = vec![0; target_len];


            for  (index, tri) in triangles.chunks_exact(3).enumerate(){
                // X, Y, Z
                let v1 : usize = tri[0];
                let v2 : usize = tri[1];
                let v3 : usize = tri[2];
                
                //let faces = [v1, v1, v2, v2, v3, v3, v1];

                //todo maybe just put the v1 v2 v2 v3  v3 v1  in a collection [v1, v2, v2, v3, v3, v1] and chunk iter and do calculate here to prevent any miss on auto vectorization and unfold this method.
                let a = self::IcoSphere::middle_point(v1,v2, ico_sphere.radius,&mut normalized_vertices, &mut middle_index_cache);
                let b = self::IcoSphere::middle_point(v2, v3, ico_sphere.radius,&mut normalized_vertices, &mut middle_index_cache);
                let c = self::IcoSphere::middle_point(v3, v1, ico_sphere.radius,&mut normalized_vertices, &mut middle_index_cache);

                let sub_divide_triangle_chunk = [v1, a, c, v2, b, a, v3, c, b, a, b, c];
                
                {
                    //memcpy fast copying
                    let offset = index * 12;
                    let (target_left, _) = face_2[offset..].split_at_mut(12);
                    target_left.copy_from_slice(&sub_divide_triangle_chunk);
                }
            }

            triangles = face_2;
        }

        let mut vertex_data = Vec::new();
        //Correct
        for vertex in normalized_vertices.chunks_exact_mut(3){
            
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
        let ico_sphere = IcoSphere::new(1.0, 3);
        let ico_model: Model = ico_sphere.into();
        for a in &ico_model.meshes[0].vertices {
            println!("{:?}", a.position);
        }
    }
}
