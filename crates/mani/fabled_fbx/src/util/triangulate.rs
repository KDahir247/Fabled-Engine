use anyhow::{anyhow, bail};
use fbxcel_dom::v7400::data::mesh::{PolygonVertexIndex, PolygonVertices};

// todo got to do this next.

pub fn triangulate(
    poly_vertices: &fbxcel_dom::v7400::data::mesh::PolygonVertices,
    poly_vertex_indices: &[fbxcel_dom::v7400::data::mesh::PolygonVertexIndex],
    result: &mut Vec<[fbxcel_dom::v7400::data::mesh::PolygonVertexIndex; 3]>,
) -> anyhow::Result<()> {
    match poly_vertex_indices.len() {
        1 | 2 => {
            // well we are in a pinch
            bail!("Not enough vertices in the polygon",);
        }
        3 => {
            result.push([
                poly_vertex_indices[0],
                poly_vertex_indices[1],
                poly_vertex_indices[2],
            ]);
            Ok(())
        }
        4 => {
            let p0 = get_vec(poly_vertices, poly_vertex_indices[0])?;
            let p1 = get_vec(poly_vertices, poly_vertex_indices[1])?;
            let p2 = get_vec(poly_vertices, poly_vertex_indices[2])?;
            let p3 = get_vec(poly_vertices, poly_vertex_indices[3])?;

            let p0_min_p1 = [p0[0] - p1[0], p0[1] - p1[1], p0[2] - p1[2]];
            let p1_min_p2 = [p1[0] - p2[0], p1[1] - p2[1], p1[2] - p2[2]];
            let p2_min_p3 = [p2[0] - p3[0], p2[1] - p3[1], p2[2] - p3[2]];
            let p3_min_p0 = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];

            let n1 = [
                p0_min_p1[1] * p1_min_p2[2] - p0_min_p1[2] * p1_min_p2[1],
                p0_min_p1[2] * p1_min_p2[0] - p0_min_p1[0] * p1_min_p2[2],
                p0_min_p1[0] * p1_min_p2[1] - p0_min_p1[1] * p1_min_p2[0],
            ];

            let n3 = [
                p2_min_p3[1] * p3_min_p0[2] - p2_min_p3[2] * p3_min_p0[1],
                p2_min_p3[2] * p3_min_p0[0] - p2_min_p3[0] * p3_min_p0[2],
                p2_min_p3[0] * p3_min_p0[1] - p2_min_p3[1] * p3_min_p0[0],
            ];

            let n1_dot_n3 = n1[0] * n3[0] + n1[1] * n3[1] + n1[2] * n3[2];

            if n1_dot_n3 >= 0.0 {
                result.extend(&[
                    [
                        poly_vertex_indices[0],
                        poly_vertex_indices[1],
                        poly_vertex_indices[2],
                    ],
                    [
                        poly_vertex_indices[2],
                        poly_vertex_indices[3],
                        poly_vertex_indices[0],
                    ],
                ]);
            } else {
                result.extend_from_slice(&[
                    [
                        poly_vertex_indices[0],
                        poly_vertex_indices[1],
                        poly_vertex_indices[2],
                    ],
                    [
                        poly_vertex_indices[2],
                        poly_vertex_indices[3],
                        poly_vertex_indices[0],
                    ],
                ])
            }

            Ok(())
        }
        n => {
            bail!("no supported yet.")
        }
    }
}

fn get_vec(
    poly_vert: &PolygonVertices,
    poly_vert_indices: PolygonVertexIndex,
) -> anyhow::Result<[f64; 3]> {
    poly_vert
        .control_point(poly_vert_indices)
        .map(Into::into)
        .ok_or_else(|| anyhow!(""))
}
