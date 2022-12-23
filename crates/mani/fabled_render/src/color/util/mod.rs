mod blend;
mod color_op;
mod transfer_function;

pub use blend::*;
pub use color_op::*;
use fabled_math::Vector3;
pub use transfer_function::*;


pub const REC2020_LUMINANCE: Vector3 = Vector3::set(0.262_698_35, 0.678_008_8, 0.059_292_894);
pub const DCI_P3_LUMINANCE: Vector3 = Vector3::set(2.094_916_9e-1, 7.215_952e-1, 6.891_306_5e-2);
pub const AP1_LUMINANCE: Vector3 = Vector3::set(0.2722287168, 0.6740817658, 0.0536895174);
pub const SRGB_LUMINANCE: Vector3 = Vector3::set(0.21263682, 0.71518298, 0.0721802);
