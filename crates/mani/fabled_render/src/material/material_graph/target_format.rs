use serde::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MaterialTargetFormat {
    Undefined,
    UnsignedInt,
    SignedInt,
    Float,
    Boolean,

    Vector2UnsignedInt,
    Vector2SignedInt,
    Vector2Float,
    Vector2Boolean,

    Vector4UnsignedInt,
    Vector4SignedInt,
    Vector4Float,
    Vector4Boolean,

    Matrix2x2Float,
    Matrix4x4Float,

    Sampler,
    Texture,
}
