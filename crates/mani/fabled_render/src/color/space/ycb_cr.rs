// is this going to be linear or non-linear?
// we will support YCbCr444 for full res for luminance and two chrominance
// channels and 422 with will mean full res for luminance and half for
// chrominance channel
pub struct YCbCr {
    /// The luminance or brightness (Y)
    pub y: f32,
    /// blue minus luminance (B-Y)
    pub cb: f32,
    /// red minus luminance (C-Y)
    pub cr: f32,
}
