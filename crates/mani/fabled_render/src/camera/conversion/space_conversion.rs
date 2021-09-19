// use crate::camera::{Camera, ViewPort};
// use fabled_transform::Orientation;
//
// impl Camera {
// pub fn screen_to_world(
// &self,
// target_position: [f32; 3],
// orientation: Orientation,
// viewport: &ViewPort,
// ) -> [f32; 3] {
// let test_model = glam::Mat4::IDENTITY.to_cols_array();
//
// let depth = viewport.max_depth * viewport.min_depth;
//
// let result = self.unproject(
// [target_position[0], viewport.h - target_position[1], depth],
// test_model,
// viewport,
// );
//
// [
// result[0] * target_position[2],
// result[1] * target_position[2],
// result[2] * target_position[2],
// ]
// }
//
// pub fn screen_to_view(&self, target_position: [f32; 3]) -> [f32; 3] {
// todo!()
// }
//
// pub fn view_to_screen(&self) -> [f32; 3] {
// todo!()
// }
//
// pub fn view_to_world(
// &self,
// target_position: [f32; 3],
// orientation: Orientation,
// viewport: &ViewPort,
// ) -> [f32; 3] {
// todo!();
// let test_model = glam::Mat4::IDENTITY.to_cols_array();
//
// let depth = viewport.max_depth * viewport.min_depth;
//
// let result = self.unproject(target_position, test_model, viewport);
//
// [
// result[0] / viewport.w,
// result[1] / viewport.h,
// target_position[2],
// ]
// }
//
// pub fn world_to_screen(
// &self,
// target_position: [f32; 3],
// orientation: Orientation,
// viewport: &ViewPort,
// ) -> [f32; 3] {
// let test_model = glam::Mat4::IDENTITY.to_cols_array();
//
// let result = self.project(
// [target_position[0], -target_position[1], target_position[2]],
// test_model,
// viewport,
// );
//
// [result[0], result[1], target_position[2]]
// }
//
// pub fn world_to_view(
// &self,
// target_position: [f32; 3],
// orientation: Orientation,
// viewport: &ViewPort,
// ) -> [f32; 3] {
// let target_position = [target_position[0], -target_position[1],
// target_position[2]];
//
// let test_model = glam::Mat4::IDENTITY.to_cols_array();
//
//
// let vector = self.project(target_position, test_model, viewport);
//
// [
// vector[0] / viewport.w, // x
// vector[1] / viewport.h, // z
// target_position[2],
// ]
// }
// }
//
//
// #[cfg(test)]
// mod world_conversion_test {
// use fabled_transform::Orientation;
//
// use crate::camera::{
// AspectRatio, Camera, ClippingPlane, Fov, FovAxis, Perspective, Projection,
// ViewPort, };
//
// fn initialize_test(orientation: Orientation) -> Camera {
// let mut camera = Camera::default();
//
// camera.calculate_look_at_matrix(orientation);
//
// camera.calculate_projection_matrix(Projection::Perspective(Perspective {
// fov: Fov {
// radian: 100.0f32.to_radians(),
// axis: FovAxis::Vertical,
// },
// aspect: AspectRatio {
// horizontal: 3840.0,
// vertical: 2160.0,
// },
// clipping: ClippingPlane {
// far: 0.1,
// near: 1000.0,
// },
// }));
// camera
// }
//
// -------------- Screen To World ------- World To Screen ----------------------
//
// #[test]
// fn world_screen() {
// let world_point = [386.24023, 626.484, 496.431_12];
//
// let error_threshold = 0.3;
//
// let orientation = Orientation::default();
//
// let camera = initialize_test(orientation);
//
// let viewport = ViewPort {
// x: 0.0,
// y: 0.0,
// w: 3840.0,
// h: 2160.0,
// min_depth: 0.1,
// max_depth: 1000.0,
// };
//
// println!("Starting world point {:?}", world_point);
//
// let screen_point = camera.world_to_screen(world_point, orientation,
// &viewport);
//
// println!("World to screen point : {:?}", screen_point);
//
// let result_world_point = camera.screen_to_world(screen_point, orientation,
// &viewport);
//
// println!("result screen to world point : {:?}", result_world_point);
//
// let result_screen_point =
// camera.world_to_screen(result_world_point, orientation, &viewport);
//
// println!("result world to screen point : {:?}", result_screen_point);
//
//
// assert!((world_point[0] - result_world_point[0]).abs() < error_threshold);
// assert!((world_point[1] - result_world_point[1]).abs() < error_threshold);
// assert!((world_point[2] - result_world_point[2]).abs() < error_threshold);
//
// assert!((screen_point[0] - result_screen_point[0]).abs() < error_threshold);
// assert!((screen_point[1] - result_screen_point[1]).abs() < error_threshold);
// assert!((screen_point[2] - result_screen_point[2]).abs() < error_threshold);
// }
//
// ------------ Viewport To World ------- World To Viewport --------------------
//
// #[test]
// fn viewport_world() {
// let orientation = Orientation::default();
//
// let camera = initialize_test(orientation);
//
// let viewport = ViewPort {
// x: 0.0,
// y: 0.0,
// w: 3840.0,
// h: 2160.0,
// min_depth: 0.1,
// max_depth: 1000.0,
// };
//
//
// let view_point = camera.world_to_view([123.24, 30.3333, 34.5678],
// orientation, &viewport);
//
// println!("World to Viewport Position {:?}", view_point);
//
// let world_point = camera.view_to_world(view_point, orientation, &viewport);
//
// println!("Viewport to World Position {:?}", world_point);
// }
// }
