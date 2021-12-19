pub use child::*;
pub use frozen::*;
pub use local_world::*;
pub use parent::*;
pub use rotation::*;
pub use scale::*;
pub use scale_ty::*;
pub use translation::*;

mod child;
mod frozen;
mod local_world;
mod parent;
mod rotation;
mod scale;
mod scale_ty;
mod translation;


#[cfg(test)]
mod data_test {
    use crate::container::rotation::Rotation;
    use crate::{Frozen, Parent, Scale, Translation};

    #[test]
    fn data_size() {
        let frozen_size = std::mem::size_of::<Frozen>();
        assert!(frozen_size.eq(&0));

        let parent_size = std::mem::size_of::<Parent>();
        assert_eq!(parent_size & (parent_size - 1), 0);


        let position_size = std::mem::size_of::<Translation>();
        assert_eq!(position_size & (position_size - 1), 0);

        let rotation_size = std::mem::size_of::<Rotation>();
        assert_eq!(rotation_size & (rotation_size - 1), 0);

        let scale_size = std::mem::size_of::<Scale>();
        assert_eq!(scale_size & (scale_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let frozen_alignment = std::mem::align_of::<Frozen>();
        assert_eq!(frozen_alignment & (frozen_alignment - 1), 0);

        let parent_alignment = std::mem::align_of::<Parent>();
        assert_eq!(parent_alignment & (parent_alignment - 1), 0);

        let position_alignment = std::mem::align_of::<Translation>();
        assert_eq!(position_alignment & (position_alignment - 1), 0);

        let rotation_alignment = std::mem::align_of::<Rotation>();
        assert_eq!(rotation_alignment & (rotation_alignment - 1), 0);

        let scale_alignment = std::mem::align_of::<Scale>();
        assert_eq!(scale_alignment & (scale_alignment - 1), 0);
    }
}
