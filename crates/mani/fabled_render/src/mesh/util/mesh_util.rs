use crate::mesh::util::NormalInstruction;
use crate::mesh::Mesh;

pub fn calculate_bi_tangent() {}

pub fn calculate_normals(mesh: &mut Mesh, instruction: NormalInstruction) {
    let indices = &mesh.indices;

    // 3 indices make a triangle.
    for triangle in indices.chunks_exact(3) {
        let ia = triangle[0];
        let ib = triangle[1];
        let ic = triangle[2];

        let aa = &mesh.vertices[ia].position;
        let ab = &mesh.vertices[ib].position;
        let ac = &mesh.vertices[ic].position;

        let va = glam::Vec3A::from_slice(aa);
        let vb = glam::Vec3A::from_slice(ab);
        let vc = glam::Vec3A::from_slice(ac);

        let e1 = vb - va;
        let e2 = vc - va;
        let no = e1.cross(e2);

        let mut anormal = glam::Vec3A::ZERO;
        let mut bnormal = glam::Vec3A::ZERO;
        let mut cnormal = glam::Vec3A::ZERO;

        match instruction {
            NormalInstruction::Flat => {
                anormal = no;
                bnormal = no;
                cnormal = no;
            }
            NormalInstruction::Smooth => {
                anormal = va + no;
                bnormal = vb + no;
                cnormal = vc + no;
            }
        }

        mesh.vertices[ia].normal = anormal.normalize().to_array();
        mesh.vertices[ib].normal = bnormal.normalize().to_array();
        mesh.vertices[ic].normal = cnormal.normalize().to_array();
    }
}

//todo got to look over this and test it before using it in code source.
pub fn calculate_tangents(mesh: &mut Mesh) {
    let triangles = &mesh.indices;

    let vertices_data = &mut mesh.vertices;

    let uniform_len = vertices_data.len();

    let mut tangents = vec![glam::Vec4::ZERO; uniform_len];
    let mut bi_tangent = vec![glam::Vec4::ZERO; uniform_len];

    for tri in triangles.chunks_exact(3) {
        let i1 = tri[0];
        let i2 = tri[1];
        let i3 = tri[2];

        let p0 = glam::Vec3A::from_slice(&vertices_data[i1].position);
        let p1 = glam::Vec3A::from_slice(&vertices_data[i2].position);
        let p2 = glam::Vec3A::from_slice(&vertices_data[i3].position);

        let w0 = glam::Vec2::from_slice(&vertices_data[i1].tex_coord);
        let w1 = glam::Vec2::from_slice(&vertices_data[i2].tex_coord);
        let w2 = glam::Vec2::from_slice(&vertices_data[i3].tex_coord);

        let e1 = p1 - p0;
        let e2 = p2 - p0;

        let xy1 = w1 - w0;
        let xy2 = w2 - w0;

        let r = 1.0 / (xy1.x * xy2.y - xy2.x * xy1.y);
        let t = (e1 * xy2.y - e2 * xy1.y) * r;
        let b = (e2 * xy1.x - e1 * xy2.x) * r;

        tangents[i1] = t.extend(0.0);
        tangents[i2] = t.extend(0.0);
        tangents[i3] = t.extend(0.0);

        bi_tangent[i1] = b.extend(0.0);
        bi_tangent[i2] = b.extend(0.0);
        bi_tangent[i3] = b.extend(0.0);

        // Orthonormalize each tangent and calculate its handedness
        for i in 0..uniform_len {
            let t = tangents[i].truncate();
            let b = bi_tangent[i].truncate();
            let n = glam::Vec3::from_slice(&vertices_data[i].normal);

            let tangent = reject(n, t).normalize();
            let handedness = ((t.cross(b).dot(n)) - f32::EPSILON).signum();

            vertices_data[i].tangent = [tangent.x, tangent.y, tangent.z, handedness];

            //to calculate the bi_tangent
            let bi_tangent = n.cross(tangent).extend(1.0) * handedness;
            vertices_data[i].bi_tangent = bi_tangent.to_array();
        }
    }
}

//todo move to fabled_math.
pub fn reject(axis: glam::Vec3, direction: glam::Vec3) -> glam::Vec3 {
    let axis = axis.normalize();
    direction - axis * direction.dot(axis)
}
