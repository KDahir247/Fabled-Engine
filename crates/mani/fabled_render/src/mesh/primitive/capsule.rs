use crate::mesh::primitive::capsule::CapsuleUvProfile::Aspect;
use crate::mesh::util::{clamp_ss, min_ss};
use crate::mesh::Model;

pub enum CapsuleUvProfile {
    Aspect,
    Uniform,
    Fixed,
}

pub struct Capsule {
    model: Model,
}

impl Default for Capsule {
    fn default() -> Self {
        Self::new(0.5, 5, 1., 16, 32, Aspect)
    }
}

impl Capsule {
    //radius (top?, bottom?, vertical?, horizontal? or both), length, center, rotation, axis, depth
    pub fn new(
        radius: f32,
        rings: usize,
        depth: f32,
        latitude: usize,
        longitude: usize,
        profile: CapsuleUvProfile,
    ) -> Capsule {
        /*
          code adapted from https://behreajj.medium.com/making-a-capsule-mesh-via-script-in-five-3d-environments-c2214abf02db
          provided by bevy. Thank you Bevy Community! \(ᵔᵕᵔ)/
        */

        let calc_middle = min_ss(rings as f32, 1.0);
        let half_lats = latitude >> 1; //equal to latitude / 2;
        let half_latsn1 = half_lats - 1;
        let half_latsn2 = half_latsn1 - 1;
        let ringsp1 = rings + 1;
        let longsp1 = longitude + 1;
        let half_depth = depth * 0.5;
        let summit = half_depth + radius;

        //Vertex index offsets;
        let vert_offset_north_hemi = longitude;
        let vert_offset_north_equator = vert_offset_north_hemi + longsp1 * half_latsn1;
        let vertex_offset_cylinder = vert_offset_north_equator + longsp1;
        let vertex_offset_south_equator = vertex_offset_cylinder + longsp1 * rings;

        let vert_offset_south_hemi = vertex_offset_south_equator + longsp1;
        let vert_offset_south_polar = vert_offset_south_hemi + longsp1 * half_latsn2;
        let vert_offset_south_cap = vert_offset_south_polar + longsp1;

        //Initialize arrays.
        let vert_len = vert_offset_south_cap + longitude;

        let mut vs: Vec<glam::Vec3A> = vec![glam::Vec3A::ZERO; vert_len];
        let mut vts: Vec<glam::Vec2> = vec![glam::Vec2::ZERO; vert_len];
        let mut vns: Vec<glam::Vec3A> = vec![glam::Vec3A::ZERO; vert_len];

        let to_tex_horizontal = 1.0 / longitude as f32;
        let inv_lats = 1.0 / latitude as f32;

        let to_theta = 2.0 * std::f32::consts::PI * to_tex_horizontal;
        let to_phi = std::f32::consts::PI * inv_lats;
        let to_tex_vertical = inv_lats * 2.0;

        let vt_aspect_ratio = match profile {
            Aspect => radius / (depth + radius + radius),
            CapsuleUvProfile::Uniform => half_lats as f32 / (ringsp1 + latitude) as f32,
            CapsuleUvProfile::Fixed => 0.33333334, //f32 representation of 1.0 / 3.0
        };

        let vt_aspect_north = 1.0 - vt_aspect_ratio;
        let vt_aspect_south = vt_aspect_ratio;

        let mut theta_cartesian: Vec<glam::Vec2> = vec![glam::Vec2::ZERO; longitude];
        let mut rho_theta_cartesian: Vec<glam::Vec2> = vec![glam::Vec2::ZERO; longitude];
        let mut s_texture_cache: Vec<f32> = vec![0.0; longsp1];

        for j in 0..longitude {
            let jf = j as f32;
            let s_texture_polar = 1.0 - ((jf + 0.5) * to_tex_horizontal);
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
            vs[idx] = glam::Vec3A::new(0.0, -summit, 0.0);
            vts[idx] = glam::Vec2::new(s_texture_polar, 0.0);
            vns[idx] = -glam::Vec3A::Y;
        }

        // Equatorial vertices.
        //println!("{} {}", (0..longsp1).len(), s_texture_cache.len());

        for j in 0..longsp1 {
            let s_texture = 1.0 - j as f32 * to_tex_horizontal;
            s_texture_cache[j] = s_texture;

            // Wrap to first element upon reaching last.
            let j_mod = j % longitude;
            let tc = theta_cartesian[j_mod];
            let rtc = rho_theta_cartesian[j_mod];

            // North equator.
            let idxn = vert_offset_north_equator + j;
            vs[idxn] = glam::Vec3A::new(rtc.x, half_depth, -rtc.y);
            vts[idxn] = glam::Vec2::new(s_texture, vt_aspect_north);
            vns[idxn] = glam::Vec3A::new(tc.x, 0.0, -tc.y);

            // South equator.
            let idxs = vertex_offset_south_equator + j;
            vs[idxs] = glam::Vec3A::new(rtc.x, -half_depth, -rtc.y);
            vts[idxs] = glam::Vec2::new(s_texture, vt_aspect_ratio);
            vns[idxs] = glam::Vec3A::new(tc.x, 0.0, -tc.y);
        }

        // Hemisphere vertices.
        for i in 0..half_latsn1 {
            let ip1f = i as f32 + 1.0;
            let phi = ip1f * to_phi;

            // For coordinates.
            let (sin_phi_south, cos_phi_south) = phi.sin_cos();

            // Symmetrical hemispheres means cosine and sine only needs
            // to be calculated once.
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
            let cmpl_tex_fac = 1.0 - t_tex_fac;
            let t_tex_north = cmpl_tex_fac + vt_aspect_north * t_tex_fac;
            let t_tex_south = cmpl_tex_fac * vt_aspect_south;

            let i_lonsp1 = i * longsp1;
            let vert_curr_lat_north = vert_offset_north_hemi + i_lonsp1;
            let vert_curr_lat_south = vert_offset_south_hemi + i_lonsp1;

            for j in 0..longsp1 {
                let j_mod = j % longitude;

                let s_texture = s_texture_cache[j];
                let tc = theta_cartesian[j_mod];

                // North hemisphere.
                let idxn = vert_curr_lat_north + j;
                vs[idxn] = glam::Vec3A::new(
                    rho_cos_phi_north * tc.x,
                    z_offset_north,
                    -rho_cos_phi_north * tc.y,
                );
                vts[idxn] = glam::Vec2::new(s_texture, t_tex_north);
                vns[idxn] =
                    glam::Vec3A::new(cos_phi_north * tc.x, -sin_phi_north, -cos_phi_north * tc.y);

                // South Hemisphere
                let idxs = vert_curr_lat_south + j;
                vs[idxs] = glam::Vec3A::new(
                    rho_cos_phi_south * tc.x,
                    z_offset_south,
                    -rho_cos_phi_south * tc.y,
                );
                vts[idxs] = glam::Vec2::new(s_texture, t_tex_south);
                vns[idxs] =
                    glam::Vec3A::new(cos_phi_south * tc.x, -sin_phi_south, -cos_phi_south * tc.y);
            }
        }

        // Cylinder vertices. (only if rings > 0).
        // Exclude both origin and destination edges
        // (North and South equators) from the interpolation.
        let to_fac = 1.0 / ringsp1 as f32;
        let mut idx_cyl_lat = vertex_offset_cylinder;

        for h in 1..(ringsp1 * calc_middle as usize) {
            let fac = h as f32 * to_fac;
            let cmpl_fac = 1.0 - fac;
            let t_texture = cmpl_fac * vt_aspect_north + fac * vt_aspect_south;
            let z = half_depth - depth * fac;

            for j in 0..longsp1 {
                let j_mod = j % longitude;
                let tc = theta_cartesian[j_mod];
                let rtc = rho_theta_cartesian[j_mod];
                let s_texture = s_texture_cache[j];

                vs[idx_cyl_lat] = glam::Vec3A::new(rtc.x, z, -rtc.y);
                vts[idx_cyl_lat] = glam::Vec2::new(s_texture, t_texture);
                vns[idx_cyl_lat] = glam::Vec3A::new(tc.x, 0.0, -tc.y);

                idx_cyl_lat += 1;
            }
        }

        //Triangle indices.

        // Stride is 3 for polar triangles;
        // stride is 6 for two triangles forming a quad.
        let lons3 = longitude * 3;
        let lons6 = lons3 * 2;
        let hemi_lons = half_latsn1 * lons6;

        let tri_offset_north_hemi = lons3;
        let tri_offset_cylinder = tri_offset_north_hemi + hemi_lons;
        let tri_offset_south_hemi = tri_offset_cylinder + ringsp1 * lons6;
        let tri_offset_south_cap = tri_offset_south_hemi + hemi_lons;

        let fs_len = tri_offset_south_cap + lons3;
        let mut tris: Vec<usize> = vec![0; fs_len];

        // Polar caps.
        let mut i = 0;
        let mut k = 0;
        let mut m = tri_offset_south_cap;

        while i < longitude {
            // North.
            tris[k] = i;
            tris[k + 1] = vert_offset_north_hemi + i;
            tris[k + 2] = vert_offset_north_hemi + i + 1;

            // South.
            tris[m] = vert_offset_south_cap + i;
            tris[m + 1] = vert_offset_south_polar + i + 1;
            tris[m + 2] = vert_offset_south_polar + i;

            i += 1;
            k += 3;
            m += 3;
        }

        // Hemispheres.

        let mut i = 0;
        let mut k = tri_offset_north_hemi;
        let mut m = tri_offset_south_hemi;

        while i < half_latsn1 {
            let i_lonsp1 = i * longsp1;

            let vert_curr_lat_north = vert_offset_north_hemi + i_lonsp1;
            let vert_next_lat_north = vert_curr_lat_north + longsp1;

            let vert_curr_lat_south = vert_offset_south_hemi + i_lonsp1;
            let vert_next_lat_south = vert_curr_lat_south + longsp1;
            let mut j = 0;
            while j < longitude {
                // North.
                let north00 = vert_curr_lat_north + j;
                let north01 = vert_next_lat_north + j;
                let north11 = vert_next_lat_north + j + 1;
                let north10 = vert_curr_lat_north + j + 1;

                tris[k] = north00;
                tris[k + 1] = north11;
                tris[k + 2] = north10;

                tris[k + 3] = north00;
                tris[k + 4] = north01;
                tris[k + 5] = north11;

                // South.
                let south00 = vert_curr_lat_south + j;
                let south01 = vert_next_lat_south + j;
                let south11 = vert_next_lat_south + j + 1;
                let south10 = vert_curr_lat_south + j + 1;

                tris[m] = south00;
                tris[m + 1] = south11;
                tris[m + 2] = south10;

                tris[m + 3] = south00;
                tris[m + 4] = south01;
                tris[m + 5] = south11;

                j += 1;
                k += 6;
                m += 6;
            }

            i += 1;
        }

        // Cylinder.
        let mut i = 0;
        let mut k = tri_offset_cylinder;

        while i < ringsp1 {
            let vert_curr_lat = vert_offset_north_equator + i * longsp1;
            let vert_next_lat = vert_curr_lat + longsp1;

            let mut j = 0;
            while j < longitude {
                let cy00 = vert_curr_lat + j;
                let cy01 = vert_next_lat + j;
                let cy11 = vert_next_lat + j + 1;
                let cy10 = vert_curr_lat + j + 1;

                tris[k] = cy00;
                tris[k + 1] = cy11;
                tris[k + 2] = cy10;

                tris[k + 3] = cy00;
                tris[k + 4] = cy01;
                tris[k + 5] = cy11;

                j += 1;
                k += 6;
            }

            i += 1;
        }

        //texture coords is vts
        //normals is vns
        //vertices is vs

        println!("{:#?}", vs);

        Self {
            model: Model { meshes: vec![] },
        }
    }
}

/*for a in 0..0{
    println!("{}",a);
}*/

/*for a in 1..0 {
println!("{}", a);
}*/

#[cfg(test)]
mod test {
    use crate::mesh::primitive::capsule::Capsule;

    #[test]
    fn test() {
        let capsule = Capsule::default();
    }
}
