use crate::mesh::util::NormalInstruction;
use crate::mesh::{Indices, Mesh};


pub fn calculate_bi_tangent(mesh: &mut Mesh) {
    let vertices_data = &mesh.vertices;

    let has_normal = vertices_data
        .iter()
        .any(|&vertex| vertex.normal.ne(&[0.0; 3]));

    let has_tangent = vertices_data
        .iter()
        .any(|&vertex| vertex.tangent.ne(&[0.0; 4]));

    if !has_normal {
        calculate_normals(mesh, NormalInstruction::Flat);
    }

    if !has_tangent {
        calculate_tangents(mesh);
    }

    for vertex in &mut mesh.vertices {
        let n = glam::Vec3A::from_slice(&vertex.normal);
        let t = glam::Vec3A::from_slice(&vertex.tangent);
        let bi_tangent = n.cross(t) * vertex.tangent[3];
        vertex.bi_tangent = [bi_tangent.x, bi_tangent.y, bi_tangent.z, vertex.tangent[3]];
    }
}

pub fn calculate_normals(mesh: &mut Mesh, instruction: NormalInstruction) {
    let indices = &mesh.indices;

    let triangles = match indices {
        Indices::U16(indices_u16) => indices_u16.iter().map(|x| *x as u32).collect::<Vec<_>>(),
        Indices::U32(indices_u32) => indices_u32.to_owned(),
    };

    // 3 indices make a triangle.
    for triangle in triangles.chunks_exact(3) {
        let ia = triangle[0] as usize;
        let ib = triangle[1] as usize;
        let ic = triangle[2] as usize;

        let aa = mesh.vertices[ia].position;
        let ab = mesh.vertices[ib].position;
        let ac = mesh.vertices[ic].position;

        let va = glam::Vec3A::from_slice(&aa);
        let vb = glam::Vec3A::from_slice(&ab);
        let vc = glam::Vec3A::from_slice(&ac);

        let e1 = vb - va;
        let e2 = vc - va;
        let no = e1.cross(e2);

        let mut anormal = glam::Vec3A::ZERO;
        let mut bnormal = glam::Vec3A::ZERO;
        let mut cnormal = glam::Vec3A::ZERO;

        match instruction {
            NormalInstruction::Flat => {
                anormal += no;
                bnormal += no;
                cnormal += no;
            }
            NormalInstruction::Smooth => {
                anormal += va + no;
                bnormal += vb + no;
                cnormal += vc + no;
            }
        }

        mesh.vertices[ia].normal = anormal.normalize().to_array();
        mesh.vertices[ib].normal = bnormal.normalize().to_array();
        mesh.vertices[ic].normal = cnormal.normalize().to_array();
    }
}


pub fn calculate_tangents(mesh: &mut Mesh) {
    let indices = &mesh.indices;

    let triangles = match indices {
        Indices::U16(indices_u16) => indices_u16.iter().map(|x| *x as u32).collect::<Vec<_>>(),
        Indices::U32(indices_u32) => indices_u32.to_owned(),
    };

    let vertices_data = &mut mesh.vertices;

    let uniform_len = vertices_data.len();

    let mut tangents = vec![glam::Vec3A::ZERO; uniform_len * 2];
    let mut bi_tangent = vec![glam::Vec3A::ZERO; uniform_len + uniform_len];

    for tri in triangles.chunks_exact(3) {
        let i1 = tri[0] as usize;
        let i2 = tri[1] as usize;
        let i3 = tri[2] as usize;

        let v1 = glam::Vec3A::from_slice(&vertices_data[i1].position);
        let v2 = glam::Vec3A::from_slice(&vertices_data[i2].position);
        let v3 = glam::Vec3A::from_slice(&vertices_data[i3].position);

        let w1 = glam::Vec2::from_slice(&vertices_data[i1].tex_coord);
        let w2 = glam::Vec2::from_slice(&vertices_data[i2].tex_coord);
        let w3 = glam::Vec2::from_slice(&vertices_data[i3].tex_coord);

        let x1 = v2.x - v1.x;
        let x2 = v3.x - v1.x;
        let y1 = v2.y - v1.y;
        let y2 = v3.y - v1.y;
        let z1 = v2.z - v1.z;
        let z2 = v3.z - v1.z;

        let s1 = w2.x - w1.x;
        let s2 = w3.x - w1.x;
        let t1 = w2.y - w1.y;
        let t2 = w3.y - w1.y;

        let r = 1.0 / (s1 * t2 - s2 * t1);

        let t = glam::vec3a(
            (t2 * x1 - t1 * x2) * r,
            (t2 * y1 - t1 * y2) * r,
            (t2 * z1 - t1 * z2) * r,
        );

        let b = glam::vec3a(
            (s1 * x2 - s2 * x1) * r,
            (s1 * y2 - s2 * y1) * r,
            (s1 * z2 - s2 * z1) * r,
        );

        tangents[i1] += t;
        tangents[i2] += t;
        tangents[i3] += t;

        bi_tangent[i1] += b;
        bi_tangent[i2] += b;
        bi_tangent[i3] += b;
    }
    // Orthonormalize each tangent and calculate its handedness
    for i in 0..uniform_len {
        let n = glam::Vec3A::from_slice(&vertices_data[i].normal);
        let t = tangents[i];
        let b = bi_tangent[i];
        let tangent = (t - n * n.dot(t)).normalize();

        let handedness = (t.cross(b).dot(n) - f32::EPSILON).signum();
        vertices_data[i].tangent = [tangent.x, tangent.y, tangent.z, handedness];
    }
}
