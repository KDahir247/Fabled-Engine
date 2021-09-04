mod compress;
mod transcode;

pub use compress::*;
pub use transcode::*;

pub use bitflags::*;

use crate::texture::_core::convert::TryFrom;
use crate::texture::container::ColorSpace;

//  ---------- Compression ----------

#[derive(Debug)]
#[repr(C, align(64))]
pub enum TranscodeSource {
    BasisTexture { basis: BasisTexture },
    BasisPath { path: String },
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BasisCompressionFormat {
    UASTC4x4 = 1,
    ETC1S = 0,
}

impl From<BasisCompressionFormat> for basis_universal::BasisTextureFormat {
    fn from(
        super_compression_algorithm: BasisCompressionFormat,
    ) -> basis_universal::BasisTextureFormat {
        match super_compression_algorithm {
            BasisCompressionFormat::UASTC4x4 => basis_universal::BasisTextureFormat::UASTC4x4,
            BasisCompressionFormat::ETC1S => basis_universal::BasisTextureFormat::ETC1S,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CompressionQuality {
    Default,
    Minimum,
    Maximum,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MipmapDescriptor {
    pub generate_mipmap: bool,
    pub color_space: ColorSpace,
    /// Smallest dimensions mipmap that will be generated. (Keep it to the power
    /// of two.)
    pub smallest_dimensions: u32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct UserData {
    pub userdata0: u32,
    pub userdata1: u32,
}

#[repr(align(16))]
#[derive(Default, Debug, Copy, Clone)]
pub struct RDODescriptor {
    /// Enable/disable U_ASTC RDO post-processing and set U_ASTC RDO quality
    /// scalar to X. Lower values=higher quality/larger LZ compressed files,
    /// higher values=lower quality/smaller LZ compressed files. Good range
    /// to try is [.2-4]
    pub rdo_uastc_quality_scalar: Option<f32>,

    /// Disable backend's endpoint rate distortion optimizations (slightly
    /// faster, less noisy output, but lower quality per output bit)
    pub no_endpoint_rdo: bool,

    /// Disable backend's selector rate distortion optimizations (slightly
    /// faster, less noisy output, but lower quality per output bit)
    pub no_selector_rdo: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct CompressionDescriptor {
    pub compression_format: BasisCompressionFormat,
    pub compression_quality: CompressionQuality,
    pub color_space: ColorSpace,
    pub mip_map_desc: Option<MipmapDescriptor>,
    pub rdo_desc: Option<RDODescriptor>,
}
impl Default for CompressionDescriptor {
    fn default() -> Self {
        Self {
            compression_format: BasisCompressionFormat::UASTC4x4,
            compression_quality: CompressionQuality::Default,
            color_space: ColorSpace::LinearSpace,
            mip_map_desc: None,
            rdo_desc: None,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct BasisTexture {
    pub data: Vec<u8>,
    pub file_size: u32,
}

//  ---------- Transcode ----------\
#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum TranscodeTextureFormat {
    /// ETC1_RGB
    ETC1RGB = 0,
    /// ETC2_RGB
    ETC2RGB = 1,

    // BC/block compression (Desktop, some mobile devices)
    /// BC1_RGB
    BC1RGB = 2,
    /// BC3_RGBA
    BC3RGBA = 3,
    /// BC4_R
    BC4R = 4,
    /// BC5_RG
    BC5RG = 5,
    /// BC7_RGBA
    BC7RGBA = 6,

    // PVRTC1 4bpp (mobile, PowerVR devices)
    /// PVRTC1_4_RGB
    PVRTC14RGB = 8,
    /// PVRTC1_4_RGBA
    PVRTC14RGBA = 9,

    // ASTC (mobile, Intel devices, and newer GPU)
    // Supported GPU currently
    // Mali (ARM) : Mali-T620, Mali-T720, Mali-T760, Mali-T820/T830, and Mali-T860\T880
    // Nvidia : Kepler, Maxwell-based, and Tegra Socs
    // Intel : Skylake and later,
    // AMD : any AMD Radeon GPU family
    /// ASTC_4x4_RGBA
    ASTC4X4RGBA = 10,

    // ATC (mobile, Adreno devices)
    /// ATC_RGB
    ATCRGB = 11,
    /// ATC_RGBA
    ATCRGBA = 12,

    // FX1 (desktop, Intel devices)
    /// FXT1_RGB
    FXT1RGB = 17,

    // ATC and FXT1 formats are not supported by some format type such as KTX2 as there are no
    // equivalent VKFormats. Don't use with KTX2 format, KTX1 is supported though.
    /// PVRTC2_4_RGB
    PVRTC24RGB = 18,
    /// PVRTC2_4_RGBA
    PVRTC24RGBA = 19,

    // EAC
    /// ETC2_EAC_R11
    ETC2EACR11 = 20,
    /// ETC2_EAC_RG11
    ETC2EACRG11 = 21,

    // Uncompressed (raw pixel) formats
    /// RGBA_32/R8G8B8A8
    RGBA32 = 13,
    /// RGB_565/R5G6B5
    RGB565 = 14,
    /// BGR_565/B5G6R5
    BGR565 = 15,
    /// RGBA_4444/R4G4B4A4
    RGBA4444 = 16,
}

impl From<TranscodeTextureFormat> for basis_universal::TranscoderTextureFormat {
    fn from(transcode_texture_format: TranscodeTextureFormat) -> Self {
        basis_universal::TranscoderTextureFormat::try_from(transcode_texture_format as i32)
            .unwrap_or(basis_universal::TranscoderTextureFormat::RGBA32)
    }
}

bitflags! {
    pub struct TranscodeDecodeFlags : u32{
        /// PVRTC1: decode non-pow2 ETC1S texture level to the next larger power of two (not implemented yet). If the texture level is already to the power of two ignore it.
        const PVRTC_DECODE_TO_NEXT_POW2 = 2;

        /// When decoding to an opaque texture format, if the basis file has alpha, decode the alpha slice instead of the color slice to the output texture format.
        /// This is primarily to allow decoding of textures with alpha to multiple ETC1 textures (one for color, another for alpha).
        const TRANSCODE_ALPHA_DATA_TO_OPAQUE_FORMAT = 4;

        /// Forbid usage of BC1 3 color blocks This flag is used internally when decoding to BC3.
        const BC1_FORBID_THREE_COLOR_BLOCKS = 8;

        /// The output buffer contains alpha endpoint/selector indices.
        /// Used internally when decoding formats like ASTC that require both color and alpha data to available when transcoding to the output format.
        const OUTPUT_HAS_ALPHA_INDICES = 16;

        /// Disregards if the width and height is uniform to the power of 2
        /// Aim for whatever yield the best quality
        const HIGHEST_QUALITY = 32;
    }
}

impl From<TranscodeDecodeFlags> for basis_universal::transcoding::DecodeFlags {
    fn from(transcode_flag: TranscodeDecodeFlags) -> Self {
        basis_universal::transcoding::DecodeFlags::from_bits(transcode_flag.bits)
            .expect("Bit to Transcode DecodeFlag")
    }
}

// Only use if you know what you're doing.
#[derive(Default, Debug, Copy, Clone)]
pub struct PixelManipulation {
    output_row_pitch_in_blocks_or_pixels: Option<u32>,
    output_rows_in_pixels: Option<u32>,
}

#[derive(Debug, Copy, Clone)]
#[repr(align(16))]
pub struct TranscodeDescriptor {
    texture_format: TranscodeTextureFormat,
    decode_flags: TranscodeDecodeFlags,
    pixel_manipulation: PixelManipulation,
}

impl Default for TranscodeDescriptor {
    fn default() -> Self {
        Self {
            texture_format: TranscodeTextureFormat::RGBA32,
            decode_flags: TranscodeDecodeFlags::HIGHEST_QUALITY,
            pixel_manipulation: Default::default(),
        }
    }
}

#[cfg(test)]
mod compression_test {
    use crate::texture::compression::*;

    #[test]
    fn data_size() {
        // Test for data alignment.
        let basis_compression_algorithm_size = std::mem::size_of::<BasisCompressionFormat>();
        assert_eq!(
            basis_compression_algorithm_size & (basis_compression_algorithm_size - 1),
            0
        );

        let compression_quality_size = std::mem::size_of::<CompressionQuality>();
        assert_eq!(compression_quality_size & (compression_quality_size - 1), 0);

        let mipmap_desc_size = std::mem::size_of::<MipmapDescriptor>();
        assert_eq!(mipmap_desc_size & (mipmap_desc_size - 1), 0);

        let rdo_desc_size = std::mem::size_of::<RDODescriptor>();
        assert_eq!(rdo_desc_size & (rdo_desc_size - 1), 0);

        let user_data_size = std::mem::size_of::<UserData>();
        assert_eq!(user_data_size & (user_data_size - 1), 0);

        let compression_desc_size = std::mem::size_of::<CompressionDescriptor>();
        assert_eq!(compression_desc_size & (compression_desc_size - 1), 0);

        let basis_texture_size = std::mem::size_of::<BasisTexture>();
        assert_eq!(basis_texture_size & (basis_texture_size - 1), 0);

        let transcode_source_size = std::mem::size_of::<TranscodeSource>();
        assert_eq!(transcode_source_size & (transcode_source_size - 1), 0);

        let transcode_texture_format_size = std::mem::size_of::<TranscodeTextureFormat>();
        assert_eq!(
            transcode_texture_format_size & (transcode_texture_format_size - 1),
            0
        );

        let transcode_decode_flag_size = std::mem::size_of::<TranscodeDecodeFlags>();
        assert_eq!(
            transcode_decode_flag_size & (transcode_decode_flag_size - 1),
            0
        );

        let pixel_manipulation_size = std::mem::size_of::<PixelManipulation>();
        assert_eq!(pixel_manipulation_size & (pixel_manipulation_size - 1), 0);

        let transcode_desc_size = std::mem::size_of::<TranscodeDescriptor>();
        assert_eq!(transcode_desc_size & (transcode_desc_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let basis_compression_algorithm_alignment = std::mem::align_of::<BasisCompressionFormat>();
        assert_eq!(
            basis_compression_algorithm_alignment & (basis_compression_algorithm_alignment - 1),
            0
        );

        let compression_quality_alignment = std::mem::align_of::<CompressionQuality>();
        assert_eq!(
            compression_quality_alignment & (compression_quality_alignment - 1),
            0
        );

        let mipmap_desc_alignment = std::mem::align_of::<MipmapDescriptor>();
        assert_eq!(mipmap_desc_alignment & (mipmap_desc_alignment - 1), 0);

        let rdo_desc_alignment = std::mem::align_of::<RDODescriptor>();
        assert_eq!(rdo_desc_alignment & (rdo_desc_alignment - 1), 0);

        let user_data_alignment = std::mem::align_of::<UserData>();
        assert_eq!(user_data_alignment & (user_data_alignment - 1), 0);

        let compression_desc_alignment = std::mem::align_of::<CompressionDescriptor>();
        assert_eq!(
            compression_desc_alignment & (compression_desc_alignment - 1),
            0
        );

        let basis_texture_alignment = std::mem::align_of::<BasisTexture>();
        assert_eq!(basis_texture_alignment & (basis_texture_alignment - 1), 0);

        let transcode_source_alignment = std::mem::align_of::<TranscodeSource>();
        assert_eq!(
            transcode_source_alignment & (transcode_source_alignment - 1),
            0
        );

        let transcode_texture_format_alignment = std::mem::align_of::<TranscodeTextureFormat>();
        assert_eq!(
            transcode_texture_format_alignment & (transcode_texture_format_alignment - 1),
            0
        );

        let transcode_decode_flag_alignment = std::mem::align_of::<TranscodeDecodeFlags>();
        assert_eq!(
            transcode_decode_flag_alignment & (transcode_decode_flag_alignment - 1),
            0
        );

        let pixel_manipulation_alignment = std::mem::align_of::<PixelManipulation>();
        assert_eq!(
            pixel_manipulation_alignment & (pixel_manipulation_alignment - 1),
            0
        );

        let transcode_desc_alignment = std::mem::align_of::<TranscodeDescriptor>();
        assert_eq!(transcode_desc_alignment & (transcode_desc_alignment - 1), 0);
    }
}
