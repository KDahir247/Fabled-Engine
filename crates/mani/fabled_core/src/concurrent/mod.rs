pub mod container;

pub use container::*;

#[cfg(test)]
mod data_alignment_test {
    use crate::concurrent::thread_op::ThreadOperation;

    #[test]
    fn data_alignment() {
        let thread_operation = std::mem::size_of::<ThreadOperation>();
        assert_eq!(thread_operation & (thread_operation - 1), 0);
    }
}
