use bitflags::bitflags;


bitflags! {
    pub struct ParseCommand : u8{
        const BLEND_TEXTURE_SUPPORT = 2;
        const BOOST_FLOAT_VALUE = 4;
    }
}
