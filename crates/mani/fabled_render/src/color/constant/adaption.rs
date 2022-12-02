use fabled_math::{Matrix3x3, Vector3};

pub const XYZ_SCALING: Matrix3x3 = Matrix3x3::IDENTITY;

pub const VON_KRIES: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.4002400, -0.2263000, 0.0),
    Vector3::set(0.7076000, 1.1653200, 0.0),
    Vector3::set(-0.0808100, 0.0457000, 0.9182200),
);

pub const BRADFORD: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.8951, -0.7502, 0.0389),
    Vector3::set(0.2664, 1.7136, -0.0685),
    Vector3::set(-0.1614, 0.0367, 1.0296),
);

pub const CIECAM02: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.7328, -0.7036, 0.0030),
    Vector3::set(0.4296, 1.6975, 0.0136),
    Vector3::set(-0.1624, 0.0061, 0.9834),
);
