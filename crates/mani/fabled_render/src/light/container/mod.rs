pub mod light;

pub use light::*;

#[cfg(test)]
mod data_alignment_test {
    use crate::light::container::Light;

    #[test]
    fn data_alignment() {
        let light = std::mem::size_of::<Light>();
        assert_eq!(light & (light - 1), 0);
    }
}
