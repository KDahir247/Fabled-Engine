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

impl From<LoadFlags> for tobj::LoadOptions {
    fn from(flags: LoadFlags) -> Self {
        let flag_bit = flags.bits;

        Self {
            merge_identical_points: (flag_bit & 2) == 2,
            reorder_data: (flag_bit & 4) == 4,
            single_index: (flag_bit & 8) == 8,
            triangulate: (flag_bit & 16) == 16,
            ignore_points: (flag_bit & 32) == 32,
            ignore_lines: (flag_bit & 64) == 64,
        }
    }
}

#[cfg(test)]
mod load_flag_test {
    use crate::LoadFlags;

    #[test]
    fn creation_test() {
        let load_flag = LoadFlags::default();
        assert_eq!(
            load_flag,
            LoadFlags::SINGLE_INDEX
                | LoadFlags::TRIANGULATE
                | LoadFlags::IGNORE_POINTS
                | LoadFlags::IGNORE_LINES
        );
    }


    #[test]
    fn from_test() {
        let load_flag = LoadFlags::from_bits(106).unwrap();
        // MERGE_IDENTICAL_POINTS | SINGLE_INDEX | IGNORE_POINTS | IGNORE_LINES
        let load_option: tobj::LoadOptions = load_flag.into();

        assert!(load_option.merge_identical_points);
        assert!(load_option.ignore_lines);
        assert!(load_option.ignore_points);
        assert!(load_option.single_index);

        assert!(!load_option.reorder_data);
        assert!(!load_option.triangulate);
    }
}
