use bitflags::bitflags;

bitflags! {
    pub struct LoadFlags : u8{

        const SINGLE_INDEX = 2;
        const TRIANGULATE = 4;
        const IGNORE_POINTS = 8;
        const IGNORE_LINES = 16;
    }
}

impl Default for LoadFlags {
    fn default() -> Self {
        Self::from_bits(30).unwrap_or_default()
    }
}

impl From<LoadFlags> for tobj::LoadOptions {
    fn from(flags: LoadFlags) -> Self {
        let flag_bit = flags.bits;

        Self {
            single_index: (flag_bit & 2) == 2,
            triangulate: (flag_bit & 4) == 4,
            ignore_points: (flag_bit & 8) == 8,
            ignore_lines: (flag_bit & 16) == 16,
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
        let load_flag = LoadFlags::from_bits(26).unwrap();
        // MERGE_IDENTICAL_POINTS | SINGLE_INDEX | IGNORE_POINTS | IGNORE_LINES
        let load_option: tobj::LoadOptions = load_flag.into();

        assert!(load_option.ignore_lines);
        assert!(load_option.ignore_points);
        assert!(load_option.single_index);

        assert!(!load_option.triangulate);
    }
}
