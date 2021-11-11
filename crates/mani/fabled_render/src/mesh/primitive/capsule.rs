use crate::mesh::primitive::capsule::CapsuleUvProfile::Aspect;
use crate::mesh::{Mesh, Model, Vertex};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CapsuleUvProfile {
    Aspect = 0,
    Uniform = 1,
    Fixed = 2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(align(16))]
pub struct Capsule {
    pub radius: f32,
    pub depth: f32,
    pub rings: u8,
    pub latitude: u8,
    pub longitude: u8,
    pub v_profile: CapsuleUvProfile,
}

impl Default for Capsule {
    fn default() -> Self {
        Self::new(0.5, 1, 1., 8, 16, Aspect)
    }
}

impl Capsule {
    pub fn new(
        radius: f32,
        mut rings: u8,
        depth: f32,
        mut latitude: u8,
        mut longitude: u8,
        profile: CapsuleUvProfile,
    ) -> Capsule {
        // Sanity check for the size of latitude, longitude, and ring to prevent
        // Latitude from reaching beyond 90, longitude from reaching above 180,
        // and ring from reaching beyond 5.
        //
        // Prevent increasing latitude and longitude intentional or unintentional
        // to the point where it cause  s extreme repeating KiPageFault and
        // MmAccessFault resulting in cache thrashing or memory can't be
        // allocated error.

        latitude = latitude.min(90);
        longitude = longitude.min(180);
        rings = rings.min(5);

        Capsule {
            radius,
            rings,
            depth,
            latitude,
            longitude,
            v_profile: profile,
        }
    }
}

impl From<Capsule> for Model {
    fn from(capsule: Capsule) -> Self {
        // code adapted with modifications from https://behreajj.medium.com/making-a-capsule-mesh-via-script-in-five-3d-environments-c2214abf02db
        // provided by bevy. Thank you Bevy Community! \(ᵔᵕᵔ)/

        let Capsule {
            radius,
            rings,
            depth,
            latitude,
            longitude,
            v_profile,
        } = capsule;

        let rings = rings as usize;
        let longitude = longitude as usize;
        let latitude = latitude as usize;

        let calc_middle = rings.min(1);
        let half_lats = latitude >> 1; // equal to latitude / 2;
        let half_lats_sub_1 = half_lats - 1;
        let half_lats_sub_2 = half_lats_sub_1 - 1;
        let rings_add_1 = rings + 1;
        let longs_add_1 = longitude + 1;
        let half_depth = depth * 0.5;
        let summit = half_depth + radius;

        // Vertex index offsets;
        let vert_offset_north_hemisphere = longitude;
        let vert_offset_north_equator =
            vert_offset_north_hemisphere + longs_add_1 * half_lats_sub_1;
        let vertex_offset_cylinder = vert_offset_north_equator + longs_add_1;
        let vertex_offset_south_equator = vertex_offset_cylinder + longs_add_1 * rings; // if ring is zero then vertex_offset_cylinder + 0;

        let vert_offset_south_hemisphere = vertex_offset_south_equator + longs_add_1;
        let vert_offset_south_polar = vert_offset_south_hemisphere + longs_add_1 * half_lats_sub_2;
        let vert_offset_south_cap = vert_offset_south_polar + longs_add_1;

        // Initialize arrays.
        let vert_len = vert_offset_south_cap + longitude;

        let mut vs: Vec<glam::Vec3A> = vec![glam::Vec3A::ZERO; vert_len];
        let mut vts: Vec<glam::Vec2> = vec![glam::Vec2::ZERO; vert_len];
        let mut vns: Vec<glam::Vec3A> = vec![glam::Vec3A::ZERO; vert_len];

        let inv_long = 1.0 / longitude as f32; // to_tex_horizontal
        let inv_lats = 1.0 / latitude as f32;

        let to_theta = 2.0 * std::f32::consts::PI * inv_long;
        let to_phi = std::f32::consts::PI * inv_lats;
        let to_tex_vertical = inv_lats * 2.0;

        let aspect_ratio_target = [
            radius / (depth + radius + radius), // CapsuleUvProfile::Aspect
            half_lats as f32 / (rings_add_1 + latitude) as f32, // CapsuleUvProfile::Uniform
            0.333_333_34,                       /* CapsuleUvProfile::Fixed f32 representation of
                                                 * 1.0 / 3.0 */
        ];

        let vt_aspect_ratio = aspect_ratio_target[v_profile as usize];

        let vt_aspect_north = 1.0 - vt_aspect_ratio;
        let vt_aspect_south = vt_aspect_ratio;

        let mut theta_cartesian: Vec<glam::Vec2> = vec![glam::Vec2::ZERO; longitude];
        let mut rho_theta_cartesian: Vec<glam::Vec2> = vec![glam::Vec2::ZERO; longitude];
        let mut s_texture_cache: Vec<f32> = vec![0.0; longs_add_1];

        for j in 0..longitude {
            let jf = j as f32;
            let s_texture_polar = 1.0 - ((jf + 0.5) * inv_long);
            let theta = jf * to_theta;

            let (sin_theta, cos_theta) = theta.sin_cos();

            theta_cartesian[j] = glam::Vec2::new(cos_theta, sin_theta);
            rho_theta_cartesian[j] = glam::Vec2::new(radius * cos_theta, radius * sin_theta);

            // North.
            vs[j] = glam::Vec3A::new(0.0, summit, 0.0);
            vts[j] = glam::Vec2::new(s_texture_polar, 1.0);
            vns[j] = glam::Vec3A::Y;

            // South.
            let idx = vert_offset_south_cap + j;
            vs[idx] = -vs[j]; // equivalent to glam::Vec3A::new(0.0, -summit, 0.0);
            vts[idx] = glam::Vec2::new(s_texture_polar, 0.0);
            vns[idx] = -glam::Vec3A::Y;
        }

        // Equatorial vertices.
        for (j, cache) in s_texture_cache.iter_mut().enumerate() {
            let s_texture = 1.0 - j as f32 * inv_long;
            *cache = s_texture;

            // Wrap to first element upon reaching last.
            let j_mod = j % longitude;

            let tc = theta_cartesian[j_mod];
            let rtc = rho_theta_cartesian[j_mod];

            // North equator.
            let index_n = vert_offset_north_equator + j;
            vs[index_n] = glam::Vec3A::new(rtc.x, half_depth, -rtc.y);
            vts[index_n] = glam::Vec2::new(s_texture, vt_aspect_north);
            vns[index_n] = glam::Vec3A::new(tc.x, 0.0, -tc.y);

            // South equator.
            let index_s = vertex_offset_south_equator + j;
            vs[index_s] = glam::Vec3A::new(rtc.x, -half_depth, -rtc.y);
            vts[index_s] = glam::Vec2::new(s_texture, vt_aspect_south);
            vns[index_s] = vns[index_n];
        }

        // Hemisphere vertices.
        for i in 0..half_lats_sub_1 {
            let ip1f = i as f32 + 1.0;
            let phi = ip1f * to_phi;

            // For coordinates.
            let (sin_phi_south, cos_phi_south) = phi.sin_cos();

            // Symmetrical hemispheres means cosine and sine only needs
            let cos_phi_north = sin_phi_south;
            let sin_phi_north = -cos_phi_south;

            let rho_cos_phi_north = radius * cos_phi_north;
            let rho_sin_phi_north = radius * sin_phi_north;
            let z_offset_north = half_depth - rho_sin_phi_north;

            let rho_cos_phi_south = radius * cos_phi_south;
            let rho_sin_phi_south = radius * sin_phi_south;
            let z_offset_south = -half_depth - rho_sin_phi_south;

            // For texture coordinates.
            let t_tex_fac = ip1f * to_tex_vertical;
            let cmp_l_tex_fac = 1.0 - t_tex_fac;
            let t_tex_north = cmp_l_tex_fac + vt_aspect_north * t_tex_fac;
            let t_tex_south = cmp_l_tex_fac * vt_aspect_south;

            let i_lon_add_1 = i * longs_add_1;
            let vert_curr_lat_north = vert_offset_north_hemisphere + i_lon_add_1;
            let vert_curr_lat_south = vert_offset_south_hemisphere + i_lon_add_1;

            for (j, &cache) in s_texture_cache.iter().enumerate() {
                let j_mod = j % longitude;

                let s_texture = cache;
                let tc = theta_cartesian[j_mod];

                // North hemisphere.
                let index_n = vert_curr_lat_north + j;
                vs[index_n] = glam::Vec3A::new(
                    rho_cos_phi_north * tc.x,
                    z_offset_north,
                    -rho_cos_phi_north * tc.y,
                );
                vts[index_n] = glam::Vec2::new(s_texture, t_tex_north);
                vns[index_n] =
                    glam::Vec3A::new(cos_phi_north * tc.x, -sin_phi_north, -cos_phi_north * tc.y);

                // South Hemisphere
                let index_s = vert_curr_lat_south + j;
                vs[index_s] = glam::Vec3A::new(
                    rho_cos_phi_south * tc.x,
                    z_offset_south,
                    -rho_cos_phi_south * tc.y,
                );
                vts[index_s] = glam::Vec2::new(s_texture, t_tex_south);
                vns[index_s] =
                    glam::Vec3A::new(cos_phi_south * tc.x, -sin_phi_south, -cos_phi_south * tc.y);
            }
        }

        // Cylinder vertices. (only if rings > 0).
        // Exclude both origin and destination edges
        // (North and South equators) from the interpolation.
        let to_fac = 1.0 / rings_add_1 as f32;
        let mut idx_cyl_lat = vertex_offset_cylinder;

        for h in 1..(rings_add_1 * calc_middle) {
            let fac = h as f32 * to_fac;
            let cmp_l_fac = 1.0 - fac;
            let t_texture = cmp_l_fac * vt_aspect_north + fac * vt_aspect_south;
            let z = half_depth - depth * fac;

            for (j, &cache) in s_texture_cache.iter().enumerate() {
                let j_mod = j % longitude;

                let s_texture = cache;

                let tc = theta_cartesian[j_mod];
                let rtc = rho_theta_cartesian[j_mod];

                vs[idx_cyl_lat] = glam::Vec3A::new(rtc.x, z, -rtc.y);
                vts[idx_cyl_lat] = glam::Vec2::new(s_texture, t_texture);
                vns[idx_cyl_lat] = glam::Vec3A::new(tc.x, 0.0, -tc.y);

                idx_cyl_lat += 1;
            }
        }

        // Triangle indices.

        // Stride is 3 for polar triangles;
        // stride is 6 for two triangles forming a quad.
        let lon_mul_3 = longitude * 3;
        let lon_mul_6 = lon_mul_3 << 1; // equivalent to (lon_mul_3 * 2);
        let hemisphere_lon = half_lats_sub_1 * lon_mul_6;

        let tri_offset_north_hemisphere = lon_mul_3;
        let tri_offset_cylinder = tri_offset_north_hemisphere + hemisphere_lon;
        let tri_offset_south_hemisphere = tri_offset_cylinder + rings_add_1 * lon_mul_6;
        let tri_offset_south_cap = tri_offset_south_hemisphere + hemisphere_lon;

        let fs_len = tri_offset_south_cap + lon_mul_3;
        let mut indices: Vec<usize> = vec![0; fs_len];

        // Polar caps.
        let mut i = 0;
        let mut k = 0;
        let mut m = tri_offset_south_cap;

        while i < longitude {
            // North.
            indices[k] = i;
            indices[k + 1] = vert_offset_north_hemisphere + i;
            indices[k + 2] = vert_offset_north_hemisphere + i + 1;

            // South.
            indices[m] = vert_offset_south_cap + i;
            indices[m + 1] = vert_offset_south_polar + i + 1;
            indices[m + 2] = vert_offset_south_polar + i;

            i += 1;
            k += 3;
            m += 3;
        }

        // Hemispheres.

        let mut i = 0;
        let mut k = tri_offset_north_hemisphere;
        let mut m = tri_offset_south_hemisphere;

        while i < half_lats_sub_1 {
            let i_lon_add_1 = i * longs_add_1;

            let vert_curr_lat_north = vert_offset_north_hemisphere + i_lon_add_1;
            let vert_next_lat_north = vert_curr_lat_north + longs_add_1;

            let vert_curr_lat_south = vertex_offset_south_equator + i_lon_add_1;
            let vert_next_lat_south = vert_curr_lat_south + longs_add_1;
            let mut j = 0;
            while j < longitude {
                // North.
                let north00 = vert_curr_lat_north + j;
                let north01 = vert_next_lat_north + j;
                let north11 = vert_next_lat_north + j + 1;
                let north10 = vert_curr_lat_north + j + 1;

                indices[k] = north00;
                indices[k + 1] = north11;
                indices[k + 2] = north10;

                indices[k + 3] = north00;
                indices[k + 4] = north01;
                indices[k + 5] = north11;

                // South.
                let south00 = vert_curr_lat_south + j;
                let south01 = vert_next_lat_south + j;
                let south11 = vert_next_lat_south + j + 1;
                let south10 = vert_curr_lat_south + j + 1;

                indices[m] = south00;
                indices[m + 1] = south11;
                indices[m + 2] = south10;

                indices[m + 3] = south00;
                indices[m + 4] = south01;
                indices[m + 5] = south11;

                j += 1;
                k += 6;
                m += 6;
            }

            i += 1;
        }

        // Cylinder.
        let mut i = 0;
        let mut k = tri_offset_cylinder;

        while i < rings_add_1 {
            let vert_curr_lat = vert_offset_north_equator + i * longs_add_1;
            let vert_next_lat = vert_curr_lat + longs_add_1;

            let mut j = 0;
            while j < longitude {
                let cy00 = vert_curr_lat + j;
                let cy01 = vert_next_lat + j;
                let cy11 = vert_next_lat + j + 1;
                let cy10 = vert_curr_lat + j + 1;

                indices[k] = cy00;
                indices[k + 1] = cy11;
                indices[k + 2] = cy10;

                indices[k + 3] = cy00;
                indices[k + 4] = cy01;
                indices[k + 5] = cy11;

                j += 1;
                k += 6;
            }

            i += 1;
        }

        let mut vertices: Vec<Vertex> = vec![Vertex::default(); vert_len];

        for i in 0..vert_len {
            vertices[i] = Vertex {
                position: vs[i].to_array(),
                tex_coord: vts[i].to_array(),
                normal: vns[i].to_array(),
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4],
            };
        }


        let mesh = Mesh {
            vertices,
            material_id: 0,
            indices: indices.into(),
        };

        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::capsule::Capsule;
    use crate::mesh::Model;

    #[test]
    fn test() {
        let capsule = Capsule::default();
        let capsule_model: Model = capsule.into();
        for vertex in &capsule_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.normal[0], vertex.normal[1], vertex.normal[2]
            );
        }
        println!("{:?}", capsule_model.meshes[0].indices);
    }
}
