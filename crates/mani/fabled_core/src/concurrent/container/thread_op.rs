#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum ThreadOperation {
    /// Single threaded execution that runs a operation in the current thread.
    Single,
    /// Multiple threaded execution that runs multiple operations depending
    /// on the number of CPUs available on the current system.
    Automatic,

    /// Multiple threaded or single threaded execution that run a operation in
    /// the current thread if it is single else it will use the user defined
    /// number of threads to use.
    Custom { thread_amount: usize },
}

impl From<ThreadOperation> for usize {
    fn from(thread_operation: ThreadOperation) -> usize {
        match thread_operation {
            ThreadOperation::Single => 1,
            ThreadOperation::Automatic => num_cpus::get(),
            ThreadOperation::Custom { thread_amount } => thread_amount,
        }
    }
}
