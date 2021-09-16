mod aperture_len;
mod appearance;
mod attenuation;
mod iso_speed;
mod light;
mod shutter;
mod unit_type;

pub use aperture_len::*;
pub use appearance::*;
pub use attenuation::*;
pub use iso_speed::*;
pub use light::*;
pub use shutter::*;
pub use unit_type::*;


#[cfg(test)]
mod data_test {

    #[test]
    fn data_size() {}


    #[test]
    fn data_alignment() {}
}
