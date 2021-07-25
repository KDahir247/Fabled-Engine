pub mod ktx;

use crate::FlipAxis;
use bitflags::*;
pub use ktx::*;
//todo add a util file under ext that will convert height map to normal map.
// and convert height map to horizon map.
// Generating horizon cube

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct KTXDescriptor {
    pub flip_axis: Option<FlipAxis>,
    pub transcode_flag: KtxTranscodeFlag,
    pub transcode_format: KtxTranscodeFormat,
}

bitflags! {
pub struct KtxTranscodeFlag : u32 {
        /// Makes the width and height to the next Pow 2
        /// Required to satisfy some API specialization (WebGL 1.0, PVRTC format)
        const PVRTC_DECODE_TO_NEXT_POW2 = 2;
        /// Disregards if the width and height is uniform to the pow 2
        /// Aim for whatever yield the best quality
        const HIGHEST_QUALITY = 4;
        /// Convert the Alpha data from the alpha channel to Opaque format
        const ALPHA_DATA_TO_OPAQUE_FORMAT = 32;
}
    }

impl From<KtxTranscodeFlag> for libktx_rs::TranscodeFlags {
    fn from(v: KtxTranscodeFlag) -> Self {
        libktx_rs::TranscodeFlags::from_bits(v.bits).expect("Bit to ktx transcode flag")
    }
}

#[derive(Copy, Debug, Clone)]
#[repr(u32)]
pub enum KtxTranscodeFormat {
    /// ETC1-2
    ETC1RGB = 0,
    ETC2RGBA = 1,

    /// BC/block compression (Desktop, some mobile devices)
    /// BC1-5, BC7
    BC1RGB = 2,
    BC3RGBA = 3,
    BC3R = 4,
    BC5RG = 5,
    BC7RGBA = 6,

    /// PVRTC1 4bpp (mobile, PowerVR devices)
    PVRTC14RGB = 8,
    PVRTC14RGBA = 9,

    /// ASTC (mobile, Intel devices, and newer GPU)
    /// Supported GPU currently
    /// Mali (ARM) : Mali-T620, Mali-T720, Mali-T760, Mali-T820/T830, and Mali-T860\T880
    /// Nvidia : Kepler, Maxwell-based, and Tegra SoCs
    /// Intel : Skylake and later.
    /// AMD : any AMD Radeon GPU family
    ASTC4X4RGBA = 10,

    /// ATC and FXT1 formats are not supported by KTX2 as there are no equivalent VKFormats.
    /// Don't use with KTX2. KTX1 is supported though.
    PVRTC24RGB = 18,
    PVRTC24RGBA = 19,

    // EAC
    ETC2EACR11 = 20,
    ETC2EACRG11 = 21,

    /// Uncompressed (raw)
    // Equivalent to RGBA 8888 (8 bits per channel)
    RGBA32 = 13,
    // Equivalent to R5G6B5 refer to (CIE 1931 color space)
    // since eye can perceive more shade of green
    RGB565 = 14,
    // Equivalent to R5G6R5 refer to (CIE 1931 color space)
    // since eye can perceive more shade of green
    RGR565 = 15,
    // 4 bit per channel
    RGBA4444 = 16,

    /// Value for automatic selection of RGB or RGBA depending if aplha is present
    ETC = 22,
    BCLOR3 = 23,

    ///No Selection specified
    NoSelection = 2147483647,
}

#[cfg(test)]
mod ktx_test {

    use crate::{KTXDescriptor, KtxTranscodeFlag, KtxTranscodeFormat};

    #[test]
    fn data_alignment() {
        let transcode = std::mem::size_of::<KtxTranscodeFlag>();
        assert_eq!(transcode & (transcode - 1), 0);

        let transcode_format = std::mem::size_of::<KtxTranscodeFormat>();
        assert_eq!(transcode_format & (transcode_format - 1), 0);

        //KTX Loader is 0 bytes and bits.
        /* let ktx_loader = std::mem::size_of::<KtxTextureLoader>();
        assert_eq!(ktx_loader & (ktx_loader - 1), 0);*/

        let ktx_desc = std::mem::size_of::<KTXDescriptor>();
        assert_eq!(ktx_desc & (ktx_desc - 1), 0);
    }
}
