fn luminance(linear_col : vec3<f32>) -> f32{
    return dot(linear_col, vec3<f32>(0.2126, 0.7152, 0.02722));
}

fn compute_exposure_intensity(intensity : f32, exposure : f32) -> f32{
    return intensity * exposure;
}