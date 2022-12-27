use crate::color::component::ACESCompression;
use crate::color::{
    aces_compression_internal, aces_uncompressed_internal, find_gamut_intersection, oklab_to_srgb,
    srgb_to_oklab,
};
use fabled_math::vector_math::{abs, component_max, gt, length, lt, min, select};
use fabled_math::{approximate_equal, Bool3, Vector3, Vector4};

pub fn aces_gamut_compression(
    aces2025: Vector3,
    aces_param: ACESCompression,
    invert: bool,
) -> Vector3 {
    // threshold is the percentage of the core gamut to protect we want to clamp
    // cmy, but don't really care about the power.
    let threshold_cmy = Vector4 {
        value: min(
            Vector4::set(0.9999, 0.9999, 0.9999, aces_param.threshold_power.w()).value,
            aces_param.threshold_power.value,
        ),
    };

    // limit is the max distance from gamut body that will be compressed.
    let limit_cmy = aces_param.limit + 1.0;

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
    let compressed_distance = aces_compression_internal(distance, limit_cmy, threshold_cmy);


    // uncompress distance with user controlled parameterized shaper function
    let uncompressed_distance =
        aces_uncompressed_internal(compressed_distance, limit_cmy, threshold_cmy);

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

// alpha is defaulted to 0.05.
pub fn gamut_clip_adaptive_l0_0_5(rgb: Vector3, alpha: f32) -> Vector3 {
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
    let ab = lab * Vector3::set(0.0, 1.0, 1.0);
    
    let length = length(ab.value).max(f32::EPSILON);
    let rcp_length = length.recip();

    let lab_ = lab * Vector3::set(1.0, rcp_length, rcp_length);

    let ld = lab_.x() - 0.5;
    let ld_abs = ld.abs();
    let e1 = (0.5 + ld_abs) + (alpha * length);

    let c = ((e1 * e1) - (ld_abs + ld_abs)).sqrt();
    let l0 = 0.5 * (1.0 + ld.signum() * (e1 - c));

    let t = find_gamut_intersection(lab_, length, l0);

    let d = 1.0 - t;
    let l_clipped = (l0 * d) + (t * lab_.x());
    let c_clipped = t * length;

    let a = c_clipped * lab_.y();
    let b = c_clipped * lab_.z();

    oklab_to_srgb(Vector3::set(l_clipped, a, b))
}
