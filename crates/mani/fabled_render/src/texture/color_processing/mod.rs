mod color_proc;

pub use color_proc::*;

#[cfg(test)]
mod color_proc_tests {
    use crate::texture::ColorProcessing;

    #[test]
    fn data_size() {
        let color_proc_size = std::mem::size_of::<ColorProcessing>();
        assert_eq!(color_proc_size & (color_proc_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let color_proc_alignment = std::mem::align_of::<ColorProcessing>();
        assert_eq!(color_proc_alignment & (color_proc_alignment - 1), 0);
    }
}
