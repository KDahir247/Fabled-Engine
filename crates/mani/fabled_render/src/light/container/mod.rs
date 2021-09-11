mod aperture_len;
mod attenuation;
mod iso_speed;
mod light;
mod shutter;

pub use aperture_len::*;
pub use attenuation::*;
pub use iso_speed::*;
pub use light::*;
pub use shutter::*;


#[cfg(test)]
mod data_test {

    #[test]
    fn data_size() {}


    #[test]
    fn data_alignment() {}
}
