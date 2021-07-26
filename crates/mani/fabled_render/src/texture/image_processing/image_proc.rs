use crate::texture::image_processing::Target;
use crate::{Extent3d, Texture};
use image::GenericImageView;

//Want to have a history
pub struct ImageProc {
    dyn_texture: image::DynamicImage,
}

impl ImageProc {
    //Doesn't work with u16 just u8
    pub fn new<T: 'static>(
        texture: Texture,
        dyn_im: fn(image::ImageBuffer<T, Vec<u8>>) -> Target,
    ) -> ImageProc
    where
        T: image::Pixel<Subpixel = u8>,
    {
        let dyn_texture =
            image::ImageBuffer::from_raw(texture.size.width, texture.size.height, texture.data)
                .unwrap();

        let x = dyn_im(dyn_texture);

        let b: image::DynamicImage = x.into();

        Self { dyn_texture: b }
    }

    pub fn blur(self) -> Self {
        //let c= image::RgbaImage::from_raw().a;

        self.dyn_texture.blur(5.0);
        //let a = image::imageops::blur(x, 15.0);
        //a
        self
    }

    pub fn build(self) -> Texture {
        let dyn_tex = self.dyn_texture;

        Texture {
            data: vec![],
            size: Extent3d {
                width: dyn_tex.width(),
                height: dyn_tex.height(),
                depth_or_array_layers: 1,
            },
            sample_count: 0,
            mip_level: 1,
            color_type: dyn_tex.color().into(),
            rows_per_image: dyn_tex.width() * dyn_tex.color().channel_count() as u32,
        }
    }
}

#[cfg(test)]
mod image_processing_test {
    use crate::texture::image_processing::{ImageProc, Target};
    use crate::{PngTextureLoader, TextureDescriptor, PNG_TEST_TEXTURE};
    use texture_synthesis::image::imageops::blur;

    #[test]
    fn creation_test() {
        let png_loader = PngTextureLoader::default();
        let pngyellow = png_loader
            .load(
                PNG_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        println!("{} {}", pngyellow.size.width, pngyellow.size.height);

        let img_proc = ImageProc::new(pngyellow, Target::ImageLuma8);
        let img_texture = img_proc.build();

        println!("{} {}", img_texture.size.width, img_texture.size.height);
    }

    #[test]
    fn blur_test() {
        let file = std::fs::File::open(PNG_TEST_TEXTURE).unwrap();
        let buf = std::io::BufReader::new(file);

        let img = image::load(buf, image::ImageFormat::Png).unwrap();

        let c = img.blur(40.0);

        //let blurred_img = super::blur(img);

        c.save_with_format("D:\\Study\\Fabled Engine\\crates\\mani\\fabled_render\\src\\texture\\texture\\test\\albedo\\pngyellowblurred.png", image::ImageFormat::Png).unwrap();
    }
}
