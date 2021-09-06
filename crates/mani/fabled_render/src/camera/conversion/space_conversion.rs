use crate::camera::{Camera, Orientation, ViewPort};

// todo need to find a way to get the model matrix or the orientation to
// construct a model matrix from. we need to have access to the camera most
// recent orientation in the space

impl Camera {
    // depth value will 0.1 unless developer know what they are doing.
    pub fn screen_to_world(
        &self,
        target_position: [f32; 3],
        depth: f32,
        orientation: Orientation,
        viewport: &ViewPort,
    ) -> [f32; 3] {
        assert!(depth >= 0.0);
        assert!(depth <= 1.0);

        let test_model = glam::Mat4::IDENTITY.to_cols_array();

        let result = self.unproject(
            [target_position[0], viewport.h - target_position[1], depth],
            test_model,
            viewport,
        );

        [
            result[0] * target_position[2],
            result[1] * target_position[2],
            result[2] * target_position[2],
        ]
    }

    pub fn screen_to_view(&self, target_position: [f32; 3]) -> [f32; 3] {
        // we need the aspect ratio. we will get it by projection.11 / projection.00;

        // self.project()

        todo!()
    }

    pub fn view_to_screen(&self) -> [f32; 3] {
        // we need the aspect ratio. we will get it by projection.11 / projection.00;

        todo!()
    }

    // need viewport.
    pub fn view_to_world(&self, view_position: [f32; 3]) -> [f32; 3] {
        // we need the aspect ratio. we will get it by projection.11 / projection.00;

        todo!()
    }

    pub fn world_to_screen(
        &self,
        target_position: [f32; 3],
        orientation: Orientation,
        viewport: &ViewPort,
    ) -> [f32; 3] {
        // we need the aspect ratio. we will get it by projection.11 / projection.00;

        let test_model = glam::Mat4::IDENTITY.to_cols_array();

        let result = self.project(
            [target_position[0], target_position[1], 0.1],
            test_model,
            viewport,
        );

        println!("result is {:?}", result);
    }

    pub fn world_to_view(&self) -> [f32; 3] {
        // we need the aspect ratio. we will get it by projection.11 / projection.00;

        todo!()
    }
}


#[cfg(test)]
mod world_conversion_test {
    use crate::camera::{
        AspectRatio, Camera, ClippingPlane, Fov, FovAxis, Orientation, Perspective, Projection,
        ProjectionCoordinate, ViewPort,
    };

    fn initialize_test(orientation: Orientation) -> Camera {
        let mut camera = Camera::default();

        camera.calculate_look_at_matrix(orientation, ProjectionCoordinate::RightHandCoordinate);

        camera.calculate_projection_matrix(Projection::Perspective(
            Perspective {
                fov: Fov {
                    radian: 100.0f32.to_radians(),
                    axis: FovAxis::Vertical,
                },
                aspect: AspectRatio {
                    horizontal: 3840.0,
                    vertical: 2160.0,
                },
                clipping: ClippingPlane {
                    far: 0.1,
                    near: 1000.0,
                },
            },
            None,
        ));
        camera
    }


    #[test]
    fn screen_to_world() {
        let orientation = Orientation::default();

        let camera = initialize_test(orientation);

        let viewport = ViewPort {
            x: 0.0,
            y: 0.0,
            w: 3840.0,
            h: 2160.0,
            min_depth: 0.1,
            max_depth: 1000.0,
        };

        let screen_to_world_point =
            camera.screen_to_world([584.307_8, 197.837_28, 32.0], 0.1, orientation, &viewport);

        println!("{:?}", screen_to_world_point)
    }


    #[test]
    fn world_to_screen() {
        let orientation = Orientation::default();

        let camera = initialize_test(orientation);

        let viewport = ViewPort {
            x: 0.0,
            y: 0.0,
            w: 3840.0,
            h: 2160.0,
            min_depth: 0.1,
            max_depth: 1000.0,
        };

        let world_to_screen =
            camera.world_to_screen([47.1225, -31.122223, 31.97122], orientation, &viewport);

        println!("{:?}", world_to_screen);
    }
}
