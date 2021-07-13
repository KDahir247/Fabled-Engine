pub use codecs::*;
use common::*;
pub use container::*;
pub use core::*;
pub use texture_data::*;
pub use texture_sampler::*;
pub use texture_view_dimension::*;
pub use EXT::*;

mod storage;
mod synthesizer;

//Clean up
pub mod EXT;
pub mod codecs;
mod common;
mod container;
mod texture_aspect;
mod texture_data;
mod texture_dimension;
mod texture_sample_type;
mod texture_sampler;
mod texture_view_dimension;

#[cfg(test)]
mod data_alignment_test {
    use crate::texture::container::*;
    use crate::texture::*;

    #[test]
    fn data_alignment() {
        let border_color = std::mem::size_of::<BorderColor>();
        println!("Border Color takes up {:?} bytes", border_color);

        let extent3d = std::mem::size_of::<Extent3d>();
        println!("Extent3d takes up {:?} bytes", extent3d);

        let texture_data = std::mem::size_of::<Texture>();
        println!("Texture data takes up {:?} bytes", texture_data);

        let texture_dimension = std::mem::size_of::<TextureViewDimension>();
        println!(
            "Texture view dimension takes up {:?} bytes",
            texture_dimension
        );

        let sampler = std::mem::size_of::<TextureSampler>();
        println!("Sampler takes up {:?} bytes", sampler);

        let color_space = std::mem::size_of::<ColorSpace>();
        println!("Color space takes up {:?} bytes", color_space);

        let flip_axis = std::mem::size_of::<flip_axis::FlipAxis>();
        println!("Flip axis takes up {:?}, bytes", flip_axis);

        let texture_access = std::mem::size_of::<texture_access::StorageTextureAccess>();
        println!("Texture access takes up {:?} bytes", texture_access);

        let texture_aspect = std::mem::size_of::<texture_aspect::TextureAspect>();
        println!("Texture aspect takes up {:?} bytes", texture_aspect);

        let sample_type = std::mem::size_of::<texture_sample_type::TextureSampleType>();
        println!("Texture sample type takes up {:?} bytes", sample_type);

        let texture_dimension = std::mem::size_of::<texture_dimension::TextureDimension>();
        println!("Texture dimension takes up {:?} bytes", texture_dimension);
    }
}
