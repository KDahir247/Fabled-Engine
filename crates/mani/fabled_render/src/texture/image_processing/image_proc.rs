use crate::texture::container::{ColorTarget, Extent2d, Extent3d, TextureData};
use crate::texture::image_processing::FilterType;
use crate::texture::ImageProcError;
use image::GenericImageView;

#[repr(align(32))]
pub struct ImageProcessing {
    dyn_texture: image::DynamicImage,
}

impl ImageProcessing {
    pub fn new<T: 'static>(
        texture: TextureData,
        color_target_predicate: fn(image::ImageBuffer<T, Vec<u8>>) -> ColorTarget,
    ) -> Result<ImageProcessing, ImageProcError>
    where
        T: image::Pixel<Subpixel = u8>, {
        let dyn_texture =
            image::ImageBuffer::from_raw(texture.size.width, texture.size.height, texture.data)
                .ok_or(ImageProcError::InSufficientAllocationSize)?;


        let texture_target = color_target_predicate(dyn_texture);

        let dyn_texture: image::DynamicImage = texture_target.into();

        Ok(Self { dyn_texture })
    }

    pub fn blur(mut self, sigma: f32) -> Self {
        self.dyn_texture = self.dyn_texture.blur(sigma);
        self
    }

    pub fn unsharpened(mut self, sigma: f32, threshold: i32) -> Self {
        self.dyn_texture = self.dyn_texture.unsharpen(sigma, threshold);
        self
    }

    pub fn rotate90(mut self) -> Self {
        self.dyn_texture = self.dyn_texture.rotate90();
        self
    }

    pub fn rotate180(mut self) -> Self {
        self.dyn_texture = self.dyn_texture.rotate180();
        self
    }

    pub fn rotate270(mut self) -> Self {
        self.dyn_texture = self.dyn_texture.rotate270();
        self
    }

    pub fn flip_horizontal(mut self) -> Self {
        image::imageops::flip_horizontal_in_place(&mut self.dyn_texture);
        self
    }

    pub fn flip_vertical(mut self) -> Self {
        image::imageops::flip_vertical_in_place(&mut self.dyn_texture);
        self
    }

    pub fn resize(mut self, target_dim: Extent2d, sample_filter: FilterType) -> Self {
        let rgba_8buff = image::imageops::resize(
            &self.dyn_texture,
            target_dim.width,
            target_dim.height,
            sample_filter.into(),
        );

        self.dyn_texture = image::DynamicImage::ImageRgba8(rgba_8buff);

        self
    }

    pub fn opacity(mut self, opacity: u8) -> Self {
        let mut rgba_img = self.dyn_texture.to_rgba8();

        let (width, height) = rgba_img.dimensions();
        for y in 0..height {
            for x in 0..width {
                rgba_img.get_pixel_mut(x, y)[3] = opacity;
            }
        }

        self.dyn_texture = image::DynamicImage::ImageRgba8(rgba_img);

        self
    }

    pub fn crop(mut self, start_point: Extent2d, end_point: Extent2d) -> Self {
        // width = width value - x;  to get the length to crop from the width if the
        // crop point start at x. height = height value - y; to get the length
        // to crop from the height if the crop point start at y.

        let delta_extent_width = end_point.width - start_point.width;
        let delta_extent_height = end_point.height - start_point.height;

        self.dyn_texture = self.dyn_texture.crop_imm(
            start_point.width,
            start_point.height,
            delta_extent_width,
            delta_extent_height,
        );

        self
    }

    pub fn replace(mut self, top_tex: &ImageProcessing, coord: Extent2d) -> Self {
        image::imageops::replace(
            &mut self.dyn_texture,
            &top_tex.dyn_texture,
            coord.width,
            coord.height,
        );
        self
    }

    pub fn overlay(mut self, top_tex: &ImageProcessing, coord: Extent2d) -> Self {
        image::imageops::overlay(
            &mut self.dyn_texture,
            &top_tex.dyn_texture,
            coord.width,
            coord.height,
        );

        self
    }

    pub fn build(self) -> TextureData {
        let dyn_tex = self.dyn_texture;

        TextureData {
            data: dyn_tex.to_bytes(),
            size: Extent3d {
                width: dyn_tex.width(),
                height: dyn_tex.height(),
                depth_or_array_layers: 1,
            },
            sample_count: 1,
            mip_level: 0,
            color_type: dyn_tex.color().into(),
            rows_per_image: dyn_tex.width() * dyn_tex.color().channel_count() as u32,
        }
    }
}

#[cfg(test)]
mod image_processing_test {
    use crate::texture::codecs::*;
    use crate::texture::common::*;
    use crate::texture::container::{ColorTarget, Extent2d};
    use crate::texture::image_processing::{FilterType, ImageProcessing};

    #[test]
    fn creation_test() {
        let dds_texture = load_test_textures("dds").pop().unwrap();
        let jpg_texture = load_test_textures("jpg").pop().unwrap();
        let png_texture = load_test_textures("png").pop().unwrap();
        let tiff_texture = load_test_textures("tiff").pop().unwrap();


        // Png
        let png_loader = PngTextureLoader::default();
        let png_yellow = png_loader
            .load(
                png_texture,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        println!(
            "before Png : {:?}, {:?}",
            png_yellow.size, png_yellow.color_type
        );

        let png_texture = ImageProcessing::new(png_yellow, ColorTarget::ImageRgba8)
            .unwrap()
            .build();

        println!(
            "after Png: {:?}, {:?}",
            png_texture.size, png_texture.color_type
        );


        // DDS
        let dds_loader = DdsTextureLoader::default();
        let dds_yellow = dds_loader
            .load(
                dds_texture,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        println!(
            "before DDS: {:?}, {:?}",
            dds_yellow.size, dds_yellow.color_type
        );

        let dds_texture = ImageProcessing::new(dds_yellow, ColorTarget::ImageRgb8)
            .unwrap()
            .build();

        println!(
            "after DDS: {:?}, {:?}",
            dds_texture.size, dds_texture.color_type
        );

        // HDR file not supported for image processing only 8 bit channels currently(16
        // bit channel will be supported later).

        // JPEG
        let jpg_loader = JpgTextureLoader::default();
        let jpg_yellow = jpg_loader
            .load(
                jpg_texture,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        println!(
            "before Jpeg: {:?}, {:?}",
            jpg_yellow.size, jpg_yellow.color_type
        );

        let jpg_texture = ImageProcessing::new(jpg_yellow, ColorTarget::ImageRgb8)
            .unwrap()
            .build();

        println!(
            "after Jpeg: {:?}, {:?}",
            jpg_texture.size, jpg_texture.color_type
        );

        // TIFF
        let tiff_loader = TiffTextureLoader::default();
        let tiff_yellow = tiff_loader
            .load(
                tiff_texture,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        println!(
            "before Tiff: {:?}, {:?}",
            tiff_yellow.size, tiff_yellow.color_type
        );

        let tiff_texture = ImageProcessing::new(tiff_yellow, ColorTarget::ImageRgba8)
            .unwrap()
            .build();

        println!(
            "after Tiff: {:?}, {:?}",
            tiff_texture.size, tiff_texture.color_type
        );
    }

    fn init_test() -> ImageProcessing {
        // Png
        let texture = load_test_textures("png").pop().unwrap();

        let png_loader = PngTextureLoader::default();
        let png_yellow = png_loader
            .load(
                texture,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        ImageProcessing::new(png_yellow, ColorTarget::ImageRgba8).unwrap()
    }

    #[test]
    fn blur_test() {
        let img_proc = init_test();
        let save_dir = save_test_texture("test/albedo/image_proc/pngyellow_blur.png");

        let result = img_proc.blur(10.0).build();

        result.write_to(save_dir, ColorTarget::ImageRgba8).unwrap();
        // Draw the result to a file
    }

    #[test]
    fn unsharpened_test() {
        let img_proc = init_test();
        let result = img_proc.unsharpened(20.0, 15).build();

        let sav_dir = save_test_texture("test/albedo/image_proc/pngyellow_unsharpen.png");
        result.write_to(sav_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn rotation90_test() {
        let img_proc = init_test();
        let result = img_proc.rotate90().build();

        let sav_dir = save_test_texture("test/albedo/image_proc/pngyellow_rot90.png");

        result.write_to(sav_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn rotation180_test() {
        let img_proc = init_test();
        let result = img_proc.rotate180().build();
        let sav_dir = save_test_texture("test/albedo/image_proc/pngyellow_rot180.png");
        result.write_to(sav_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn rotation270_test() {
        let img_proc = init_test();
        let result = img_proc.rotate270().build();
        let sav_dir = save_test_texture("test/albedo/image_proc/pngyellow_rot270.png");

        result.write_to(sav_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn flip_horizontal_test() {
        let img_proc = init_test();
        let result = img_proc.flip_horizontal().build();
        let save_dir = save_test_texture("test/albedo/image_proc/pngyellow_fliph.png");

        result.write_to(save_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn flip_vertical_test() {
        let img_proc = init_test();
        let result = img_proc.flip_vertical().build();
        let save_dir = save_test_texture("test/albedo/image_proc/pngyellow_flipv.png");

        result.write_to(save_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn resize_test() {
        let img_proc = init_test();
        let save_dir = save_test_texture("test/albedo/image_proc/pngyellow_resize.png");

        let result = img_proc
            .resize(
                Extent2d {
                    width: 1024,
                    height: 1024,
                },
                FilterType::Lanczos3,
            )
            .build();

        result.write_to(save_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn opacity_test() {
        let img_proc = init_test();
        let result = img_proc.opacity(128).build();
        let save_dir = save_test_texture("test/albedo/image_proc/pngyellow_opacity.png");
        result.write_to(save_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn crop_test() {
        let img_proc = init_test();
        let save_dir = save_test_texture("test/albedo/image_proc/pngyellow_crop.png");

        let result = img_proc
            .crop(
                Extent2d {
                    width: 195,
                    height: 90,
                },
                Extent2d {
                    width: 422,
                    height: 250,
                },
            )
            .build();


        result.write_to(save_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn replace() {
        let img_proc = init_test();

        let texture = load_test_textures("png");
        let save_dir = save_test_texture("test/albedo/image_proc/pngyellow_replace.png");

        let second_img = PngTextureLoader::default()
            .load(&texture[1], &TextureDescriptor::default())
            .unwrap();

        let result1 = ImageProcessing::new(second_img, ColorTarget::ImageRgba8).unwrap();

        let result = img_proc
            .replace(
                &result1,
                Extent2d {
                    width: 350,
                    height: 350,
                },
            )
            .build();

        result.write_to(save_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn overlay_test() {
        let img_proc = init_test();

        let texture = load_test_textures("png");
        let save_dir = save_test_texture("test/albedo/image_proc/pngyellow_overlay.png");

        let second_img = PngTextureLoader::default()
            .load(&texture[1], &TextureDescriptor::default())
            .unwrap();

        let result1 = ImageProcessing::new(second_img, ColorTarget::ImageRgba8).unwrap();

        let result = img_proc
            .overlay(
                &result1,
                Extent2d {
                    width: 100,
                    height: 100,
                },
            )
            .build();

        result.write_to(save_dir, ColorTarget::ImageRgba8).unwrap();
    }
}
