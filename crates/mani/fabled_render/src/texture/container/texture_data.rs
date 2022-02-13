use crate::texture::container::{ColorType, Extent3d};
use crate::texture::{ColorTarget, TextureError};
use std::path::Path;

#[repr(align(16))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextureData {
    pub data: Vec<u8>,
    pub size: Extent3d,
    pub sample_count: u32,
    pub mip_level: u32,
    /// Bytes per row of the image.
    pub rows_per_image: u32,
    pub color_type: ColorType,
}


impl TextureData {
    pub fn write_to<P: AsRef<Path>, T: 'static + image::Pixel<Subpixel = u8>>(
        &self,
        path: P,
        color_target_predicate: fn(image::ImageBuffer<T, Vec<u8>>) -> ColorTarget,
    ) -> Result<(), TextureError> {
        let img_buf =
            image::ImageBuffer::from_raw(self.size.width, self.size.height, self.data.to_owned())
                .ok_or(TextureError::InSufficientAllocationSize)?;

        let dyn_img: image::DynamicImage = color_target_predicate(img_buf).into();

        dyn_img.save(path).map_err(TextureError::ImageError)?;

        Ok(())
    }
}
