use std::{fmt::Display, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign}};

use crate::Bool3;

#[derive(Copy, Clone)]
pub struct Bool4 {
    pub value: std::simd::Mask<i32, 4>,
}

impl Default for Bool4{
    #[inline]
    fn default() -> Self {
        Bool4 { value: std::simd::mask32x4::default() }
    }
}

impl Bool4{
    #[inline]
    pub fn trunc_bool3(self) -> Bool3{
        Bool3::set(self.x(), self.y(), self.z())
    }

    #[inline(always)]
    pub fn set(x : bool, y : bool, z : bool, w : bool) -> Bool4{
        Bool4 { value: std::simd::mask32x4::from_array([x,y,z,w]) }
    }

    #[inline]
    pub fn splat(val : bool) -> Bool4{
        Bool4 {
            value: std::simd::mask32x4::from_array([val; 4])
        }
    }

    #[inline]
    pub fn all(self) -> bool{
        self.value.all()
    }

    #[inline]
    pub fn any(self) -> bool{
        self.value.any()
    }

    #[inline]
    pub fn to_primitive(self) -> [bool;4]{
        self.value.to_array()
    }

    #[inline]
    pub fn from_primitive(array : [bool;4]) -> Bool4{
        Bool4 { value: std::simd::mask32x4::from_array(array) }
    }

    #[inline]
    pub fn x(self) -> bool{
        let bool_mask = self.value.to_array();

        bool_mask[0]
    }

    #[inline]
    pub fn y(self) -> bool{
        let bool_mask = self.value.to_array();

        bool_mask[1]
    }

    #[inline]
    pub fn z(self) -> bool{
        let bool_mask = self.value.to_array();

        bool_mask[2]
    }

    #[inline]
    pub fn w(self) -> bool{
        let bool_mask = self.value.to_array();

        bool_mask[3]
    }
}

impl Display for Bool4{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bool4 (x : {}, y : {}, z : {}, w : {})", self.x(), self.y(), self.z(), self.w())
    }
}

impl BitAnd<bool> for Bool4{
    type Output = Bool4;

    #[inline]
    fn bitand(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool4{ value: self.value & splat_rhs }
    }
}

impl BitAnd for Bool4{
    type Output = Bool4;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bool4{ value: self.value & rhs.value }
    }
}

impl BitAndAssign<bool> for Bool4{
    #[inline]
    fn bitand_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value &= splat_rhs;
    }
}

impl BitAndAssign for Bool4{
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl BitOr<bool> for Bool4{
    type Output = Bool4;

    #[inline]
    fn bitor(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool4{ value: self.value | splat_rhs }
    }
}

impl BitOr for Bool4{
    type Output = Bool4;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bool4{ value: self.value | rhs.value }
    }
}

impl BitOrAssign<bool> for Bool4{
    #[inline]
    fn bitor_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value |= splat_rhs;
    }
}

impl BitOrAssign for Bool4{
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl BitXor<bool> for Bool4{
    type Output = Bool4;

    #[inline]
    fn bitxor(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool4{ value: self.value ^ splat_rhs }
    }
}

impl BitXor for Bool4 {
    type Output = Bool4;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bool4{ value: self.value ^ rhs.value }
    }
}

impl BitXorAssign<bool> for Bool4{
    #[inline]
    fn bitxor_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value ^= splat_rhs;
    }
}

impl BitXorAssign for Bool4 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}