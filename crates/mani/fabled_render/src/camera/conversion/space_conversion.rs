use crate::camera::{Camera, Orientation, ViewPort};

// todo need to find a way to get the model matrix or the orientation to
//  construct a model matrix from. we need to have access to the camera most
//  recent orientation in the space

impl Camera {
    pub fn screen_to_world(
        &self,
        target_position: [f32; 3],
        orientation: Orientation,
        viewport: &ViewPort,
    ) -> [f32; 3] {
        let test_model = glam::Mat4::IDENTITY.to_cols_array();

        let depth = viewport.max_depth * viewport.min_depth;

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
        todo!()
    }

    pub fn view_to_screen(&self) -> [f32; 3] {
        todo!()
    }

    pub fn view_to_world(&self, view_position: [f32; 3]) -> [f32; 3] {
        todo!()
    }

    pub fn world_to_screen(
        &self,
        target_position: [f32; 3],
        orientation: Orientation,
        viewport: &ViewPort,
    ) -> [f32; 3] {
        let test_model = glam::Mat4::IDENTITY.to_cols_array();

        let result = self.project(
            [target_position[0], -target_position[1], target_position[2]],
            test_model,
            viewport,
        );

        [result[0], result[1], target_position[2]]
    }

    pub fn world_to_view(&self) -> [f32; 3] {
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

    // ---------------------- Screen To World ---------------------------

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
            camera.screen_to_world([584.307_8, 197.837_28, 0.5], orientation, &viewport);
        println!("{:?}", screen_to_world_point)
    }

    // ---------------------- World To Screen ---------------------------

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
            camera.world_to_screen([0.7362891, 0.48628473, 0.49955022], orientation, &viewport);

        println!("{:?}", world_to_screen);


        let screen_to_world =
            camera.screen_to_world([584.30786, 1962.1627, 0.4995503], orientation, &viewport);

        println!("{:?}", screen_to_world);
    }

    //

    #[test]
    fn convert_test() {
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

        let starting_value = [25.333, 12.27, 0.1];

        println!("starting value is {:?}", starting_value);

        let b = camera.unproject(
            starting_value,
            glam::Mat4::IDENTITY.to_cols_array(),
            &viewport,
        );
        println!("unproject value is {:?}", b);

        let a = camera.project(b, glam::Mat4::IDENTITY.to_cols_array(), &viewport);
        println!("project value is {:?}", a);

        let c = camera.unproject(a, glam::Mat4::IDENTITY.to_cols_array(), &viewport);

        println!("unproject value is {:?}", c);
    }
}
