use core::arch::x86_64::*;
//todo move max_ss, min_ss, and clamp_ss to fabled_math when possible.

pub fn transpose_4x4_array<T: Copy>(
    input: &[T; 16],
    output: &mut [T; 16],
    width: usize,
    height: usize,
) {
    //Note this is a internal implementation and should not be considered to be public.
    for y in 0..height {
        for x in 0..width {
            let input_index = y + x * width;
            let output_index = x + y * height;

            *output.get_mut(output_index).expect("") = *input.get(input_index).expect("");
        }
    }
}

pub fn min_ss(mut a: f32, b: f32) -> f32 {
    #[cfg(target_feature = "sse2")]
    unsafe {
        /*
         1. Store Single Scalar Floating Point (MOVSS) to a
         2. MINSS instruction. Compare the low single precision floating-point value to a, b.
            If a is less then it returns a otherwise b is copied to a.
         3. Converting f32 to __m128 128-bit wide set of four `f32` types on the lowest set and zero out the other element.
            eg. (value, 0.0, 0.0, 0.0)
        */
        _mm_store_ss(&mut a, _mm_min_ss(_mm_set_ss(a), _mm_set_ss(b)));
        return a;
    }

    //Could do >> 1, but doesn't work on float point type unless casting to integer type which
    (a + b - (a - b).abs()) * 0.5
}

pub fn max_ss(mut a: f32, b: f32) -> f32 {
    #[cfg(all(target_feature = "sse2"))]
    unsafe {
        /*
         1. Store Single Scalar Floating Point (MOVSS) to a
         2. MAXSS instruction. Compare the low single precision floating-point value to a, b.
            If a is greater then it returns a otherwise b is copied to a.
         3. Converting f32 to __m128 128-bit wide set of four `f32` types on the lowest set and zero out the other element.
            eg. (value, 0.0, 0.0, 0.0)
        */
        _mm_store_ss(&mut a, _mm_max_ss(_mm_set_ss(a), _mm_set_ss(b)));
        return a;
    }

    //Could do >> 1, but doesn't work on float point type unless casting to integer type which
    (a + b + (a - b).abs()) * 0.5
}

pub fn clamp_ss(mut value: f32, min: f32, max: f32) -> f32 {
    #[cfg(all(target_feature = "sse2"))]
    unsafe {
        /*
        1. Store Single Scalar Floating Point (MOVSS) to value
        2. MAXSS instruction. Compare the low single precision floating-point value to max, max(min, min_instruction()(3)).
        3. MINSS instruction. Compare the low single precision floating-point value to min, min(max,value)
        4. Set_SS (set single scalar floating point) Converting f32 to __m128 128-bit wide set of four `f32` types on the lowest set and zero out the other element.
            eg. (value, 0.0, 0.0, 0.0)
        */
        _mm_store_ss(
            &mut value,
            _mm_max_ss(
                _mm_set_ss(min),
                _mm_min_ss(_mm_set_ss(max), _mm_set_ss(value)),
            ),
        );
        return value;
    }

    //Call in-order to perform saturation of min and max to value.
    max_ss(min, min_ss(max, value))
}
