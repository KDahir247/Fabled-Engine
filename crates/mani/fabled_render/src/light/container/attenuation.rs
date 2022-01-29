#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Attenuation {
    InverseSquareAttenuation,

    ExponentialAttenuation,

    SmoothAttenuation,
}
