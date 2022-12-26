use crate::color::component::ColorSpaceAdaption;
use crate::color::{cct_to_chromatic_coord, chromatic_coord_to_tri_stimulus_white};
use fabled_math::matrix3x3_math::inverse_mat3;
use fabled_math::vector_math::rcp;
use fabled_math::{Matrix3x3, Vector3};

// Look below for equation.
// http://brucelindbloom.com/index.html?Eqn_ChromAdapt.html
pub fn compute_adaption_matrix(
    src_tri_stimulus_white_point: Vector3,
    dst_tri_stimulus_white_point: Vector3,
    adaptation_param: ColorSpaceAdaption,
) -> Matrix3x3 {
    let src_cone_response_domain = adaptation_param.adaption_matrix * src_tri_stimulus_white_point;
    let dst_cone_response_domain = adaptation_param.adaption_matrix * dst_tri_stimulus_white_point;

    let src_cone_response_domain_rcp = Vector3 {
        value: rcp(src_cone_response_domain.value),
    };

    let dst_diff_src = dst_cone_response_domain * src_cone_response_domain_rcp;

    let diagonal_diff_matrix = Matrix3x3::set(
        dst_diff_src * Vector3::RIGHT,
        dst_diff_src * Vector3::UP,
        dst_diff_src * Vector3::FORWARD,
    );

    let inverse_adaption_matrix = inverse_mat3(adaptation_param.adaption_matrix);

    let src_destination_white_points =
        inverse_adaption_matrix * diagonal_diff_matrix * adaptation_param.adaption_matrix;

    let adapted_matrix = src_destination_white_points * adaptation_param.tri_stimulus_matrix;

    adapted_matrix
}

pub fn apply_adaption_matrix_cct(
    color: Vector3,
    cct_src: f32,
    cct_dst: f32,
    adaptation_param: ColorSpaceAdaption,
) -> Vector3 {
    let tri_stimulus = adaptation_param.tri_stimulus_matrix * color;

    let src_chromatic_coordinate = cct_to_chromatic_coord(cct_src);

    let src_tri_stimulus_white_point =
        chromatic_coord_to_tri_stimulus_white(src_chromatic_coordinate);

    let dst_chromatic_coordinate = cct_to_chromatic_coord(cct_dst);
    let dst_tri_stimulus_white_point =
        chromatic_coord_to_tri_stimulus_white(dst_chromatic_coordinate);

    apply_adaption_matrix_tri_stimulus(
        tri_stimulus,
        src_tri_stimulus_white_point,
        dst_tri_stimulus_white_point,
        adaptation_param,
    )
}


pub fn apply_adaption_matrix_tri_stimulus(
    tri_stimulus: Vector3,
    src_tri_stimulus_white_point: Vector3,
    dst_tri_stimulus_white_point: Vector3,
    adaptation_param: ColorSpaceAdaption,
) -> Vector3 {
    let adapted_transform_matrix = compute_adaption_matrix(
        src_tri_stimulus_white_point,
        dst_tri_stimulus_white_point,
        adaptation_param,
    );


    adapted_transform_matrix * tri_stimulus
}
