use crate::Vector4;

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq)]
pub struct EulerOrder(pub(crate) Vector4);

impl Default for EulerOrder {
    fn default() -> Self {
        EulerOrder::XYZ
    }
}

impl EulerOrder {
    pub const XYZ: EulerOrder = EulerOrder {
        0: Vector4::set(1.0, -1.0, 1.0, -1.0),
    };
    pub const YXZ: EulerOrder = EulerOrder {
        0: Vector4::set(1.0, -1.0, -1.0, 1.0),
    };
    pub const ZXY: EulerOrder = EulerOrder {
        0: Vector4::set(-1.0, 1.0, 1.0, -1.0),
    };
    pub const ZYX: EulerOrder = EulerOrder {
        0: Vector4::set(-1.0, 1.0, -1.0, 1.0),
    };
    pub const YZX: EulerOrder = EulerOrder {
        0: Vector4::set(1.0, 1.0, -1.0, -1.0),
    };
    pub const XZY: EulerOrder = EulerOrder {
        0: Vector4::set(-1.0, -1.0, 1.0, 1.0),
    };
}