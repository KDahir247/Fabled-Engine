pub mod directional_light;
pub mod light;
pub mod point_light;
pub mod spot_light;

pub use directional_light::*;
pub use light::*;
pub use point_light::*;
pub use spot_light::*;


#[cfg(test)]
mod data_alignment_test {
    use crate::light::container::Light;

    #[test]
    fn data_alignment() {
        let light = std::mem::size_of::<Light>();
        assert_eq!(light & (light - 1), 0);
    }
}
