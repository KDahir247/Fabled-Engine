use crate::texture::container::{ColorType, Extent3d};
use crate::texture::{ColorTarget, TextureError};
use std::ops::{Add, Index, IndexMut, Range};
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

    fn get_pixel_range(&self, x: u32, y: u32) -> Range<usize> {
        let channel = self.color_type.channel_count() as usize;

        let pixel_indices = (x + y * self.size.width) as usize * channel;

        (pixel_indices)..(pixel_indices + channel)
    }
}


impl Index<(u32, u32)> for TextureData {
    type Output = [u8];

    fn index(&self, (row, column): (u32, u32)) -> &Self::Output {
        let pixel_range = self.get_pixel_range(row, column);

        let validated_pixel = self.data.get(pixel_range).unwrap_or_default();

        validated_pixel
    }
}

impl IndexMut<(u32, u32)> for TextureData {
    fn index_mut(&mut self, (row, column): (u32, u32)) -> &mut Self::Output {
        let pixel_range = self.get_pixel_range(row, column);

        let validated_mut_pixel = self.data.get_mut(pixel_range).unwrap_or_default();

        validated_mut_pixel
    }
}
