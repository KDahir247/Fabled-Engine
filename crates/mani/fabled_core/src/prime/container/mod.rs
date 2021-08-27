pub mod bytes;
pub mod wrapper;

#[cfg(test)]
mod data_test {
    use crate::prime::container::wrapper::Wrapper;

    #[test]
    fn data_size() {
        let wrapper_u8_size = std::mem::size_of::<Wrapper<u8>>();
        println!("{}", wrapper_u8_size);

        let wrapper_u16_size = std::mem::size_of::<Wrapper<u16>>();
        println!("{}", wrapper_u16_size);

        let wrapper_u32_size = std::mem::size_of::<Wrapper<u32>>();
        println!("{}", wrapper_u32_size);

        let wrapper_u64_size = std::mem::size_of::<Wrapper<u64>>();
        println!("{}", wrapper_u64_size);

        let wrapper_u128_size = std::mem::size_of::<Wrapper<u128>>();
        println!("{}", wrapper_u128_size);

        let wrapper_usize_size = std::mem::size_of::<Wrapper<usize>>();
        println!("{}", wrapper_usize_size);
    }

    #[test]
    fn data_alignment() {
        let wrapper_u8_alignment = std::mem::align_of::<Wrapper<u8>>();
        println!("{}", wrapper_u8_alignment);

        let wrapper_u16_alignment = std::mem::align_of::<Wrapper<u16>>();
        println!("{}", wrapper_u16_alignment);

        let wrapper_u32_alignment = std::mem::align_of::<Wrapper<u32>>();
        println!("{}", wrapper_u32_alignment);

        let wrapper_u64_alignment = std::mem::align_of::<Wrapper<u64>>();
        println!("{}", wrapper_u64_alignment);

        let wrapper_u128_alignment = std::mem::align_of::<Wrapper<u128>>();
        println!("{}", wrapper_u128_alignment);

        let wrapper_usize_alignment = std::mem::align_of::<Wrapper<usize>>();
        println!("{}", wrapper_usize_alignment);
    }
}
