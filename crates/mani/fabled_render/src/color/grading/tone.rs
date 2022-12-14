use fabled_math::vector_math::{clamp, max, saturate};
use fabled_math::{Matrix3x3, Vector3};

// https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
pub fn aces_filmic_tonemap(color: Vector3) -> Vector3 {
    let aces_color = color * 0.6;

    let ldr_unclamped =
        (aces_color * (aces_color * 2.51 + 0.3)) / (aces_color * (aces_color * 2.43 + 0.59) + 0.14);

    Vector3 {
        value: clamp(ldr_unclamped.value, Vector3::ZERO.value, Vector3::ONE.value),
    }
}

// https://twitter.com/jimhejl/status/1137568973367783424/photo/1
pub fn filmic_alu_tonemap(color: Vector3) -> Vector3 {
    let color = Vector3 {
        value: max(Vector3::ZERO.value, (color - 0.004f32).value),
    };

    (color * (color * 6.2 + 0.5)) / (color * (color * 6.2 + 1.7) + 0.06)
}


const ACES_INPUT_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.59719, 0.07600, 0.02840),
    Vector3::set(0.35458, 0.90834, 0.13383),
    Vector3::set(0.04823, 0.01566, 0.83777),
);


const ACES_OUTPUT_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.60475, -0.10208, -0.00327),
    Vector3::set(-0.53108, 1.10813, -0.07276),
    Vector3::set(-0.07367, -0.00605, 1.07602),
);

fn rrt_and_odt_fit(v: Vector3) -> Vector3 {
    let a = v * (v + Vector3::broadcast(0.0245786)) - Vector3::broadcast(0.000090537);
    let b = v * (Vector3::broadcast(0.983729) * v + Vector3::broadcast(0.4329510))
        + Vector3::broadcast(0.238081);
    a / b
}

// https://github.com/TheRealMJP/BakingLab/blob/master/BakingLab/ACES.hlsl
pub fn aces_fitted_tonemap(color: Vector3) -> Vector3 {
    let color = ACES_OUTPUT_MATRIX * rrt_and_odt_fit(ACES_INPUT_MATRIX * color);

    Vector3 {
        value: saturate(color.value),
    }
}

// http://www.oscars.org/science-technology/sci-tech-projects/aces
pub fn aces_tonemap(color: Vector3) -> Vector3 {
    let color = ACES_INPUT_MATRIX * color;
    let a = color * (color + 0.0245786) - 0.000090537;
    let b = color * (color * 0.983729 + 0.4329510) + 0.238081;

    Vector3 {
        value: clamp((a / b).value, Vector3::ZERO.value, Vector3::ONE.value),
    }
}
