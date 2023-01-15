use crate::PerspectiveCameraViewMut;
use fabled_math::matrix4x4_math::inverse_mat4;
use fabled_math::vector_math::{length, normalize, pow, round};
use fabled_math::{from_euler_quat, EulerOrder, Matrix4x4, Swizzles4, Vector3, Vector4};
use fabled_render::camera::{
    compute_look_at_matrix, compute_orthographic_matrix, compute_perspective_matrix, AspectRatio,
    ClippingPlane, Fov, RenderProjection, RenderView,
};
use fabled_render::light::{CascadeFrustum, CascadeSplit};
use fabled_transform::{Rotation, Translation};
use shipyard::{
    EntitiesViewMut, IntoIter, IntoWithId, IntoWorkload, UniqueView, UniqueViewMut, View, ViewMut,
    Workload,
};

const MAX_CASCADE_SIZE: usize = 4;

fn compute_csm_split_system(
    plane: View<ClippingPlane>,
    mut csm_split: UniqueViewMut<CascadeSplit>,
) {
    for clipping_plane in plane.iter() {
        let clipping_range = clipping_plane.far - clipping_plane.near;

        let ratio = clipping_plane.far / clipping_plane.near;
        let rcp_clipping_range = 1.0 / clipping_range;

        let lambda = Vector4::broadcast(csm_split.lambda);

        // 4 Cascades
        let p = Vector4::set(0.25, 0.5, 0.75, 1.0);

        let p_pow = Vector4 {
            value: pow(Vector4::broadcast(ratio).value, p.value),
        };

        let p_mul_range = p * clipping_range;
        let log = p_pow * clipping_plane.near;

        let uniform = p_mul_range + clipping_plane.near;
        let log_min_uniform = log - uniform;
        let d = lambda * log_min_uniform + uniform;

        // X = split_1, Y = split_2, Z = split_3, W = split_4
        let cascade_split = (d - clipping_plane.near) * rcp_clipping_range;

        csm_split.splits = cascade_split;
    }
}

fn compute_csm_frustum_system(
    cascade_split: UniqueView<CascadeSplit>,
    projection: UniqueView<RenderProjection>,
    view: UniqueView<RenderView>,
    mut frustum: UniqueViewMut<CascadeFrustum>,
) {
    let inverse_view_projection = inverse_mat4(view.view_matrix * projection.projection_matrix);

    for cascade_index in 0..MAX_CASCADE_SIZE {
        let mut frustum_corner_ws = [
            Vector3::set(-1.0, 1.0, 0.0),
            Vector3::set(1.0, 1.0, 0.0),
            Vector3::set(1.0, -1.0, 0.0),
            Vector3::set(-1.0, -1.0, 0.0),
            Vector3::set(-1.0, 1.0, 1.0),
            Vector3::set(1.0, 1.0, 1.0),
            Vector3::set(1.0, -1.0, 1.0),
            Vector3::set(-1.0, -1.0, 1.0),
        ];

        let prev_split_distance = if cascade_index == 0 {
            0.0
        } else {
            cascade_split.splits.value[cascade_index - 1]
        };

        let current_split_distance = cascade_split.splits.value[cascade_index];

        for index in 0..8 {
            // temp solution
            let inverse_point = inverse_view_projection
                * Vector4::set(
                    frustum_corner_ws[index].x(),
                    frustum_corner_ws[index].y(),
                    frustum_corner_ws[index].z(),
                    1.0,
                );

            frustum_corner_ws[index] = inverse_point.xyz() / inverse_point.w();
        }


        for index in 0..4 {
            let corner_ray = frustum_corner_ws[index + 4] - frustum_corner_ws[index];
            let near_corner_ray = corner_ray * prev_split_distance;
            let far_corner_ray = corner_ray * current_split_distance;
            frustum_corner_ws[index + 4] = frustum_corner_ws[i] + far_corner_ray;
            frustum_corner_ws[i] = frustum_corner_ws[index] + near_corner_ray;
        }

        let mut frustum_center = Vector3::ZERO;

        for index in 0..8 {
            frustum_center += frustum_corner_ws[index];
        }
        frustum_center *= 0.125;

        let mut sphere_radius = 0.0f32;

        for index in 0..8 {
            let dist = length((frustum_corner_ws[index] - frustum_center).value);
            sphere_radius = sphere_radius.max(dist);
        }

        sphere_radius = (sphere_radius * 16.0).ceil() * 0.0625;

        let max_extent = Vector3::broadcast(sphere_radius);
        let min_extent = -max_extent;

        // assignment
        frustum.max_extent[cascade_index] = max_extent;
        frustum.min_extent[cascade_index] = min_extent;
        frustum.center[cascade_index] = frustum_center;
    }
}


fn compute_csm_shadow_matrix_system(
    cascade_frustum: UniqueViewMut<CascadeFrustum>,
    mut projection: UniqueView<RenderProjection>,
    mut view: UniqueView<RenderView>,
) {
    for cascade_index in 0..MAX_CASCADE_SIZE {
        let maximum_extent = cascade_frustum.max_extent[cascade_index];
        let minimum_extent = cascade_frustum.min_extent[cascade_index];

        let cascade_extent = maximum_extent - minimum_extent;

        let orthographic_rect = Vector4::set(
            maximum_extent.x(),
            minimum_extent.x(),
            maximum_extent.y(),
            minimum_extent.y(),
        );

        let mut light_orthographic_matrix = compute_orthographic_matrix(
            orthographic_rect,
            ClippingPlane::new(cascade_extent.z(), 0.0),
        );

        // todo: khal the direction is hard coded.
        let light_direction = cascade_frustum.center[cascade_index]
            - Vector3 {
                value: normalize(Vector3::set(-0.1, -0.5, 0.0).value),
            } * -maximum_extent.z();

        let light_view_matrix = compute_look_at_matrix(
            cascade_frustum.center[cascade_index],
            Vector3::UP,
            from_euler_quat(light_direction, EulerOrder::XYZ),
        );

        let shadow_matrix = light_orthographic_matrix * light_view_matrix;

        let shadow_origin = (shadow_matrix * Vector4::W) * (2048.0 * 0.5);

        let rounded_origin = Vector4 {
            value: round(shadow_origin.value),
        };

        let mut round_offset = rounded_origin - shadow_origin;

        round_offset = round_offset * 2.0 / 2048.0;

        let shadow_w = shadow_matrix.column_w + round_offset;

        light_orthographic_matrix = Matrix4x4::set(
            light_orthographic_matrix.column_x,
            light_orthographic_matrix.column_y,
            light_orthographic_matrix.column_z,
            shadow_w,
        );

        projection.projection_matrix = light_orthographic_matrix;
        view.view_matrix = light_view_matrix;
    }
}


pub fn construct_cascade_shadow_map() -> Workload {
    (
        compute_csm_split_system,
        compute_csm_frustum_system,
        compute_csm_shadow_matrix_system,
    )
        .into_workload()
}
