#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FovScalingAlgorithm {
    HorizontalPlus,
    Anamorphic,
    VerticalMinus,
    Stretch,
}
