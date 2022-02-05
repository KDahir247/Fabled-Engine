use crate::camera::{ClippingPlane, Oblique, Orthographic, Projection, ViewPort};
use fabled_transform::{Rotation, Translation};

use fabled_math::Matrix4x4;

#[derive(Copy, Clone, Default, Debug)]
pub struct MatrixDescriptor {
    pub projection: Matrix4x4,
    pub view: Matrix4x4,
    pub model: Matrix4x4,
}

pub fn project(
    target: [f32; 3],
    viewport: &ViewPort,
    matrix_descriptor: MatrixDescriptor,
) -> [f32; 3] {
    let model = glam::Mat4::from_cols_array(&matrix_descriptor.model.inner);
    let projection = glam::Mat4::from_cols_array(&matrix_descriptor.projection.inner);
    let view = glam::Mat4::from_cols_array(&matrix_descriptor.view.inner);

    let t_mvp_target_vector =
        projection * view * model * glam::Vec4::new(target[0], target[1], target[2], 1.0);

    let normalized_factor = 1.0 / t_mvp_target_vector.w;

    assert!(normalized_factor.ne(&0.0));

    [
        (((t_mvp_target_vector.x * normalized_factor + 1.0) * 0.5) * viewport.w) + viewport.x,
        (((-t_mvp_target_vector.y * normalized_factor + 1.0) * 0.5) * viewport.h) + viewport.y,
        (t_mvp_target_vector.z * normalized_factor * (viewport.max_depth - viewport.min_depth))
            + viewport.min_depth,
    ]
}

pub fn unproject(
    target: [f32; 3],
    viewport: &ViewPort,
    matrix_descriptor: MatrixDescriptor,
) -> [f32; 3] {
    let model = glam::Mat4::from_cols_array(&matrix_descriptor.model.inner);
    let projection = glam::Mat4::from_cols_array(&matrix_descriptor.projection.inner);
    let view = glam::Mat4::from_cols_array(&matrix_descriptor.view.inner);

    let matrix = (projection * (view * model)).inverse();

    let vector = matrix
        * glam::vec4(
            ((target[0] - viewport.x) / viewport.w) * 2.0 - 1.0,
            -(((target[1] - viewport.y) / viewport.h) * 2.0 - 1.0),
            (target[2] - viewport.min_depth) / (viewport.max_depth - viewport.min_depth),
            1.0,
        );

    assert!(vector.w.ne(&0.0));

    let rcp_vec_w = 1.0 / vector.w;

    [
        vector.x * rcp_vec_w,
        vector.y * rcp_vec_w,
        vector.z * rcp_vec_w,
    ]
}

pub fn compute_look_at_matrix(
    translation: Translation,
    rotation: Rotation,
) -> Matrix4x4 {
    let forward = fabled_transform::forward(rotation);

    let translation_xyz = [
        translation.value[0],
        translation.value[1],
        translation.value[2],
    ];

    let rcp_translation_scalar = translation.value[3].recip();

    let position = glam::Vec3A::from(translation_xyz) * rcp_translation_scalar;

    let z_axis = -glam::Vec3A::from(forward).normalize_or_zero();
    let x_axis = glam::Vec3A::Y.cross(z_axis).normalize_or_zero();
    let y_axis = z_axis.cross(x_axis);

    let t_axis = glam::Vec3A::new(
        -position.dot(x_axis),
        -position.dot(y_axis),
        -position.dot(z_axis),
    );

    [
        x_axis.x, y_axis.x, z_axis.x, 0.0, // 0
        x_axis.y, y_axis.y, z_axis.y, 0.0, // 1
        x_axis.z, y_axis.z, z_axis.z, 0.0, // 2
        t_axis.x, t_axis.y, t_axis.z, 1.0,
    ]
    .into()
}

#[deprecated(note = "Calculate arc ball matrix function has not been tested.")]
pub fn compute_arc_ball_matrix(
    translation: Translation,
    rotation: Rotation,
    center: Option<[f32; 3]>,
) -> Matrix4x4 {
    let rcp_translation_scalar = translation.value[3].recip();

    let normalized_translation = glam::Vec3::from([
        translation.value[0],
        translation.value[1],
        translation.value[2],
    ]) * rcp_translation_scalar;

    let rotation = glam::Quat::from_array(rotation.value);

    let center = glam::Vec3::from(center.unwrap_or_default());

    let rotation_translation_matrix =
        glam::Mat4::from_rotation_translation(rotation.inverse(), -normalized_translation);

    let translation_to_center = glam::Mat4::from_translation(-center);

    (rotation_translation_matrix * translation_to_center)
        .to_cols_array()
        .into()
}

pub fn compute_inverse_view_matrix(view_matrix: Matrix4x4) -> Matrix4x4 {
    let view_matrix = glam::Mat4::from_cols_array(&view_matrix.inner);

    view_matrix.inverse().to_cols_array().into()
}

#[rustfmt::skip]
pub fn compute_projection_matrix(projection: Projection, clipping_plane : ClippingPlane) -> Matrix4x4 {

    let far_plane = clipping_plane.far;
    let near_plane = clipping_plane.near;

    let near_plane_min_far_plane = near_plane - far_plane;

    match projection {
        Projection::Orthographic(orthographic) => {

            let Orthographic {
                right,
                left,
                top,
                bottom,
            } = orthographic;
            
            let right_min_left = right - left;
            let right_plus_left = right + left;
            let top_min_bottom = top - bottom;
            let top_plus_bottom = top + bottom;

            [
                2.0 / right_min_left, 0.0, 0.0, 0.0, // 0
                0.0, 2.0 / top_min_bottom, 0.0, 0.0, // 1
                0.0, 0.0, -1.0 / near_plane_min_far_plane, 0.0, // 2
                -(right_plus_left / right_min_left), -(top_plus_bottom / top_min_bottom), -(far_plane / near_plane_min_far_plane), 1.0 // 3
            ]
            
        }
        Projection::Perspective(perspective) => {

            let h = 1.0 / (perspective.fov.radian * 0.5).tan();
            let w = h / (perspective.aspect.horizontal / perspective.aspect.vertical);


            let inv_near_min_far = 1.0 / near_plane_min_far_plane;

             [
                w, 0.0, 0.0, 0.0, // 0
                0.0, h , 0.0, 0.0, // 1
                0.0, 0.0, far_plane * inv_near_min_far, -1.0, // 2
                0.0, 0.0, near_plane * far_plane * inv_near_min_far, 0.0, // 3
            ]
        }
    }.into()

}


#[rustfmt::skip]
pub fn compute_oblique_projection_matrix(orthographic : Orthographic, oblique : Oblique, clipping_plane: ClippingPlane) -> Matrix4x4{
    
    let projection = compute_projection_matrix(Projection::Orthographic(orthographic), clipping_plane).inner;
    
    let size = oblique.vertical_position / orthographic.top;
    
    let rotation_x = size * -oblique.angle_rad.sin();
    let rotation_y = -size * -oblique.angle_rad.cos();

    let depth_offset_x = -oblique.depth_offset * rotation_x;
    let depth_offset_y = -oblique.depth_offset * rotation_y;

    [
        projection[0], projection[1], rotation_x, depth_offset_x, // 0
        projection[4], projection[5], rotation_y, depth_offset_y, // 1
        projection[8], projection[9], projection[10], projection[11], // 2
        projection[12], projection[13], projection[14], projection[15], // 3
    ].into()
}

pub fn compute_inverse_projection_matrix(projection_matrix: Matrix4x4) -> Matrix4x4 {
    let projection = glam::Mat4::from_cols_array(&projection_matrix.inner);

    projection.inverse().to_cols_array().into()
}

#[cfg(test)]
mod camera_matrix_test {
    use crate::camera::{
        compute_inverse_projection_matrix, compute_inverse_view_matrix, compute_look_at_matrix,
        compute_oblique_projection_matrix, compute_projection_matrix, project, unproject,
        ClippingPlane, MatrixDescriptor, Oblique, Orthographic, Perspective, Projection, ViewPort,
    };
    use fabled_math::Matrix4x4;

    use fabled_transform::{Rotation, Translation};

    #[test]
    fn calculate_project() {
        const THRESHOLD: f32 = 0.00002;

        // Tested in other game engine "project" function
        //         10.734505  -10.861914  0.9197902
        const TESTED_RESULT: [f32; 3] = [10.734505, -10.861914, 0.9197902];

        let translation_target = Translation {
            value: [20.0, 1.0, 0.3, 1.0],
        };

        let rotation_target = Rotation {
            value: [0.2497666, -0.2125415, 0.9384303, 0.1085877],
        };

        let point_target = [1.0, 15.0, 20.2];

        let (view_matrix, projection_matrix) =
            compute_view_projection(rotation_target, translation_target);

        let viewport = ViewPort::default();

        let project_vector = project(
            point_target,
            &viewport,
            MatrixDescriptor {
                projection: projection_matrix,
                view: view_matrix,
                model: Matrix4x4::default(),
            },
        );

        assert!(!project_vector[0].is_nan());
        assert!(!project_vector[1].is_nan());
        assert!(!project_vector[2].is_nan());

        assert!((TESTED_RESULT[0] - project_vector[0]).abs() < THRESHOLD);
        assert!((TESTED_RESULT[1] - project_vector[1]).abs() < THRESHOLD);
        assert!((TESTED_RESULT[2] - project_vector[2]).abs() < THRESHOLD);
    }

    #[test]
    fn calculate_unproject() {
        // Threshold to determine if passed due to float precision error.
        const THRESHOLD: f32 = 0.2;

        // Tested in other game engine "unproject" function
        //         90.0543  1.3233609  23.065624
        const TESTED_RESULT: [f32; 3] = [90.0543, 1.3233609, 23.065624];

        let translation_target = Translation {
            value: [90.0, 1.234, 23.3, 1.0],
        };

        let rotation_target = Rotation {
            value: [0.6212777, 0.0184258, 0.7806049, 0.0658067],
        };

        let point_target = [23.0, 15.333, 20.2];

        let (view_matrix, projection_matrix) =
            compute_view_projection(rotation_target, translation_target);

        // Create viewport and calculate the project.
        let viewport = ViewPort::default();

        let unprojected_vector = unproject(
            point_target,
            &viewport,
            MatrixDescriptor {
                projection: projection_matrix,
                view: view_matrix,
                model: Matrix4x4::default(),
            },
        );

        assert!(!unprojected_vector[0].is_nan());
        assert!(!unprojected_vector[1].is_nan());
        assert!(!unprojected_vector[2].is_nan());


        assert!((TESTED_RESULT[0] - unprojected_vector[0]).abs() < THRESHOLD);
        assert!((TESTED_RESULT[1] - unprojected_vector[1]).abs() < THRESHOLD);
        assert!((TESTED_RESULT[2] - unprojected_vector[2]).abs() < THRESHOLD);
    }

    fn compute_view_projection(
        rotation_target: Rotation,
        translation_target: Translation,
    ) -> (Matrix4x4, Matrix4x4) {
        let view_matrix =
            compute_look_at_matrix(translation_target, rotation_target);

        let perspective = Perspective::default();
        let projection = Projection::Perspective(perspective);

        let clipping_plane = ClippingPlane::default();

        let projection_matrix = compute_projection_matrix(projection, clipping_plane);
        (view_matrix, projection_matrix)
    }

    #[test]
    fn calc_look_at_matrix() {
        let translation = Translation {
            value: [10.0f32, 2.0f32, 30.0f32, 1.0f32],
        };

        let rotation = Rotation {
            value: [0.5213338, -0.4777144, 0.7064338, 0.0308436],
        };

        let view_matrix = compute_look_at_matrix(translation, rotation);

        println!("view matrix {:?}", view_matrix);
    }

    #[test]
    fn calc_arc_ball_matrix() {
        // will test arc ball matrix later. since I have nothing to test it
    }

    #[test]
    fn calculate_inverse_matrix() {
        let translation = Translation {
            value: [10.0f32, 20.0f32, 30.0f32, 1.0],
        };

        let rotation = Rotation {
            value: [-0.6532815, -0.2705981, -0.3265056, -0.6272114],
        };

        let (view_matrix, _) = compute_view_projection(rotation, translation);

        let inv_view = glam::Mat4::from_cols_array(&view_matrix.inner)
            .inverse()
            .to_cols_array();

        let inverse_view = compute_inverse_view_matrix(view_matrix);

        assert!(inv_view.eq(&inverse_view.inner));
    }

    #[test]
    fn calculate_projection_matrix() {
        const THRESHOLD: f32 = 0.0003;

        let translation = Translation {
            value: [1.0, 23.0, 15.0, 1.0],
        };

        let rotation = Rotation {
            value: [0.2141074, 0.0374137, 0.4750758, 0.8526788],
        };

        let (_, projection_matrix) = compute_view_projection(rotation, translation);

        // Our Game engine
        //[
        // 0.9742786, 0.0, 0.0, 0.0,
        // 0.0, 1.7320509, 0.0, 0.0,
        // 0.0, 0.0, -1.0001, -1.0,
        // 0.0, 0.0, -0.10001, 0.0
        // ]

        // Proven Game engine
        // 0.97428	0.00000	0.00000	0.00000
        // 0.00000	1.73205	0.00000	0.00000
        // 0.00000	0.00000	-1.00020	-1.00000
        // 0.00000	0.00000	-0.20002	0.00000

        // Proven C# Matrix4x4 Core Library
        //{
        // {M11:0.9742759 M12:0 M13:0 M14:0}
        // {M21:0 M22:1.732046 M23:0 M24:0}
        // {M31:0 M32:0 M33:-1.0001 M34:-1}
        // {M41:0 M42:0 M43:-0.10001 M44:0}
        // }

        let proven_projection_matrix = [
            0.9742759, 0.00000, 0.00000, 0.00000, 0.00000, 1.732046, 0.00000, 0.00000, 0.00000,
            0.00000, -1.00020, -1.00000, 0.00000, 0.00000, -0.10001, 0.00000,
        ];

        assert!((projection_matrix.inner[0] - proven_projection_matrix[0]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[1] - proven_projection_matrix[1]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[2] - proven_projection_matrix[2]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[3] - proven_projection_matrix[3]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[4] - proven_projection_matrix[4]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[5] - proven_projection_matrix[5]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[6] - proven_projection_matrix[6]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[7] - proven_projection_matrix[7]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[8] - proven_projection_matrix[8]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[9] - proven_projection_matrix[9]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[10] - proven_projection_matrix[10]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[11] - proven_projection_matrix[11]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[12] - proven_projection_matrix[12]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[13] - proven_projection_matrix[13]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[14] - proven_projection_matrix[14]).abs() < THRESHOLD);
        assert!((projection_matrix.inner[15] - proven_projection_matrix[15]).abs() < THRESHOLD);
    }

    #[test]
    fn calculate_inverse_projection_matrix() {
        let translation = Translation {
            value: [10.0f32, 20.0f32, 30.0f32, 1.0],
        };

        let rotation = Rotation {
            value: [-0.6532815, -0.2705981, -0.3265056, -0.6272114],
        };

        let (_, projection_matrix) = compute_view_projection(rotation, translation);

        let inv_proj = glam::Mat4::from_cols_array(&projection_matrix.inner)
            .inverse()
            .to_cols_array();

        let inverse_projection_matrix = compute_inverse_projection_matrix(projection_matrix).inner;

        assert!(inverse_projection_matrix.eq(&inv_proj));
    }


    #[rustfmt::skip]
    #[test]
    fn calculate_oblique_projection_matrix() {
        let error_threshold = 0.001;

        // Angle 64
        // Vertical 0.055
        // Depth 8.64
        // Lhs coordinate OpenGL
        // 0.11250	    0.00000	    -0.00989	 0.08542
        // 0.00000	    0.20000	    -0.00482	 0.04166
        // 0.00000	    0.00000	    -0.00200    -1.00020
        // 0.00000	    0.00000	     0.00000	 1.00000

        // Rhs coordinate WEBGPU
        //  0.11249986,  0.0,    -0.009886734,   0.08542138,
        //  0.0,         0.2,     0.004822083,  -0.041662797,
        //  0.0,         0.0,     0.0010000999,  0.0,
        // -0.0,        -0.0,     1.0001,        1.0
        
        let proven_oblique_matrix = [
            0.11250, 0.00000, -0.00989, 0.08542, // 0
            0.00000, 0.20000, 0.00482, -0.04166, // 1
            0.00000, 0.00000, 0.00100,  0.00000, // 2
            0.00000, 0.00000, 1.0001,  1.00000, // 3
        ];

        
        let orthographic_detail = Orthographic {
            right: 8.8889,
            left: -8.8889,
            top: 5.0,
            bottom: -5.0,

        };

        let oblique_detail = Oblique {
            angle_rad: 64.0f32.to_radians(),
            vertical_position: 0.055,
            depth_offset: 8.64,
        };

        let clipping_plane = ClippingPlane::default();

        let projection =
            compute_oblique_projection_matrix(orthographic_detail, oblique_detail, clipping_plane).inner;
        

        println!("{} {}", proven_oblique_matrix[10], projection[10]);
        println!("{} {}", proven_oblique_matrix[14], projection[14]);

        assert!((proven_oblique_matrix[0] - projection[0]).abs() < error_threshold);
        assert!((proven_oblique_matrix[1] - projection[1]).abs() < error_threshold);
        assert!((proven_oblique_matrix[2] - projection[2]).abs() < error_threshold);
        assert!((proven_oblique_matrix[3] - projection[3]).abs() < error_threshold);
        assert!((proven_oblique_matrix[4] - projection[4]).abs() < error_threshold);
        assert!((proven_oblique_matrix[5] - projection[5]).abs() < error_threshold);
        assert!((proven_oblique_matrix[6] - projection[6]).abs() < error_threshold);
        assert!((proven_oblique_matrix[7] - projection[7]).abs() < error_threshold);
        assert!((proven_oblique_matrix[8] - projection[8]).abs() < error_threshold);
        assert!((proven_oblique_matrix[9] - projection[9]).abs() < error_threshold);
        assert!((proven_oblique_matrix[10] - projection[10]).abs() < error_threshold);
        assert!((proven_oblique_matrix[11] - projection[11]).abs() < error_threshold);
        assert!((proven_oblique_matrix[12] - projection[12]).abs() < error_threshold);
        assert!((proven_oblique_matrix[13] - projection[13]).abs() < error_threshold);
        assert!((proven_oblique_matrix[14] - projection[14]).abs() < error_threshold);
        assert!((proven_oblique_matrix[15] - projection[15]).abs() < error_threshold);


        println!("oblique projection \n{:?}", projection);
    }


    #[test]
    fn convert_test() {
        let error_threshold = 0.2;

        let rotation = Rotation {
            value: [-0.6532815, -0.2705981, -0.3265056, -0.6272114],
        };

        let translation = Translation {
            value: [10.0f32, 20.0f32, 30.0f32, 1.0f32],
        };

        let (view_matrix, projection_matrix) = compute_view_projection(rotation, translation);

        let viewport = ViewPort {
            x: 0.0,
            y: 0.0,
            w: 3840.0,
            h: 2160.0,
            min_depth: 0.1,
            max_depth: 1000.0,
        };

        let starting_value = [25.333, 12.27, 0.1];

        println!("starting value is {:?}", starting_value);

        let start_unproject = unproject(
            starting_value,
            &viewport,
            MatrixDescriptor {
                projection: projection_matrix,
                view: view_matrix,
                model: Matrix4x4::default(),
            },
        );
        println!("unproject value is {:?}", start_unproject);

        let start_project = project(
            start_unproject,
            &viewport,
            MatrixDescriptor {
                projection: projection_matrix,
                view: view_matrix,
                model: Matrix4x4::default(),
            },
        );

        println!("project value is {:?}", start_project);

        let end_unproject = unproject(
            start_project,
            &viewport,
            MatrixDescriptor {
                projection: projection_matrix,
                view: view_matrix,
                model: Matrix4x4::default(),
            },
        );

        println!("unproject value is {:?}", end_unproject);

        assert!((starting_value[0] - start_project[0]).abs() < error_threshold);
        assert!((starting_value[1] - start_project[1]).abs() < error_threshold);
        assert!((starting_value[2] - start_project[2]).abs() < error_threshold);

        assert!((start_unproject[0] - end_unproject[0]).abs() < error_threshold);
        assert!((start_unproject[1] - end_unproject[1]).abs() < error_threshold);
        assert!((start_unproject[2] - end_unproject[2]).abs() < error_threshold)
    }
}
