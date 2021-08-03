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
