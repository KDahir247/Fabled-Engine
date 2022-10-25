use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use crate::{Bool2, Bool4};

use crate::math_trait::Swizzles3;

#[derive(Copy, Clone)]
pub struct Bool3 {
    pub value: std::simd::Mask<i32, 4>,
}


impl Default for Bool3{
    #[inline]
    fn default() -> Self {
        Bool3 { value: std::simd::mask32x4::default() }
    }
}

impl Bool3{
    #[inline]
    pub fn trunc_bool2(self) -> Bool2{
        Bool2::set(self.x(), self.y())
    }

    #[inline(always)]
    pub fn set(x : bool, y : bool, z : bool) -> Bool3{
        Bool3 { value: std::simd::mask32x4::from_array([x,y,z, false]) }
    }

    #[inline]
    pub fn splat(val : bool) -> Bool3{
        Bool3 {
            value: std::simd::mask32x4::from_array([val, val,val,false])
        }
    }

    // we need to do seperate check for xyz for any and all since we are using a simd 4 lanes
    #[inline]
    pub fn all(self) -> bool{
        self.x() & self.y() & self.z()
    }

    #[inline]
    pub fn any(self) -> bool{
        self.x() | self.y() || self.z()
    }

    #[inline]
    pub fn to_primitive(self) -> [bool;3]{
        [self.x(), self.y(), self.y()]
    }

    #[inline]
    pub fn from_primitive(array : [bool;3]) -> Bool3{
        Bool3::set(array[0], array[1], array[2])
    }

    #[inline]
    pub fn x(self)-> bool{
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
}

impl Display for Bool3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bool3 (x : {}, y : {}, z : {})", self.x(), self.y(), self.z())
    }
}

impl BitAnd<bool> for Bool3{
    type Output = Bool3;

    #[inline]
    fn bitand(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool3{ value: self.value & splat_rhs }
    }
}

impl BitAnd for Bool3{
    type Output = Bool3;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bool3{ value: self.value & rhs.value }
    }
}

impl BitAndAssign<bool> for Bool3{
    #[inline]
    fn bitand_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value &= splat_rhs;
    }
}

impl BitAndAssign for Bool3{
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl BitOr<bool> for Bool3{
    type Output = Bool3;

    #[inline]
    fn bitor(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool3{ value: self.value | splat_rhs }
    }
}

impl BitOr for Bool3{
    type Output = Bool3;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bool3{ value: self.value | rhs.value }
    }
}

impl BitOrAssign<bool> for Bool3{
    #[inline]
    fn bitor_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value |= splat_rhs;
    }
}

impl BitOrAssign for Bool3{
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl BitXor<bool> for Bool3{
    type Output = Bool3;

    #[inline]
    fn bitxor(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool3{ value: self.value ^ splat_rhs }
    }
}

impl BitXor for Bool3{
    type Output = Bool3;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bool3{ value: self.value ^ rhs.value }
    }
}

impl BitXorAssign<bool> for Bool3{
    #[inline]
    fn bitxor_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);
        self.value ^= splat_rhs;
    }
}

impl BitXorAssign for Bool3{
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}