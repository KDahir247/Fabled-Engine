mod aperture_len;
mod attenuation;
mod light;
mod shutter;

pub use aperture_len::*;
pub use attenuation::*;
pub use light::*;
pub use shutter::*;


#[cfg(test)]
mod data_test {

    #[test]
    fn data_size() {}


    #[test]
    fn data_alignment() {}
}
