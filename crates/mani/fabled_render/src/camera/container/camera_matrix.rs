use crate::camera::{
    Orientation, Orthographic, PerspectiveDistance, PerspectiveOrientation, Projection,
    ProjectionCoordinate, YAxis,
};

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
    pub fn project(&self, target: [f32; 3]) -> [f32; 3] {
        //It will need the model matrix, projection matrix and view matrix.
        todo!()
    }

    /// The target position is transformed in the following space to reach model space.
    /// 1. From screen space to view space by the inverse projection matrix.
    /// 2. From view space to world space by the inverse view matrix.
    /// 3. From world space to model space by the inverse of the world matrix.
    pub fn unproject(&self, target: [f32; 3]) -> [f32; 3] {
        //It will need the the model matrix, projection matrix and the view matrix.
        todo!()
    }

    pub fn calculate_view_matrix(
        &mut self,
        orientation: Orientation,
        coordinate_direction: ProjectionCoordinate,
    ) {
        let Orientation {
            transformation_matrix,
            forward,
            ..
        } = orientation;

        let coordinate_direction = coordinate_direction as i32;

        // We are getting the last column of the transformation matrix to retrieve the translation.
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

        // Column Major matrix
        let view_matrix = [
            x_axis.x, y_axis.x, z_axis.x, 0.0, //0
            x_axis.y, y_axis.y, z_axis.y, 0.0, //1
            x_axis.z, y_axis.z, z_axis.z, 0.0, //2
            w.x, w.y, w.z, 1.0, //3
        ];

        self.view = view_matrix;
    }

    pub fn calculate_inverse_view_matrix(&mut self) {
        let mat4_representation = glam::Mat4::from_cols_array(&self.view);
        let inverse_view_matrix = mat4_representation.inverse();
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

                let Orthographic {
                    right,
                    left, top,
                    bottom, z_near,
                    z_far,
                } = orthographic;

                let right_min_left = right - left;
                let right_plus_left = right + left;
                let top_min_bottom = top - bottom;
                let top_plus_bottom = top + bottom;
                let far_min_near = z_far - z_near;

                let dir = direction as i32;
                let axis =  y_axis as i32;

                let y_direction = 0.5 * axis as f32;

                // Coordinate direction is range from -1 to 1 because we are following DirectX matrix and not OpenGL
                // Since OpenGL clip space for z is -1, to 1 while DirectX is 0 to 1
                let coordinate_direction = dir as f32;

                // Column Major matrix
                let orthographic_matrix =
                    [
                        0.5 * right_min_left, 0.0,  0.0, 0.0,
                        0.0, y_direction * top_min_bottom, 0.0, 0.0,
                        0.0, 0.0, coordinate_direction / far_min_near, 0.0,
                        -(right_plus_left/ right_min_left), -(top_plus_bottom/ top_min_bottom), -(z_near / far_min_near), 1.0
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

                //1/tan(x) == cot(x)
                // h = cot(fovY/2)
                let h = 1.0 / (perspective.fovy * 0.5);

                // w = h / aspect ratio
                let w  = h / perspective.aspect;
                let near_min_far = perspective.z_near - perspective.z_far;

                let direction = direction as i32;

                // Matrix Formula specified in microsoft's directx documentation.
                let mut c3r3 = perspective.z_far / near_min_far;
                let mut c4r3 = perspective.z_near * perspective.z_far / near_min_far;

                match distance{
                    PerspectiveDistance::Standard => {
                        if orientation == PerspectiveOrientation::Reversed {
                            c3r3 = -perspective.z_far / near_min_far -1.0;
                            c4r3 = -perspective.z_near * perspective.z_far / near_min_far
                        }
                    }
                    PerspectiveDistance::Infinite => {
                        c3r3 = -1.0;
                        c4r3 = -perspective.z_near;

                        if orientation == PerspectiveOrientation::Reversed{
                            c3r3 = 0.0;
                            c4r3 = perspective.z_near
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
}

//    pub view_position: [f32; 4], //transformation matrix orientation .xyz.extend(1.0)

#[cfg(test)]
mod camera_matrix_test {
    use crate::camera::{CameraMatrix, Orientation, ProjectionCoordinate};

    #[test]
    fn linear_to_glam_mat4_test() {
        let linear_matrix = [
            1., 2., 3., 4., //1
            5., 6., 7., 8., // 2
            9., 10., 11., 12., // 3
            13., 14., 15., 16.,
        ];

        let glam_rep = glam::Mat4::from_cols_array(&linear_matrix);

        println!("{:#?}", glam_rep);

        let glam = glam::mat4(
            glam::vec4(2., 4., 6., 8.),
            glam::vec4(10., 12., 14., 16.),
            glam::vec4(18., 20., 22., 24.),
            glam::vec4(26., 28., 30., 32.),
        );

        println!("{:?}", glam.to_cols_array());

        CameraMatrix::default()
            .calculate_view_matrix(Orientation::default(), ProjectionCoordinate::default());
    }
}
