#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum LuaSafetySate {
    Unsafe,
    Safe,
}
