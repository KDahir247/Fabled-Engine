use image::ImageDecoder;

//todo
#[derive(Default, Clone)]
pub struct JpgTextureLoader {}

impl JpgTextureLoader {
    pub fn load(&self, path: &[u8]) -> () {
        let jpg_decoder = image::codecs::jpeg::JpegDecoder::new(path).unwrap();
    }
}

/*#[cfg(test)]
mod test {

    use crate::codecs::*;

    #[test]
    fn test() {
        let g = JpgTextureLoader::default();
        let f = std::fs::read("D:\\Study\\Fabled Engine\\crates\\mani\\fabled_render\\src\\texture\\texture\test\\albedo\\scpgdgca_2K_Albedo.jpg");
        let a = g.load(f.unwrap().as_slice());
    }
}*/
