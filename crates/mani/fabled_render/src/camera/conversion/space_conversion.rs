use crate::camera::{Camera, Orientation, ViewPort};

// todo need to find a way to get the model matrix or the orientation to
// construct a model matrix from. we need to have access to the camera most
// recent orientation in the space

impl Camera {
    // Orientation is the same as transform and is needed. All entity will probably
    // have an orientation. So Orientation will be moved out of camera and into
    // a separate module maybe fabled_transform.
    pub fn screen_to_world(
        &self,
        target_position: [f32; 3],
        orientation: Orientation,
        viewport: &ViewPort,
    ) -> [f32; 3] {
        // We need the model matrix and the viewport width and height for the resolution
        // of the screen. self.unproject(view_position, [0.0; 3])

        let test_model = glam::Mat4::IDENTITY.to_cols_array();

        let result = self.unproject(
            [
                target_position[0],
                viewport.h - target_position[1],
                target_position[2],
            ],
            test_model,
            viewport,
        );
        result
    }

    pub fn screen_to_view(&self) -> [f32; 3] {
        // we need the aspect ratio. we will get it by projection.11 / projection.00;


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

    pub fn world_to_screen(&self) -> [f32; 3] {
        // we need the aspect ratio. we will get it by projection.11 / projection.00;

        todo!()
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

    #[test]
    fn screen_to_world() {
        let orientation = Orientation::default();
        let mut camera = Camera::default();

        camera.calculate_look_at_matrix(orientation, ProjectionCoordinate::RightHandCoordinate);

        camera.calculate_projection_matrix(Projection::Perspective(
            Perspective {
                fov: Fov {
                    radian: 60.0f32.to_radians(),
                    axis: FovAxis::Vertical,
                },
                aspect: AspectRatio {
                    horizontal: 1920.0,
                    vertical: 1080.0,
                },
                clipping: ClippingPlane {
                    far: 0.1,
                    near: 1000.0,
                },
            },
            None,
        ));

        let viewport = ViewPort {
            x: 0.0,
            y: 0.0,
            w: 1920.0,
            h: 1080.0,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let a = camera.screen_to_world([25.0, 3.3, 10.11], orientation, &viewport);

        println!("{:?}", a)
    }
}
