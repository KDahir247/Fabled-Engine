use crate::color::component::ACESCompression;
use crate::color::{
    aces_compression_internal, aces_uncompressed_internal, find_gamut_intersection, oklab_to_srgb,
    srgb_to_oklab,
};
use fabled_math::vector_math::{abs, component_max, gt, lt, min, select};
use fabled_math::{approximate_equal, Bool3, Vector3};

pub fn aces_gamut_compression(
    aces2025: Vector3,
    aces_param: ACESCompression,
    invert: bool,
) -> Vector3 {
    // threshold is the percentage of the core gamut to protect
    let threshold_cmy = Vector3 {
        value: min(
            Vector3::broadcast(0.9999).value,
            aces_param.threshold_cmy.value,
        ),
    };

    // limit is the max distance from gamut body that will be compressed.
    let limit_cmy = aces_param.limit_cmy + 1.0;

    // achromatic axis
    let achromatic = Vector3::broadcast(component_max(aces2025.value));

    let zero_mask = Bool3::broadcast(approximate_equal(achromatic.x(), 0.0, f32::EPSILON));

    // achromatic zero check.
    let valid_achromatic = Vector3 {
        value: select(Vector3::ONE.value, achromatic.value, zero_mask.value),
    };

    // distance from the achromatic axis for each color component
    let distance = Vector3 {
        value: select(
            Vector3::ZERO.value,
            (valid_achromatic - aces2025).value / abs(valid_achromatic.value),
            zero_mask.value,
        ),
    };

    let mask = Bool3::broadcast(!invert);

    // compress distance with user controlled parameterized shaper function
    let compressed_distance = Vector3::set(
        aces_compression_internal(
            distance.x(),
            limit_cmy.x(),
            threshold_cmy.x(),
            aces_param.power,
        ),
        aces_compression_internal(
            distance.y(),
            limit_cmy.y(),
            threshold_cmy.y(),
            aces_param.power,
        ),
        aces_compression_internal(
            distance.z(),
            limit_cmy.z(),
            threshold_cmy.z(),
            aces_param.power,
        ),
    );

    // uncompress distance with user controlled parameterized shaper function
    let uncompressed_distance = Vector3::set(
        aces_uncompressed_internal(
            distance.x(),
            limit_cmy.x(),
            threshold_cmy.x(),
            aces_param.power,
        ),
        aces_uncompressed_internal(
            distance.y(),
            limit_cmy.y(),
            threshold_cmy.y(),
            aces_param.power,
        ),
        aces_uncompressed_internal(
            distance.z(),
            limit_cmy.z(),
            threshold_cmy.z(),
            aces_param.power,
        ),
    );

    let desired_distance = Vector3 {
        value: select(
            compressed_distance.value,
            uncompressed_distance.value,
            mask.value,
        ),
    };

    let compressed_rgb = achromatic
        - desired_distance
            * Vector3 {
                value: abs(achromatic.value),
            };

    // we can convert this back to ACEScct or ACEScc depending on choice.
    compressed_rgb
}

pub fn gamut_clip_adaptive_l0_0_5(rgb: Vector3, alpha: Option<f32>) -> Vector3 {
    let alpha = alpha.unwrap_or(0.05);

    let normalized_min = Bool3 {
        value: lt(rgb.value, Vector3::broadcast(1.0).value),
    };
    let normalized_max = Bool3 {
        value: gt(rgb.value, Vector3::broadcast(0.0).value),
    };

    if normalized_min.all() & normalized_max.all() {
        return rgb;
    }

    let lab = srgb_to_oklab(rgb);

    let l = lab.x();

    let length = ((lab.y() * lab.y()) + (lab.z() * lab.z()))
        .sqrt()
        .max(f32::EPSILON);

    let rcp_length = length.recip();

    let a_ = lab.y() * rcp_length;
    let b_ = lab.z() * rcp_length;

    let ld = l - 0.5;
    let ld_abs = ld.abs();
    let e1 = (0.5 + ld_abs) + (alpha * length);

    let c = ((e1 * e1) - (ld_abs + ld_abs)).sqrt();
    let l0 = 0.5 * (1.0 + ld.signum() * (e1 - c));

    let t = find_gamut_intersection(a_, b_, l, length, l0);

    let d = 1.0 - t;
    let l_clipped = (l0 * d) + (t * l);
    let c_clipped = t * length;

    let a = c_clipped * a_;
    let b = c_clipped * b_;

    oklab_to_srgb(Vector3::set(l_clipped, a, b))
}
