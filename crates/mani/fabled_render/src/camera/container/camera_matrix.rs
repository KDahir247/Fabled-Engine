use crate::camera::{Orientation, Projection};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraMatrix {
    pub view: [f32; 16],
    pub proj: [f32; 16],
    pub inv_proj: [f32; 16],
    pub inv_view: [f32; 16],
}

impl CameraMatrix {
    pub fn calculate_view_matrix(&mut self, orientation: Orientation) {
        let Orientation {
            transformation_matrix,
            forward,
            ..
        } = orientation;

        let position = glam::Mat4::from_cols_array(&transformation_matrix)
            .w_axis
            .to_array();

        let position = glam::Vec3A::from_slice(&position[0..3]);

        let forward_slice = &forward[0..3];

        let f = glam::Vec3A::from_slice(forward_slice).normalize();
        let s = f.cross(glam::Vec3A::Y).normalize();
        let u = s.cross(f);

        let w = glam::Vec4::new(-position.dot(s), -position.dot(u), position.dot(f), 1.0);

        let view_matrix = [
            s.x, u.x, -f.x, 0.0, //0
            s.y, u.y, -f.y, 0.0, //1
            s.z, u.z, -f.z, 0.0, //2
            w.x, w.y, w.z, w.w, //3
        ];

        self.view = view_matrix;
    }

    pub fn calculate_projection_matrix(&mut self, projection: Projection) {
        //
    }
}

//    pub view_position: [f32; 4], //transformation matrix orientation .xyz.extend(1.0)

#[cfg(test)]
mod camera_matrix_test {
    use crate::camera::{CameraMatrix, Orientation};

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

        CameraMatrix::default().calculate_view_matrix(Orientation::default());
    }
}
