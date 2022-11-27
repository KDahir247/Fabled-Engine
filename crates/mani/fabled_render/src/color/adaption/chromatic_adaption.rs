use fabled_math::matrix3x3_math::inverse_mat3;
use fabled_math::vector_math::rcp;
use fabled_math::{Matrix3x3, Vector3};
use crate::color
use crate::color::{s_rgb_to_linear, SRGB_TO_XYZ_MATRIX, XYZ_TO_SRGB_MATRIX};

// Look below for equation.
// http://brucelindbloom.com/index.html?Eqn_ChromAdapt.html
pub fn compute_adaption_matrix(
    src_tristimulus_white_point: Vector3,
    dst_tristmulus_white_point: Vector3,
    adaption_matrix: Matrix3x3,
) -> Matrix3x3 {
    let src_cone_response_domain = adaption_matrix * src_tristimulus_white_point;

    let src_cone_response_domain_rcp = Vector3 {
        value: rcp(src_cone_response_domain.value),
    };

    let dst_cone_response_domain = adaption_matrix * dst_tristmulus_white_point;

    let dst_diff_src = dst_cone_response_domain * src_cone_response_domain_rcp;

    let diagonal_diff_matrix = Matrix3x3::set(
        Vector3::set(dst_diff_src.x(), 0.0, 0.0),
        Vector3::set(0.0, dst_diff_src.y(), 0.0),
        Vector3::set(0.0, 0.0, dst_diff_src.z()),
    );

    let inverse_adaption_matrix = inverse_mat3(adaption_matrix);

    inverse_adaption_matrix * diagonal_diff_matrix * adaption_matrix
}

pub fn apply_adaption_matrix_tristimulus(
    tristimulus: Vector3,
    src_tristimulus_white_point: Vector3,
    dst_tristmulus_white_point: Vector3,
    adaption_matrix: Matrix3x3,
) -> Vector3 {
    let transform_matrix = compute_adaption_matrix(
        src_tristimulus_white_point,
        dst_tristmulus_white_point,
        adaption_matrix,
    );

    transform_matrix * tristimulus
}


pub fn apply_adaption_matrix_srgb(
    srgb_linear: Vector3,
    dst_tristimulus_white_point: Vector3,
    adaption_matrix: Matrix3x3,
) -> Vector3 {


    //SRGB D65
    let src_tristimulus = Vector3::set(0.9504559271, 1.0, 1.089057751);

    let tristimulus = SRGB_TO_XYZ_MATRIX * srgb_linear;


    let adapted_tristimulus = apply_adaption_matrix_tristimulus(tristimulus, src_tristimulus, dst_tristimulus_white_point, adaption_matrix);


    XYZ_TO_SRGB_MATRIX * adapted_tristimulus
}
