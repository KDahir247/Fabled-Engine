use lib::component::prelude::*;

pub fn run() {
    //println!("{}", std::mem::size_of::<CameraController>());

    //todo switch independent position, rotation, and scale to be a Mat4
    //scale, quaternion, translation matrix.

    let a = glam::Mat4::from_scale_rotation_translation(
        glam::vec3(1.0, 1.0, 1.0),
        glam::Quat::from_euler(glam::EulerRot::XYZ, 5.0_f32.to_radians(), 0.0, 0.0),
        glam::Vec3::ZERO,
    );

    a.transform_point3(glam::Vec3::ONE);
    let b = a.to_scale_rotation_translation();

    /*   println!(
        "scale {}, quat {:?}, position {}",
        b.0,
        b.1.to_euler(glam::EulerRot::XYZ).0 * 180.0f32 / std::f32::consts::PI,
        b.2
    );*/
}
