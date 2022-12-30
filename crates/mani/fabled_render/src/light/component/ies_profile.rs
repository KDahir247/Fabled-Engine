// TILT={filename} will not be supported
// TILT=None will be supported
// TILT=INCLUDE might or might not be supported. Contains additional parameter
// lamp-to-luminaire geometry, # of pairs of angles and multiplying factors,
// angles, multiplying factors

// snippet example
// IESNA:LM-63-1995 // Version
// [TEST]
// [MANUFAC] PHILIPS CDM-R111 35W/830 10°
// [LUMCAT] HIPAR111 35W/830 10°
// [LUMINAIRE] HIPAR111 35W/830 10°
// [LAMPCAT] HIPAR111 35W / 830 1
// [LAMP]    1400 lm,   43 W // We don't care about the above data except for
// the version TILT=NONE
// 1    1400 1.0 37 24 1 2 -0.110 0.000 0.000

use fabled_math::Vector3;

#[allow(non_camel_case_types)]
pub enum ECSVersion {
    LM_63_1986,
    LM_63_1991,
    LM_63_1995,
}

// type C is the most common type used in computer graphics. A, B are rarely
// used.
pub enum PhotometryType {
    A,
    B,
    C,
}

pub enum IESUnit {
    Feet,
    Meter,
}

// Luminous intensity as intensity
// todo fix the way the data is structured.
pub struct IESProfile {
    pub version: ECSVersion,
    pub total_light: usize,
    pub total_lumen: f32, // can be -1.0 from absolute photometry (very rare), but handle case
    pub candela_multiplier: f32,
    pub number_of_vertical_angle: usize,
    pub number_of_horizontal_angle: usize,
    pub photometric_type: PhotometryType,
    pub unit_type: IESUnit,
    pub luminaire_dimension: Vector3, // width height length in that order
    pub ballast_factor: f32,
    pub future_use: f32,
    pub input_watt: f32,
    pub horizontal_angle: Vec<f32>,
    pub vertical_angle: Vec<f32>,
    pub candela_values: Vec<f32>,
    // This will be used to distinguish IES from IES mask.
    // mask will allow the user to specify the target intensity rather then
    // fetch if from the IES profile.
    // if mask
    // eg. let multiplier = IES.desired_intensity * IES.integrated_intensity;
    // let light_intensity = IES.max_intensity * multiplier;
    pub desired_intensity: f32,
}
