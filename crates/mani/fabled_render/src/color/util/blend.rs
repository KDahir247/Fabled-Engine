use fabled_math::vector_math::{abs, component_le, component_min, cos, pow, select, sqrt};
use fabled_math::Vector3;

pub fn create_palette(time: f32, a: Vector3, b: Vector3, c: Vector3, d: Vector3) -> Vector3 {
    const TAU_VEC3: Vector3 = Vector3::broadcast(std::f32::consts::TAU);

    let e = TAU_VEC3 * (c * time + d);

    a + b * Vector3 {
        value: cos(e.value),
    }
}

// todo rework the blending with opacity.

pub fn multiply_blend(a: Vector3, b: Vector3) -> Vector3 {
    a * b
}

pub fn screen_blend(a: Vector3, b: Vector3) -> Vector3 {
    const ONE_VEC: Vector3 = Vector3::broadcast(1.0);

    let a = (ONE_VEC - a);
    let b = (ONE_VEC - b);
    1.0 - a * b
}

pub fn darken_blend(a: Vector3, b: Vector3) -> Vector3 {
    let min_r = a.x().min(b.x());
    let min_g = a.y().min(b.y());
    let min_b = a.z().min(b.z());

    Vector3::set(min_r, min_g, min_b)
}

pub fn lighten_blend(a: Vector3, b: Vector3) -> Vector3 {
    let max_r = a.x().max(b.x());
    let max_g = a.y().max(b.y());
    let max_b = a.z().max(b.z());

    Vector3::set(max_r, max_g, max_b)
}

pub fn burn_blend(a: Vector3, b: Vector3, opacity: f32) -> Vector3 {
    1.0 - ((1.0 - a) / (1.0 + opacity * b - opacity))
}


pub fn hardlight_blend(a: Vector3, b: Vector3) -> Vector3 {
    let mul = multiply_blend(a, b * 2.0);

    let screen = screen_blend(a, b * 2.0 - 1.0);

    let mask = component_le(b.value, 0.5);

    Vector3 {
        value: select(mul.value, screen.value, mask),
    }
}

pub fn overlay_blend(a: Vector3, b: Vector3) -> Vector3 {
    hardlight_blend(b, a)
}

pub fn color_dodge_blend(a: Vector3, b: Vector3) -> Vector3 {
    let mut target = Vector3::broadcast(0.0);

    if a == Vector3::ZERO {
        target = Vector3::broadcast(0.0);
    } else if b == Vector3::ONE {
        target = Vector3::broadcast(1.0);
    } else {
        let denometer = Vector3::ONE - b;

        let dodge_r = a.x() * denometer;
        let dodge_g = a.y() * denometer;
        let dodge_b = a.z() * denometer;

        target = Vector3::set(dodge_r, dodge_g, dodge_b);
    }

    target
}

pub fn soft_light_blend(a: Vector3, b: Vector3) -> Vector3 {
    const HALF_VEC: Vector3 = Vector3::broadcast(0.5);

    let blend_func = if a <= 0.25 {
        ((Vector3::broadcast(16.0) * a - Vector3::broadcast(12.0)) * a + Vector3::broadcast(4.0))
            * a
    } else {
        Vector3 {
            value: sqrt(a.value),
        }
    };

    if b <= 0.5 {
        a - (Vector3::broadcast(1.0) - Vector3::broadcast(2.0) * b)
            * a
            * (Vector3::broadcast(1.0) - a)
    } else {
        a + (Vector3::broadcast(2.0) * b - Vector3::broadcast(1.0)) * (blend_func - a)
    }
}

pub fn difference_blend(a: Vector3, b: Vector3) -> Vector3 {
    Vector3 { value: abs(a - b) }
}
