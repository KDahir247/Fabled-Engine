use fabled_math::vector_math::cos;
use fabled_math::Vector3;

pub fn create_palette(time: f32, a: Vector3, b: Vector3, c: Vector3, d: Vector3) -> Vector3 {
    const TAU_VEC3: Vector3 = Vector3::broadcast(std::f32::consts::TAU);

    let e = TAU_VEC3 * (c * time + d);

    a + b * Vector3 {
        value: cos(e.value),
    }
}
