#[derive(Copy, Clone, Debug)]
pub enum AspectRatioMode {
    // No lock at all
    WindowSize,
    // Lock the aspect ratio to constraint the resolution to be within the aspect ratio
    FixedRatio,
    // Lock the resolution
    FixedResolution,
    // Lock the width of the resolution
    FixedWidth,
    // Lock the height of the resolution
    FixedHeight,
}
