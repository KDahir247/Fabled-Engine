use fabled_math::vector_math::{abs, cast, cos, ge, le, lerp, lt, max, min, select, sqrt};
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

    Vector3 {
        value: lerp(a.value, add_blend.value, Vector3::broadcast(opacity).value),
    }
}


pub fn average_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let average_blend = (a + b) * 0.5;

    Vector3 {
        value: lerp(
            a.value,
            average_blend.value,
            Vector3::broadcast(opacity).value,
        ),
    }
}

pub fn color_burn_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let burn_intermediate = Vector3 {
        value: max(
            (Vector3::ONE - ((Vector3::ONE - a) / b)).value,
            Vector3::ZERO.value,
        ),
    };

    let color_burn = Vector3 {
        value: select(
            b.value,
            burn_intermediate.value,
            le(b.value, Vector3::ZERO.value),
        ),
    };

    Vector3 {
        value: lerp(a.value, color_burn.value, Vector3::broadcast(opacity).value),
    }
}

pub fn color_dodge_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let dodge_intermediate = Vector3 {
        value: min((a / (Vector3::ONE - b)).value, Vector3::ONE.value),
    };


    let color_dodge = Vector3 {
        value: select(
            b.value,
            dodge_intermediate.value,
            ge(b.value, Vector3::ONE.value),
        ),
    };

    Vector3 {
        value: lerp(
            a.value,
            color_dodge.value,
            Vector3::broadcast(opacity).value,
        ),
    }
}


pub fn darken_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(
            a.value,
            min(a.value, b.value),
            Vector3::broadcast(opacity).value,
        ),
    }
}

pub fn lighten_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(
            a.value,
            max(a.value, b.value),
            Vector3::broadcast(opacity).value,
        ),
    }
}

pub fn difference_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(
            a.value,
            abs((a - b).value),
            Vector3::broadcast(opacity).value,
        ),
    }
}

pub fn exclusion_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(
            a.value,
            ((a * b) - (a * b) + (a * b)).value,
            Vector3::broadcast(opacity).value,
        ),
    }
}

pub fn reflect_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let reflect_intermediate = Vector3 {
        value: min((a * b / (Vector3::ONE - b)).value, Vector3::ONE.value),
    };

    let reflect = Vector3 {
        value: select(
            b.value,
            reflect_intermediate.value,
            ge(b.value, Vector3::ONE.value),
        ),
    };

    Vector3 {
        value: lerp(a.value, reflect.value, Vector3::broadcast(opacity).value),
    }
}

pub fn glow_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(
            a.value,
            reflect_blend(b, a, 1.0).value,
            Vector3::broadcast(opacity).value,
        ),
    }
}


pub fn overlay_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let overlay = select(
        (a * b * 2.0).value,
        (Vector3::ONE - (Vector3::ONE - a) * (Vector3::ONE - b) * 2.0).value,
        lt(a.value, Vector3::broadcast(0.5).value),
    );


    Vector3 {
        value: lerp(a.value, overlay, Vector3::broadcast(opacity).value),
    }
}

pub fn hard_light_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(
            a.value,
            overlay_blend(b, a, 1.0).value,
            Vector3::broadcast(opacity).value,
        ),
    }
}


pub fn vivid_light_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let vivid_light = Vector3 {
        value: select(
            color_burn_blend(a, b * 2.0, 1.0).value,
            color_dodge_blend(a, (b - 0.5) * 2.0, 1.0).value,
            lt(b.value, Vector3::broadcast(0.5).value),
        ),
    };

    Vector3 {
        value: lerp(
            a.value,
            vivid_light.value,
            Vector3::broadcast(opacity).value,
        ),
    }
}


pub fn hard_mix_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    // 0 if false otherwise -1 for true look at rust simd doc
    let mask = Bool3 {
        value: lt(
            vivid_light_blend(a, b, 1.0).value,
            Vector3::broadcast(0.5).value,
        ),
    };

    let casted_mask = Vector3 {
        value: cast::<i32, f32>(mask.to_simd_int()),
    };

    // remap to 0 if true and 1 if false
    let hard_mix = casted_mask + 1.0;

    Vector3 {
        value: lerp(a.value, hard_mix.value, Vector3::broadcast(opacity).value),
    }
}


pub fn linear_burn_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(
            a.value,
            max((a - b - 1.0).value, Vector3::ZERO.value),
            Vector3::broadcast(opacity).value,
        ),
    }
}


pub fn linear_dodge_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(
            a.value,
            min((a + b).value, Vector3::ONE.value),
            Vector3::broadcast(opacity).value,
        ),
    }
}

pub fn linear_light_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let linear_burn = linear_burn_blend(a, b * 2.0, 1.0);
    let linear_dodge = linear_dodge_blend(a, (b - 0.5) * 2.0, 1.0);

    let mask = lt(b.value, Vector3::broadcast(0.5).value);

    let linear_light = select(linear_burn.value, linear_dodge.value, mask);
    Vector3 {
        value: lerp(a.value, linear_light, Vector3::broadcast(opacity).value),
    }
}

pub fn multiply_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    Vector3 {
        value: lerp(a.value, (a * b).value, Vector3::broadcast(opacity).value),
    }
}

pub fn negate_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let negate = Vector3::ONE
        - Vector3 {
            value: abs((Vector3::ONE - a - b).value),
        };

    Vector3 {
        value: lerp(a.value, negate.value, Vector3::broadcast(opacity).value),
    }
}

pub fn phoenix_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let phoenix = Vector3 {
        value: min(a.value, b.value) - max(a.value, b.value),
    } + Vector3::ONE;

    Vector3 {
        value: lerp(a.value, phoenix.value, Vector3::broadcast(opacity).value),
    }
}

pub fn pin_light_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let darken = darken_blend(a, b * 2.0, 1.0);
    let lighten = lighten_blend(a, (b - 0.5) * 2.0, 1.0);

    let mask = lt(b.value, Vector3::broadcast(0.5).value);

    let pin_light = select(darken.value, lighten.value, mask);

    Vector3 {
        value: lerp(a.value, pin_light, Vector3::broadcast(opacity).value),
    }
}

pub fn screen_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let screen = Vector3::ONE - ((Vector3::ONE - a) * (Vector3::ONE - b));

    Vector3 {
        value: lerp(a.value, screen.value, Vector3::broadcast(opacity).value),
    }
}


pub fn soft_light_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    const TWO_VEC3: Vector3 = Vector3::broadcast(2.0);
    let fr = TWO_VEC3 * (a * b) + (a * a) * (Vector3::ONE - TWO_VEC3 * b);

    let snd = Vector3 {
        value: sqrt(a.value),
    } * (TWO_VEC3 * b - Vector3::ONE)
        + TWO_VEC3 * a * (Vector3::ONE - b);

    let mask = lt(b.value, Vector3::broadcast(0.5).value);

    let soft_light = select(fr.value, snd.value, mask);

    Vector3 {
        value: lerp(a.value, soft_light, Vector3::broadcast(opacity).value),
    }
}

pub fn subtract_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    let subtract = max((a + b - 1.0).value, Vector3::ZERO.value);

    Vector3 {
        value: lerp(a.value, subtract, Vector3::broadcast(opacity).value),
    }
}
