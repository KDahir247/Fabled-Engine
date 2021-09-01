#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0; 3], // Origin
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
        }
    }
}

impl Transform {
    pub fn new(position: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    #[rustfmt::skip]
    pub fn get_rotation_matrix(&self) -> [f32; 9] {

        let x = self.rotation[0];
        let y = self.rotation[1];
        let z = self.rotation[2];
        let w = self.rotation[3];

        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;

        [
            1.0 - 2.0 * (y2 + z2), 2.0 * (xy + wz), 2.0 * (xz - wy),//col 0
            2.0 * (xy - wz), 1.0 - 2.0 * (x2 + z2), 2.0 * (yz + wx),//col 1
            2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (x2 + y2)//col 2
        ]
    }

    pub fn get_transformation_matrix(&self) -> [f32; 16] {
        let rotation = glam::Quat::from_array(self.rotation);
        let scale = glam::Vec3::from(self.scale);
        let translation = glam::Vec3::from(self.position);

        let transformation_matrix =
            glam::Mat4::from_scale_rotation_translation(scale, rotation, translation);

        transformation_matrix.to_cols_array()
    }
}
