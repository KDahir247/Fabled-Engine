use fabled_math::vector_math::{abs, cos, ge, gt, le, max, min, select, sqrt};
use fabled_math::{Bool3, Vector3};

pub fn create_palette(time: f32, a: Vector3, b: Vector3, c: Vector3, d: Vector3) -> Vector3 {
    const TAU_VEC3: Vector3 = Vector3::broadcast(std::f32::consts::TAU);

    let e = TAU_VEC3 * (c * time + d);

    a + b * Vector3 {
        value: cos(e.value),
    }
}

pub fn add_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let add_blend = Vector3 {
        value: min((a + b).value, Vector3::ONE.value),
    };

    add_blend * opacity + a * (1.0 - opacity)
}


pub fn average_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let average_blend = (a + b) * 0.5;

    average_blend * opacity + a * (1.0 - opacity)
}

pub fn color_burn_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let burn_intermediate = Vector3 {
        value: max(
            (Vector3::ONE - ((Vector3::ONE - a) / b)).value,
            Vector3::ZERO.value,
        ),
    };

    let color_burn = burn_intermediate * opacity + a * (1.0 - opacity);

    Vector3 {
        value: select(b.value, color_burn.value, le(b.value, Vector3::ZERO.value)),
    }
}

pub fn color_dodge_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let dodge_intermediate = Vector3 {
        value: min((a / (Vector3::ONE - b)).value, Vector3::ONE.value),
    };

    let color_dodge = dodge_intermediate * opacity + a * (1.0 - opacity);

    Vector3 {
        value: select(b.value, color_dodge.value, ge(b.value, Vector3::ONE.value)),
    }
}


pub fn darken_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: min(a.value, b.value),
    } * opacity
        + a * (1.0 - opacity)
}

pub fn lighten_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {}
