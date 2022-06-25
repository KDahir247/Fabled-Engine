pub trait ColorSpace {
    type TargetSpace;

    fn convert_space_from(_: Self::TargetSpace) -> Self;
}
// // T is will get the bit size for each channel
// // N is the number of channels in the color space type.
// pub trait Color<T, const N: usize> {
//     fn get(&self) -> [T; N];
// }
//
// // Can be represented as 0 to 1
// pub trait SRgbSpace<T>: Color<T, 3> {}
//
