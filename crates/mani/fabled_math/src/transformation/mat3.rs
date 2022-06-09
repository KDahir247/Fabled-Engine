pub struct Matrix3x3 {
    pub value: std::simd::Simd<f32, 16>,
}

#[rustfmt::skip]
impl Default for Matrix3x3 {
    fn default() -> Self {
        Self {
            value: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                1.0, 1.0, 1.0, 1.0,
            ]
            .into(),
        }
    }
}

#[rustfmt::skip]
impl From<[f32; 16]> for Matrix3x3 {
    fn from(matrix: [f32; 16]) -> Self {
        
        let modified_matrix = 
            [
                matrix[0], matrix[1], matrix[2], 0.0,
                matrix[4], matrix[5], matrix[6], 0.0,
                matrix[8], matrix[9], matrix[10], 0.0,
                0.0, 0.0, 0.0, 0.0
            ];
        
        Self { value: modified_matrix.into() }
    }
}

#[rustfmt::skip]
impl From<[f32; 9]> for Matrix3x3 {
    fn from(matrix: [f32; 9]) -> Self {
        
        let modified_matrix = 
            [
                matrix[0], matrix[1], matrix[2], 0.0,
                matrix[3], matrix[4], matrix[5], 0.0,
                matrix[6], matrix[7], matrix[8], 0.0,
                0.0, 0.0, 0.0, 0.0
            ];
        
        Self {
            value: modified_matrix.into(),
        }
    }
}

pub struct SOAMatrix3x3<const N: usize> {
    pub value: [Matrix3x3; N],
}

#[test]
fn a() {
    let a = Matrix3x3::from([1.0, 8.0, 14.0, 4.0, 8.0, 6.0, 1.0, 3.0, 7.0]);

    println!("{}", std::mem::align_of_val(&a))
}
