#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FovScalingAlgorithm {
    HorizontalPlus,
    Anamorphic,
    VerticalMinus,
    Stretch,
}
