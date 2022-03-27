#[derive(Copy, Clone)]
pub enum FunctionType {
    Standard,
    Coroutine,
}


impl Default for FunctionType {
    fn default() -> Self {
        Self::Standard
    }
}
