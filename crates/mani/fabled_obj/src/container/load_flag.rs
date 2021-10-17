use bitflags::bitflags;

bitflags! {
    pub struct LoadFlags : u8{

        const MERGE_IDENTICAL_POINTS = 2;
        const REORDER_DATA = 4;
        const SINGLE_INDEX = 8;
        const TRIANGULATE = 16;
        const IGNORE_POINTS = 32;
        const IGNORE_LINES = 64;
    }
}

impl Default for LoadFlags {
    fn default() -> Self {
        Self::from_bits(120).unwrap_or_default()
    }
}

impl LoadFlags {}

#[cfg(test)]
mod load_flag_test {
    use crate::LoadFlags;

    #[test]
    fn creation_test() {
        let load_flag = LoadFlags::default();
        println!("{:?}", load_flag);
    }
}
