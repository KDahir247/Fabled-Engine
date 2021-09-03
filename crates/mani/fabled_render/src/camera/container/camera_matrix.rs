use crate::camera::{
    Orientation, Orthographic, PerspectiveDistance, PerspectiveOrientation, Projection,
    ProjectionCoordinate, ViewPort, YAxis,
};

use glam::Vec4Swizzles;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraMatrix {
    pub view: [f32; 16],
    pub proj: [f32; 16],
    pub inv_proj: [f32; 16],
    pub inv_view: [f32; 16],
}

impl CameraMatrix {
    /// The target position is transformed in the following spaces to reach screen space.
    /// 1. From model space to world space by the world matrix,
    /// 2. From world space to view space from the view matrix.
    /// 3. from view space to screen space from the projection matrix.
    pub fn project(&self, target: [f32; 3], model: [f32; 16], viewport: &ViewPort) -> [f32; 3] {
        let model_representation = glam::Mat4::from_cols_array(&model);
        let projection_representation = glam::Mat4::from_cols_array(&self.proj);
        let view_representation = glam::Mat4::from_cols_array(&self.view);

        let vector = model_representation
            * view_representation
            * projection_representation
            * glam::Vec4::new(target[0], target[1], target[2], 1.0);

        let normalized_factor = 1.0 / vector.w;

        let project = glam::Vec3::new(
            viewport.x + (viewport.w * ((vector.x * normalized_factor + 1.0) * 0.5)),
            viewport.y + (viewport.h * ((-vector.y * normalized_factor + 1.0) * 0.5)),
            vector.z * normalized_factor,
        );

        project.to_array()
    }

    /// The target position is transformed in the following space to reach model space.
    /// 1. From screen space to view space by the inverse projection matrix.
    /// 2. From view space to world space by the inverse view matrix.
    /// 3. From world space to model space by the inverse of the world matrix.
    pub fn unproject(&self, target: [f32; 3], model: [f32; 16], viewport: &ViewPort) -> [f32; 3] {
        let model_representation = glam::Mat4::from_cols_array(&model);
        let projection_representation = glam::Mat4::from_cols_array(&self.proj);
        let view_representation = glam::Mat4::from_cols_array(&self.view);

        let matrix =
            (projection_representation * (model_representation * view_representation)).inverse();

        let vector = glam::vec4(
            2.0 * (target[0] - viewport.x) / viewport.w - 1.0,
            2.0 * (target[1] - viewport.y) / viewport.h - 1.0,
            target[2],
            1.0,
        );

        let result = matrix * vector;

        (result.xyz() / result.w).to_array()
    }

    pub fn calculate_look_at_matrix(
        &mut self,
        orientation: Orientation,
        coordinate_direction: ProjectionCoordinate,
    ) {
        let Orientation {
            transform, forward, ..
        } = orientation;

        let coordinate_direction = coordinate_direction as i32;

        let transformation_matrix = transform.get_transformation_matrix();

        let position = glam::Mat4::from_cols_array(&transformation_matrix)
            .w_axis
            .to_array();

        let position = glam::Vec3A::from_slice(&position[0..3]);

        let forward_slice = &forward[0..3];

        let z_axis =
            glam::Vec3A::from_slice(forward_slice).normalize() * coordinate_direction as f32;
        let x_axis = glam::Vec3A::Y.cross(z_axis).normalize();
        let y_axis = z_axis.cross(x_axis);

        let w = glam::Vec3A::new(
            -position.dot(x_axis),
            -position.dot(y_axis),
            -position.dot(z_axis),
        );

        let view_matrix = [
            x_axis.x, y_axis.x, z_axis.x, 0.0, //0
            x_axis.y, y_axis.y, z_axis.y, 0.0, //1
            x_axis.z, y_axis.z, z_axis.z, 0.0, //2
            w.x, w.y, w.z, 1.0, //3
        ];

        self.view = view_matrix;
    }

    //todo not tested.
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

    pub fn calculate_view_matrix_fast(&mut self, orientation: Orientation) {
        let transformation_matrix = orientation.transform.get_transformation_matrix();

        let mat4_transformation_representation =
            glam::Mat4::from_cols_array(&transformation_matrix);

        self.view = mat4_transformation_representation.inverse().to_cols_array();
    }

    pub fn calculate_inverse_view_matrix(&mut self) {
        let mat4_view_representation = glam::Mat4::from_cols_array(&self.view);
        let inverse_view_matrix = mat4_view_representation.inverse();
        self.inv_view = inverse_view_matrix.to_cols_array()
    }


    #[rustfmt::skip]
    /// [Orthographic Matrix Reference](https://en.wikipedia.org/wiki/Orthographic_projection) <br/>
    /// [DirectX/WEBGPU Implementation](https://blog.demofox.org/2017/03/31/orthogonal-projection-matrix-plainly-explained/) <br/>
    /// [DirectX Documentation](https://docs.microsoft.com/en-us/windows/win32/direct3d9/dx9-graphics)
    pub fn calculate_projection_matrix(&mut self, projection: Projection) {
        let mut direction = ProjectionCoordinate::default();
        let mut y_axis = YAxis::default();

        match projection {
            Projection::Orthographic(orthographic, orthographic_option) => {
                if let Some(option) = orthographic_option {
                    direction = option.direction;
                    y_axis = option.y_axis;
                }

                let Orthographic{
                    right, left, top, bottom, clipping
                } = orthographic;

                let right_min_left = right - left;
                let right_plus_left = right + left;
                let top_min_bottom = top - bottom;
                let top_plus_bottom = top + bottom;
                let far_min_near = clipping.far - clipping.near;

                let dir = direction as i32;
                let axis =  y_axis as i32;

                let y_direction = 0.5 * axis as f32;

                let coordinate_direction = dir as f32;

                let orthographic_matrix =
                    [
                        0.5 * right_min_left, 0.0,  0.0, 0.0,
                        0.0, y_direction * top_min_bottom, 0.0, 0.0,
                        0.0, 0.0, coordinate_direction / far_min_near, 0.0,
                        -(right_plus_left/ right_min_left), -(top_plus_bottom/ top_min_bottom), -(clipping.near / far_min_near), 1.0
                    ];


                self.proj = orthographic_matrix;
            }
            Projection::Perspective(perspective, perspective_option) => {

                let mut distance = PerspectiveDistance::default();
                let mut orientation = PerspectiveOrientation::default();

                if let Some(option) = perspective_option {
                    direction = option.direction;
                    y_axis = option.y_axis;
                    distance = option.distance;
                    orientation = option.orientation;
                }

                let y_axis = y_axis as i32;

                let h = 1.0 / (perspective.fovy.radian * 0.5);
                
                let w  = h / (perspective.aspect.horizontal / perspective.aspect.vertical);
                let near_min_far = perspective.clipping.near - perspective.clipping.far;

                let direction = direction as i32;

                let mut c3r3 = perspective.clipping.far / near_min_far;
                let mut c4r3 = perspective.clipping.near * perspective.clipping.near / near_min_far;

                match distance{
                    PerspectiveDistance::Standard => {
                        if orientation == PerspectiveOrientation::Reversed {
                            c3r3 = -perspective.clipping.far / near_min_far -1.0;
                            c4r3 = -perspective.clipping.near * perspective.clipping.far / near_min_far
                        }
                    }
                    PerspectiveDistance::Infinite => {
                        c3r3 = -1.0;
                        c4r3 = -perspective.clipping.near;

                        if orientation == PerspectiveOrientation::Reversed{
                            c3r3 = 0.0;
                            c4r3 = perspective.clipping.near
                        }

                    }
                }

                let projection_matrix =
                    [
                        w, 0.0, 0.0, 0.0,
                        0.0, h * y_axis as f32, 0.0, 0.0,
                        0.0, 0.0, c3r3, direction as f32,
                        0.0, 0.0, c4r3, 0.0,
                        //
                    ];


                self.proj = projection_matrix;

            }
        }
    }

    pub fn calculate_inverse_projection_matrix(&mut self) {
        let mat4_projection_representation = glam::Mat4::from_cols_array(&self.proj);
        let inverse_view_matrix = mat4_projection_representation.inverse();
        self.inv_proj = inverse_view_matrix.to_cols_array()
    }
}

#[cfg(test)]
mod camera_matrix_test {}
