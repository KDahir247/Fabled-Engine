use crate::FbxLoadError;
use fabled_render::camera::{
    focal_length_to_directional_fov, focal_length_to_fov, fov_to_focal_length, Aperture,
    ApertureMode, FishLens, FovAxis, GateFit, Projection,
};

// What we need.
// Projection, Sensor Type, Aperture (x, y), ISO, Shutter, Gate, Focal length,
// Aperture

// This function will be responsible only for loading the fbx camera data.
pub fn load_camera_handle(
    obj_handle: &fbxcel_dom::v7400::object::ObjectHandle,
) -> Result<(), FbxLoadError> {
    if let fbxcel_dom::v7400::object::TypedObjectHandle::NodeAttribute(
        fbxcel_dom::v7400::object::nodeattribute::TypedNodeAttributeHandle::Camera(camera_handle),
    ) = obj_handle.get_typed()
    {
        let camera_handle = crate::CameraProperties::new(&camera_handle).ok_or(
            FbxLoadError::FBXPropertiesError("failed to retrieve camera properties"),
        )?;

        let film_width = camera_handle.film_width_or_default().unwrap_or(0.979_921_3);
        let film_height = camera_handle.film_height_or_default().unwrap_or(0.73464567);

        let aperture_x_mm = fabled_render::camera::inch_to_millimeter(film_width);
        let aperture_y_mm = fabled_render::camera::inch_to_millimeter(film_height);

        let aperture_mode = camera_handle
            .aperture_mode_or_default()
            .unwrap_or(ApertureMode::Vertical);


        let focal_length = camera_handle.focal_length_or_default().unwrap_or(21.0);

        let focus_distance = camera_handle.focus_distance_or_default().ok();

        let aperture = Aperture::new(aperture_x_mm, aperture_y_mm);

        let iso_speed = fabled_render::camera::ISOSpeed::new_standard(
            400.0,
            fabled_render::camera::ISOSpeedUnit::Arithmetic,
        );

        let f_stop = fabled_render::camera::FStop::FullStop(fabled_render::camera::FStop::F4_STOP);

        let shutter_speed = fabled_render::camera::Shutter::compute_shutter_speed(f_stop);

        let gate_fit = camera_handle.gate_fit_or_default().unwrap_or(GateFit::None);

        let camera_near_plane = camera_handle.near_plane_or_default().unwrap_or(0.01);
        let camera_far_plane = camera_handle.far_plane_or_default().unwrap_or(1000.0);

        let camera_clipping_plane =
            fabled_render::camera::ClippingPlane::new(camera_far_plane, camera_near_plane);

        let projection = camera_handle.camera_projection_or_default().map_err(|_| {
            FbxLoadError::FBXPropertiesError("failed to retrieve camera projection property")
        })?;

        match projection {
            Projection::Orthographic(mut orthographic) => {
                let orthographic_zoom = camera_handle.ortho_zoom_or_default().unwrap_or(1.0);

                orthographic.clipping = camera_clipping_plane;

                orthographic.left = -orthographic_zoom;
                orthographic.right = orthographic_zoom;

                orthographic.top = orthographic_zoom;
                orthographic.bottom = orthographic_zoom;
            }
            Projection::Perspective(mut perspective) => {
                let vertical_field_of_view = camera_handle
                    .field_of_view_Y_or_default()
                    .unwrap_or(77.319_62)
                    .to_radians();


                let aspect_width = camera_handle.aspect_width_or_default().unwrap_or(16.0);
                let aspect_height = camera_handle.aspect_height_or_default().unwrap_or(9.0);

                perspective.clipping = camera_clipping_plane;


                let magnification = focus_distance
                    .map(|focus_dist| {
                        fabled_render::camera::compute_approx_magnification(
                            focal_length,
                            focus_dist,
                        )
                    })
                    .unwrap_or(1.0);

                let fov_detail = match aperture_mode {
                    ApertureMode::HorizontalAndVertical => {
                        let (diagonal_fov, magnification) = focal_length_to_directional_fov(
                            focal_length,
                            [aperture_x_mm, aperture_y_mm],
                            focus_distance,
                            None,
                            FishLens::Rectilinear,
                        );

                        diagonal_fov
                    }
                    ApertureMode::Horizontal => {
                        let (horizontal_fov, magnification) = focal_length_to_fov(
                            focal_length,
                            aperture_x_mm,
                            FovAxis::Horizontal,
                            focus_distance,
                            None,
                            FishLens::Rectilinear,
                        );

                        horizontal_fov.radian
                    }
                    ApertureMode::Vertical => {
                        let (vertical_fov, magnification) = focal_length_to_fov(
                            focal_length,
                            aperture_y_mm,
                            FovAxis::Vertical,
                            focus_distance,
                            None,
                            FishLens::Rectilinear,
                        );

                        vertical_fov.radian
                    }
                    ApertureMode::FocalLength => fov_to_focal_length(
                        vertical_field_of_view,
                        [aperture.aperture_size_x, aperture.aperture_size_y],
                        None,
                        magnification,
                        FishLens::Rectilinear,
                    ),
                };

                perspective.fov = fabled_render::camera::Fov::new(
                    vertical_field_of_view,
                    fabled_render::camera::FovAxis::Vertical,
                );

                perspective.aspect = fabled_render::camera::AspectRatio {
                    horizontal: aspect_width,
                    vertical: aspect_height,
                };
            }
        }
    }
    Ok(())
}
