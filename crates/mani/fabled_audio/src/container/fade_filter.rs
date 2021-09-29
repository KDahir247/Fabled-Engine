#[derive(Copy, Clone, Debug)]
pub enum FadeFilter {
    // abruptly stop the clip after reaching the destination
    ABRUPT,
    // softly fade out the clip till the volume reaches zero then stop the clip after reaching the
    // destination
    FADE,
}

impl Default for FadeFilter {
    fn default() -> Self {
        Self::ABRUPT
    }
}
