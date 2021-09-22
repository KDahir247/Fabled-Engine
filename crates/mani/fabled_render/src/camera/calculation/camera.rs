use crate::camera::{Oblique, Orthographic, Projection, ViewPort};
use fabled_transform::Orientation;

use glam::Vec4Swizzles;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub view: [f32; 16],
    pub proj: [f32; 16],
    pub inv_proj: [f32; 16],
    pub inv_view: [f32; 16],
}

impl Camera {
    pub fn project(&self, target: [f32; 3], model: [f32; 16], viewport: &ViewPort) -> [f32; 3] {
        let model_representation = glam::Mat4::from_cols_array(&model);
        let projection_representation = glam::Mat4::from_cols_array(&self.proj);
        let view_representation = glam::Mat4::from_cols_array(&self.view);

        let vector = projection_representation
            * view_representation
            * model_representation
            * glam::Vec4::new(target[0], target[1], target[2], 1.0);

        let normalized_factor = 1.0 / vector.w;

        assert!(normalized_factor.ne(&0.0));

        let project = glam::Vec3::new(
            (((vector.x * normalized_factor + 1.0) * 0.5) * viewport.w) + viewport.x,
            (((-vector.y * normalized_factor + 1.0) * 0.5) * viewport.h) + viewport.y,
            (vector.z * normalized_factor * (viewport.max_depth - viewport.min_depth))
                + viewport.min_depth,
        );

        project.to_array()
    }

    pub fn unproject(&self, target: [f32; 3], model: [f32; 16], viewport: &ViewPort) -> [f32; 3] {
        let model_representation = glam::Mat4::from_cols_array(&model);
        let projection_representation = glam::Mat4::from_cols_array(&self.proj);
        let view_representation = glam::Mat4::from_cols_array(&self.view);

        let matrix =
            (projection_representation * (view_representation * model_representation)).inverse();

        let mut vector = glam::vec4(
            ((target[0] - viewport.x) / viewport.w) * 2.0 - 1.0,
            -(((target[1] - viewport.y) / viewport.h) * 2.0 - 1.0),
            (target[2] - viewport.min_depth) / (viewport.max_depth - viewport.min_depth),
            1.0,
        );

        vector = matrix * vector;

        assert!(vector.w.ne(&0.0));

        let result = vector.xyz() / vector.w;

        result.to_array()
    }


    pub fn calculate_look_at_matrix(&mut self, orientation: Orientation) {
        let Orientation {
            transform, forward, ..
        } = orientation;

        let transformation_matrix = transform.get_transformation_matrix();

        let position = [
            transformation_matrix[12],
            transformation_matrix[13],
            transformation_matrix[14],
        ];

        let position = glam::Vec3A::from(position);


        let z_axis = -glam::Vec3A::from(forward).normalize();
        let x_axis = glam::Vec3A::Y.cross(z_axis).normalize();
        let y_axis = z_axis.cross(x_axis);


        let w = glam::Vec3A::new(
            -position.dot(x_axis),
            -position.dot(y_axis),
            -position.dot(z_axis),
        );

        let view_matrix = [
            x_axis.x, y_axis.x, z_axis.x, 0.0, // 0
            x_axis.y, y_axis.y, z_axis.y, 0.0, // 1
            x_axis.z, y_axis.z, z_axis.z, 0.0, // 2
            w.x, w.y, w.z, 1.0, // 3
        ];

        self.view = view_matrix;
    }

    #[deprecated(note = "Calculate arc ball matrix has not been tested.")]
    pub fn calculate_arc_ball_matrix(
        &mut self,
        orientation: Orientation,
        center: Option<[f32; 3]>,
    ) {
        let transformation_matrix = orientation.transform.get_transformation_matrix();

        let mat4_transformation_representation =
            glam::Mat4::from_cols_array(&transformation_matrix);

        let (_, rotation, _) = mat4_transformation_representation.to_scale_rotation_translation();

        let translation = mat4_transformation_representation.w_axis.xyz()
            / mat4_transformation_representation.w_axis.w;

        let center = glam::Vec3::from(center.unwrap_or([0.0; 3]));

        let rotation_translation =
            glam::Mat4::from_rotation_translation(rotation.inverse(), -translation);
        let translation_to_center = glam::Mat4::from_translation(-center);

        let result_view = rotation_translation * translation_to_center;

        self.view = result_view.to_cols_array();
    }

    pub fn calculate_inverse_view_matrix(&mut self) {
        let mat4_view_representation = glam::Mat4::from_cols_array(&self.view);
        let inverse_view_matrix = mat4_view_representation.inverse();

        self.inv_view = inverse_view_matrix.to_cols_array()
    }

    #[rustfmt::skip]
    pub fn calculate_projection_matrix(&mut self, projection: Projection) {
        match projection {
            Projection::Orthographic(orthographic) => {

                let Orthographic {
                    right,
                    left,
                    top,
                    bottom,
                    clipping,
                } = orthographic;

                let right_min_left = right - left;
                let right_plus_left = right + left;
                let top_min_bottom = top - bottom;
                let top_plus_bottom = top + bottom;
                let far_min_near = clipping.far - clipping.near;

                let orthographic_matrix = [
                    2.0 / right_min_left, 0.0, 0.0, 0.0, // 0
                    0.0, 2.0 /  top_min_bottom, 0.0, 0.0, // 1
                    0.0, 0.0, -1.0 / far_min_near, 0.0, // 2
                    -(right_plus_left / right_min_left), -(top_plus_bottom / top_min_bottom), -(clipping.near / far_min_near), 1.0, // 3
                ];


                self.proj = orthographic_matrix;
            }


            Projection::Perspective(perspective) => {

                let h = 1.0 / (perspective.fov.radian * 0.5).tan();
                let w = h / (perspective.aspect.horizontal / perspective.aspect.vertical);

                let near_min_far = perspective.clipping.near - perspective.clipping.far;

                let inv_near_min_far = 1.0 / near_min_far;

                let projection_matrix = [
                    w, 0.0, 0.0, 0.0, // 0
                    0.0, h , 0.0, 0.0, // 1
                    0.0, 0.0, perspective.clipping.far * inv_near_min_far, -1.0, // 2
                    0.0, 0.0, perspective.clipping.near * perspective.clipping.far * inv_near_min_far, 0.0, // 3
                ];


                self.proj = projection_matrix;
            }
        }
    }

    #[rustfmt::skip]
    pub fn calculate_oblique_projection_matrix(&mut self, orthographic: Orthographic, oblique: Oblique) {
        Self::calculate_projection_matrix(self, Projection::Orthographic(orthographic));

        let size = oblique.vertical_position / orthographic.top;

        let rotation_x =  size * -oblique.angle_rad.sin();
        let rotation_y  = -size * -oblique.angle_rad.cos();

        self.proj = [
            self.proj[0], self.proj[1], rotation_x, -oblique.depth_offset * rotation_x, // 0
            self.proj[4], self.proj[5], rotation_y, -oblique.depth_offset * rotation_y, // 1
            self.proj[8], self.proj[9], self.proj[10], self.proj[11], // 2
            self.proj[12], self.proj[13], self.proj[14], self.proj[15]
        ]
    }

    pub fn calculate_inverse_projection_matrix(&mut self) {
        let mat4_projection_representation = glam::Mat4::from_cols_array(&self.proj);
        let inverse_view_matrix = mat4_projection_representation.inverse();

        self.inv_proj = inverse_view_matrix.to_cols_array()
    }
}

#[cfg(test)]
mod camera_matrix_test {

    use crate::camera::{
        Camera, ClippingPlane, Oblique, Orthographic, Perspective, Projection, ViewPort,
    };
    use fabled_transform::Orientation;

    fn initialize_projection_view_matrix(
        translation_target: [f32; 3],
        rotation_target: [f32; 3],
    ) -> Camera {
        // Create camera orientation and update translation and rotation.
        let mut camera_orientation = Orientation::default();
        camera_orientation.update_translation(translation_target);
        camera_orientation.update_rotation(rotation_target);


        // Create a camera matrix and create a view matrix and a projection matrix.
        let mut camera_matrix = Camera::default();

        camera_matrix.calculate_look_at_matrix(camera_orientation);

        let perspective = Perspective::default();
        let projection = Projection::Perspective(perspective);

        camera_matrix.calculate_projection_matrix(projection);


        camera_matrix
    }


    #[test]
    fn calculate_project() {
        // Threshold to determine if passed due to float precision error.
        let threshold = 0.00002;

        // Predefine data such as rotation translation and point target.
        let rotation_target = [
            30.0f32.to_radians(),
            25.0f32.to_radians(),
            160.0f32.to_radians(),
        ];
        let translation_target = [20.0, 1.0, 0.3];
        let point_target = [1.0, 15.0, 20.2];

        let camera_matrix = initialize_projection_view_matrix(rotation_target, translation_target);

        let viewport = ViewPort::default();

        let project_vector = camera_matrix.project(
            point_target,
            glam::Mat4::IDENTITY.to_cols_array(),
            &viewport,
        );

        assert!(!project_vector[0].is_nan());
        assert!(!project_vector[1].is_nan());
        assert!(!project_vector[2].is_nan());

        // Tested in other game engine "project" function
        //         -2.2656322  4.9326377  1.0340623
        let tested_result = [-2.2656322, 4.9326377, 1.0340623];

        assert!((tested_result[0] - project_vector[0]).abs() < threshold);
        assert!((tested_result[1] - project_vector[1]).abs() < threshold);
        assert!((tested_result[2] - project_vector[2]).abs() < threshold);
    }

    #[test]
    fn calculate_unproject() {
        // Threshold to determine if passed due to float precision error.
        let threshold = 0.2;

        // Predefine data such as rotation translation and point target.
        let rotation_target = [
            30.0f32.to_radians(),
            270.0f32.to_radians(),
            160.0f32.to_radians(),
        ];
        let translation_target = [90.0, 1.234, 23.3];
        let point_target = [23.0, 15.333, 20.2];


        let camera_matrix = initialize_projection_view_matrix(rotation_target, translation_target);

        // Create viewport and calculate the project.
        let viewport = ViewPort::default();

        let unprojected_vector = camera_matrix.unproject(
            point_target,
            glam::Mat4::IDENTITY.to_cols_array(),
            &viewport,
        );

        assert!(!unprojected_vector[0].is_nan());
        assert!(!unprojected_vector[1].is_nan());
        assert!(!unprojected_vector[2].is_nan());

        // Tested in other game engine "unproject" function
        //         0.5074327  4.799162  2.5515323
        let tested_result = [0.5074327, 4.799162, 2.5515323];

        assert!((tested_result[0] - unprojected_vector[0]).abs() < threshold);
        assert!((tested_result[1] - unprojected_vector[1]).abs() < threshold);
        assert!((tested_result[2] - unprojected_vector[2]).abs() < threshold);
    }

    #[test]
    fn calculate_look_at_matrix() {
        // Create camera orientation and update translation and rotation.
        let rotation = [
            90.0f32.to_radians(),
            45.0f32.to_radians(),
            130.0f32.to_radians(),
        ];

        let translation = [10.0f32, 20.0f32, 30.0f32];

        let mut camera_orientation = Orientation::default();
        camera_orientation.update_translation(translation);

        camera_orientation.update_rotation(rotation);

        // Create a camera matrix and create a view matrix and a projection matrix.
        let mut camera_matrix = Camera::default();

        camera_matrix.calculate_look_at_matrix(camera_orientation);


        println!("view matrix {:?}", camera_matrix.view);
    }

    #[test]
    fn calculate_arc_ball_matrix() {
        // will test arc ball matrix later. since I have nothing to test it
    }


    #[test]
    fn calculate_inverse_matrix() {
        // Create camera orientation and update translation and rotation.
        let rotation = [
            270.0f32.to_radians(),
            130.0f32.to_radians(),
            185.0f32.to_radians(),
        ];

        let translation = [10.0f32, 20.0f32, 30.0f32];

        let mut camera_matrix = initialize_projection_view_matrix(translation, rotation);


        let inv_view = glam::Mat4::from_cols_array(&camera_matrix.view)
            .inverse()
            .to_cols_array();

        camera_matrix.calculate_inverse_view_matrix();

        assert!(camera_matrix.inv_view.eq(&inv_view));
    }

    #[test]
    fn calculate_projection_matrix() {
        let threshold = 0.0003;

        let translation = [1.0, 23.0, 15.0];
        let rotation = [
            20.0f32.to_radians(),
            15.5f32.to_radians(),
            55.5f32.to_radians(),
        ];

        let camera_matrix = initialize_projection_view_matrix(translation, rotation);


        let proven_projection_matrix = [
            0.9742759, 0.00000, 0.00000, 0.00000, 0.00000, 1.732046, 0.00000, 0.00000, 0.00000,
            0.00000, -1.00020, -1.00000, 0.00000, 0.00000, -0.10001, 0.00000,
        ];

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

        println!("{:?}", camera_matrix.proj);

        assert!((camera_matrix.proj[0] - proven_projection_matrix[0]).abs() < threshold);
        assert!((camera_matrix.proj[1] - proven_projection_matrix[1]).abs() < threshold);
        assert!((camera_matrix.proj[2] - proven_projection_matrix[2]).abs() < threshold);
        assert!((camera_matrix.proj[3] - proven_projection_matrix[3]).abs() < threshold);
        assert!((camera_matrix.proj[4] - proven_projection_matrix[4]).abs() < threshold);
        assert!((camera_matrix.proj[5] - proven_projection_matrix[5]).abs() < threshold);
        assert!((camera_matrix.proj[6] - proven_projection_matrix[6]).abs() < threshold);
        assert!((camera_matrix.proj[7] - proven_projection_matrix[7]).abs() < threshold);
        assert!((camera_matrix.proj[8] - proven_projection_matrix[8]).abs() < threshold);
        assert!((camera_matrix.proj[9] - proven_projection_matrix[9]).abs() < threshold);
        assert!((camera_matrix.proj[10] - proven_projection_matrix[10]).abs() < threshold);
        assert!((camera_matrix.proj[11] - proven_projection_matrix[11]).abs() < threshold);
        assert!((camera_matrix.proj[12] - proven_projection_matrix[12]).abs() < threshold);
        assert!((camera_matrix.proj[13] - proven_projection_matrix[13]).abs() < threshold);
        assert!((camera_matrix.proj[14] - proven_projection_matrix[14]).abs() < threshold);
        assert!((camera_matrix.proj[15] - proven_projection_matrix[15]).abs() < threshold);
    }

    #[test]
    fn calculate_inverse_projection_matrix() {
        // Create camera orientation and update translation and rotation.
        let rotation = [
            270.0f32.to_radians(),
            130.0f32.to_radians(),
            185.0f32.to_radians(),
        ];

        let translation = [10.0f32, 20.0f32, 30.0f32];

        let mut camera_matrix = initialize_projection_view_matrix(translation, rotation);


        let inv_proj = glam::Mat4::from_cols_array(&camera_matrix.proj)
            .inverse()
            .to_cols_array();

        camera_matrix.calculate_inverse_projection_matrix();

        assert!(camera_matrix.inv_proj.eq(&inv_proj));
    }


    #[test]
    fn calculate_oblique_projection_matrix() {
        let rotation = [
            270.0f32.to_radians(),
            130.0f32.to_radians(),
            185.0f32.to_radians(),
        ];

        let translation = [10.0f32, 20.0f32, 30.0f32];


        // Create camera orientation and update translation and rotation.
        let mut camera_orientation = Orientation::default();
        camera_orientation.update_translation(translation);
        camera_orientation.update_rotation(rotation);


        // Create a camera matrix and create a view matrix and a projection matrix.
        let mut camera = Camera::default();

        let error_threshold = 0.001;

        let orthographic_detail = Orthographic {
            right: 8.8889,
            left: -8.8889,
            top: 5.0,
            bottom: -5.0,
            clipping: ClippingPlane {
                far: 0.1,
                near: 1000.0,
            },
        };

        let oblique_detail = Oblique {
            angle_rad: 64.0f32.to_radians(),
            vertical_position: 0.055,
            depth_offset: 8.64,
        };

        camera.calculate_oblique_projection_matrix(orthographic_detail, oblique_detail);

        let proven_oblique_matrix = [
            0.11250, 0.00000, -0.00989, 0.08542, // 0
            0.00000, 0.20000, 0.00482,
            -0.04166, // 1 swap -0.0482 and -0.04166 to positive since we are rhs
            0.00000, 0.00000, 0.00200, 0.00000, 0.00000, 0.00000, 1.00000, 1.00000,
        ];

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

        assert!((proven_oblique_matrix[0] - camera.proj[0]).abs() < error_threshold);
        assert!((proven_oblique_matrix[1] - camera.proj[1]).abs() < error_threshold);
        assert!((proven_oblique_matrix[2] - camera.proj[2]).abs() < error_threshold);
        assert!((proven_oblique_matrix[3] - camera.proj[3]).abs() < error_threshold);
        assert!((proven_oblique_matrix[4] - camera.proj[4]).abs() < error_threshold);
        assert!((proven_oblique_matrix[5] - camera.proj[5]).abs() < error_threshold);
        assert!((proven_oblique_matrix[6] - camera.proj[6]).abs() < error_threshold);
        assert!((proven_oblique_matrix[7] - camera.proj[7]).abs() < error_threshold);
        assert!((proven_oblique_matrix[8] - camera.proj[8]).abs() < error_threshold);
        assert!((proven_oblique_matrix[9] - camera.proj[9]).abs() < error_threshold);
        assert!((proven_oblique_matrix[10] - camera.proj[10]).abs() < error_threshold);
        assert!((proven_oblique_matrix[11] - camera.proj[11]).abs() < error_threshold);
        assert!((proven_oblique_matrix[12] - camera.proj[12]).abs() < error_threshold);
        assert!((proven_oblique_matrix[13] - camera.proj[13]).abs() < error_threshold);
        assert!((proven_oblique_matrix[14] - camera.proj[14]).abs() < error_threshold);
        assert!((proven_oblique_matrix[15] - camera.proj[15]).abs() < error_threshold);


        println!("oblique projection \n{:?}", camera.proj);
    }


    #[test]
    fn convert_test() {
        let error_threshold = 0.2;

        let rotation = [
            270.0f32.to_radians(),
            130.0f32.to_radians(),
            185.0f32.to_radians(),
        ];

        let translation = [10.0f32, 20.0f32, 30.0f32];


        let camera = initialize_projection_view_matrix(translation, rotation);

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

        let start_unproject = camera.unproject(
            starting_value,
            glam::Mat4::IDENTITY.to_cols_array(),
            &viewport,
        );
        println!("unproject value is {:?}", start_unproject);

        let start_project = camera.project(
            start_unproject,
            glam::Mat4::IDENTITY.to_cols_array(),
            &viewport,
        );
        println!("project value is {:?}", start_project);

        let end_unproject = camera.unproject(
            start_project,
            glam::Mat4::IDENTITY.to_cols_array(),
            &viewport,
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
