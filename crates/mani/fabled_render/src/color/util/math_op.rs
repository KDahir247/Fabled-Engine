use fabled_math::{Vector2, Vector3};

pub fn saturate(val: f32) -> f32 {
    let rhs = val + 1.0;
    let lhs = val - 1.0;

    let temp = rhs - lhs.abs();
    (temp + temp.abs()) * 0.25
}


pub fn chromatic_coord_to_tri_stimulus_white(chromaticity_coord: Vector2) -> Vector3 {
    let denometer = chromaticity_coord.y().recip();

    Vector3::set(
        chromaticity_coord.x() * denometer,
        1.0,
        (1.0 - chromaticity_coord.x() - chromaticity_coord.y()) * denometer,
    )
}

pub fn tri_stimulus_white_to_chromatic_coord(chromatic_coord: Vector3) -> Vector2 {
    let denometer = (chromatic_coord.x() + chromatic_coord.y() + chromatic_coord.z()).recip();
    Vector2::set(
        chromatic_coord.x() * denometer,
        chromatic_coord.y() * denometer,
    )
}
