#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DecayType {
    // No decay. The light's intensity will not diminish with distance.
    None,
    // Linear decay. The light's intensity will diminish linearly with the distance from the
    // light.
    Linear,
    // Quadratic decay. The light's intensity will diminish with the squared distance from the
    // light. This is the most physically accurate decay rate.
    Quadratic,
    // Cubic decay. The light's intensity will diminish with the cubed distance from the light.
    Cubic,
}
