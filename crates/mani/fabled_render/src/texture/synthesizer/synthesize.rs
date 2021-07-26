use fabled_core::thread_op::ThreadOperation;

#[repr(align(256))]
pub struct SessionBuilder<'a> {
    session_builder: texture_synthesis::SessionBuilder<'a>,
}

impl<'a> SessionBuilder<'a> {
    pub fn create_builder(thread_op: ThreadOperation) -> SessionBuilder<'a> {
        let num_thread = thread_op.into();

        Self {
            session_builder: texture_synthesis::Session::builder().max_thread_count(num_thread),
        }
    }

    //all the rest take a mutable self and return it.

    pub fn build(self) -> texture_synthesis::Session {
        self.session_builder.build().unwrap()
    }
}
