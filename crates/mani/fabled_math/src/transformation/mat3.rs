use std::fmt::{Display, Formatter};

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

impl Display for Matrix3x3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array_representation: [f32; 16] = *self.value.as_array();

        write!(
            f,
            "Matrix3x3\n[\n\t{}, {}, {}\n\t{}, {}, {},\n\t{}, {}, {}\n]",
            array_representation[0],
            array_representation[1],
            array_representation[2],
            array_representation[4],
            array_representation[5],
            array_representation[6],
            array_representation[8],
            array_representation[9],
            array_representation[10]
        )
    }
}

pub struct SOAMatrix3x3<const N: usize> {
    pub value: [Matrix3x3; N],
}
