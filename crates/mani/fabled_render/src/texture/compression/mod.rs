use crate::texture::ColorSpace;

pub mod compress;
pub use compress::*;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum ThreadOperation {
    /// Single threaded execution that runs a operation in the current thread.
    Single,
    /// Multiple threaded execution that runs multiple operations depending
    /// on the number of CPUs available on the current system.
    Automatic,

    /// Multiple threaded or single threaded execution that run a operation in the
    /// current thread if it is single else it will use the user defined number of threads to use.
    Custom { thread_amount: u32 },
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

#[allow(dead_code)]
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
    //try to keep it to the power of two.
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
    /// Enable/disable UASTC RDO post-processing and set UASTC RDO quality scalar to X. Lower
    /// values=higher quality/larger LZ compressed files, higher values=lower quality/smaller LZ
    /// compressed files. Good range to try is [.2-4]
    pub rdo_uastc_quality_scalar: Option<f32>,

    /// Disable backend's endpoint rate distortion optimizations (slightly faster, less noisy
    /// output, but lower quality per output bit)
    pub no_endpoint_rdo: bool,

    /// Disable backend's selector rate distortion optimizations (slightly faster, less noisy
    /// output, but lower quality per output bit)
    pub no_selector_rdo: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct CompressionDescriptor {
    pub compression_format: BasisCompressionFormat,
    pub compression_quality: CompressionQuality,
    pub color_space: ColorSpace,
    pub mip_map_desc: Option<MipmapDescriptor>,
    pub rdo_desc: Option<RDODescriptor>,
    //pub user_data : Option<UserData>,
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

#[cfg(test)]
mod data_alignment_test {
    use crate::texture::compression::{
        BasisCompressionFormat, BasisTexture, CompressionDescriptor, CompressionQuality,
        MipmapDescriptor, RDODescriptor, ThreadOperation, UserData,
    };

    #[test]
    fn data_alignment() {
        //Test for data alignment.
        let basis_compression_algorithm = std::mem::size_of::<BasisCompressionFormat>();
        assert_eq!(
            basis_compression_algorithm & (basis_compression_algorithm - 1),
            0
        );

        let compression_quality = std::mem::size_of::<CompressionQuality>();
        assert_eq!(compression_quality & (compression_quality - 1), 0);

        let mipmap_desc = std::mem::size_of::<MipmapDescriptor>();
        assert_eq!(mipmap_desc & (mipmap_desc - 1), 0);

        let rdo_desc = std::mem::size_of::<RDODescriptor>();
        assert_eq!(rdo_desc & (rdo_desc - 1), 0);

        let user_data = std::mem::size_of::<UserData>();
        assert_eq!(user_data & (user_data - 1), 0);

        let compression_desc = std::mem::size_of::<CompressionDescriptor>();
        assert_eq!(compression_desc & (compression_desc - 1), 0);

        let thread_operation = std::mem::size_of::<ThreadOperation>();
        assert_eq!(thread_operation & (thread_operation - 1), 0);

        let basis_texture = std::mem::size_of::<BasisTexture>();
        assert_eq!(basis_texture & (basis_texture - 1), 0);
    }
}
