#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StorageTextureAccess {
    Readonly,
    WriteOnly,
    ReadWrite,
}
