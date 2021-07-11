#[derive(Default, Clone)]
pub struct DdsTextureLoader {
    //Maybe will store the saved texture and the new texture
}
// DDS File Format
// The default value is: R8G8B8.
// No generating Mip Map level
// If mipmap is "Yes", the sizes of saved images must be equal to the power of two (128, 256, 512, 1024 etc.).
impl DdsTextureLoader {
    //Decoder
    pub fn load<P: AsRef<std::path::Path>>(&self, path: P) -> anyhow::Result<image::DynamicImage> {
        //RGBA8 (R8, G8, B8, A9)

        let buf = std::fs::read(path.as_ref()).unwrap();

        let dds_decoder = image::codecs::dds::DdsDecoder::new(buf.as_slice()).unwrap();

        let dyn_img = image::DynamicImage::from_decoder(dds_decoder).unwrap();

        Ok(dyn_img) //todo maybe return custom container.
    }
}
