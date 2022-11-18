use fabled_math::Vector2;

#[derive(Copy, Clone, Debug)]
pub enum AspectRatioMode {
    // No lock at all
    WindowSize,
    // Lock the aspect ratio to constraint the resolution to be within the aspect ratio
    FixedRatio(AspectCommon),
    // Lock the resolution
    FixedResolution(ResolutionCommon),
    // Lock the width of the resolution
    FixedWidth,
    // Lock the height of the resolution
    FixedHeight,
}

#[non_exhaustive]
pub struct ResolutionCommon(pub Vector2);

impl ResolutionCommon {}

#[non_exhaustive]
pub struct AspectCommon(pub Vector2);

impl AspectCommon {
    pub const XGA: Vector2 = Vector2::set(4.0, 3.0);

    pub const WIDE_XGA: Vector2 = Vector2::set(16.0, 10.0);

    pub const WIDE_SCREEN: Vector2 = Vector2::set(16.0, 9.0);

    pub const ULTRA_WIDESCREEN: Vector2 = Vector2::set(21.0, 9.0);

    pub const SUPER_ULTRA_WIDESCREEN: Vector2 = Vector2::set(32.0, 9.0);
}
