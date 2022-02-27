pub trait ColorSpace {
    type TargetSpace;

    fn convert_space_from(_: Self::TargetSpace) -> Self;
}
