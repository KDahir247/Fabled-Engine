use crate::boolean::{Bool3, Bool4};

use crate::math_trait::Swizzles2;

use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Bool2 {
    pub value: [bool; 2],
}


impl Bool2 {
    #[inline(always)]
    pub const fn set(x: bool, y: bool) -> Bool2 {
        Bool2 { value: [x, y] }
    }

    #[inline(always)]
    pub const fn broadcast(val: bool) -> Bool2 {
        Bool2 { value: [val; 2] }
    }

    #[inline(always)]
    pub const fn x(self) -> bool {
        self.value[0]
    }

    #[inline(always)]
    pub const fn y(self) -> bool {
        self.value[1]
    }

    #[inline]
    pub const fn all(self) -> bool {
        self.y() & self.x()
    }

    #[inline]
    pub const fn any(self) -> bool {
        self.y() | self.y()
    }

    #[inline]
    pub const fn to_primitive(self) -> [bool; 2] {
        self.value
    }

    #[inline]
    pub const fn from_primitive(array: [bool; 2]) -> Bool2 {
        Bool2 { value: array }
    }

    #[inline]
    pub fn to_int(self) -> [i32; 2] {
        [i32::from(self.x()), i32::from(self.y())]
    }

    #[inline]
    pub fn to_simd_int(self) -> std::simd::i32x4 {
        let int_repr = self.to_int();
        std::simd::i32x4::from_array([int_repr[0], int_repr[1], 0, 0])
    }

    #[inline]
    pub fn to_simd_mask(self) -> std::simd::mask32x4 {
        std::simd::mask32x4::from_array([self.x(), self.y(), false, false])
    }
}

impl Display for Bool2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bool2 (x : {}, y : {})", self.x(), self.y())
    }
}

impl BitAnd<bool> for Bool2 {
    type Output = Bool2;

    #[inline]
    fn bitand(self, rhs: bool) -> Self::Output {
        Bool2 {
            value: [self.x() & rhs, self.y() & rhs],
        }
    }
}

impl BitAnd<Bool2> for Bool2 {
    type Output = Bool2;

    #[inline]
    fn bitand(self, rhs: Bool2) -> Self::Output {
        Bool2 {
            value: [self.x() & rhs.x(), self.y() & rhs.y()],
        }
    }
}

impl BitAndAssign<bool> for Bool2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: bool) {
        self.value[0] &= rhs;
        self.value[1] &= rhs;
    }
}

impl BitAndAssign<Bool2> for Bool2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Bool2) {
        self.value[0] &= rhs.x();
        self.value[1] &= rhs.y();
    }
}

impl BitOr<bool> for Bool2 {
    type Output = Bool2;

    #[inline]
    fn bitor(self, rhs: bool) -> Self::Output {
        Bool2 {
            value: [self.x() | rhs, self.y() | rhs],
        }
    }
}

impl BitOr<Bool2> for Bool2 {
    type Output = Bool2;

    #[inline]
    fn bitor(self, rhs: Bool2) -> Self::Output {
        Bool2 {
            value: [self.x() | rhs.x(), self.y() | rhs.y()],
        }
    }
}

impl BitOrAssign<bool> for Bool2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: bool) {
        self.value[0] |= rhs;
        self.value[1] |= rhs;
    }
}

impl BitOrAssign<Bool2> for Bool2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Bool2) {
        self.value[0] |= rhs.x();
        self.value[1] |= rhs.y();
    }
}

impl BitXor<bool> for Bool2 {
    type Output = Bool2;

    #[inline]
    fn bitxor(self, rhs: bool) -> Self::Output {
        Bool2 {
            value: [self.x() ^ rhs, self.y() ^ rhs],
        }
    }
}

impl BitXor<Bool2> for Bool2 {
    type Output = Bool2;

    #[inline]
    fn bitxor(self, rhs: Bool2) -> Self::Output {
        Bool2 {
            value: [self.x() ^ rhs.x(), self.y() ^ rhs.y()],
        }
    }
}

impl BitXorAssign<bool> for Bool2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: bool) {
        self.value[0] ^= rhs;
        self.value[1] ^= rhs;
    }
}

impl BitXorAssign<Bool2> for Bool2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Bool2) {
        self.value[0] ^= rhs.x();
        self.value[1] ^= rhs.y();
    }
}

impl Swizzles2 for Bool2 {
    type Swizzle3 = Bool3;
    type Swizzle4 = Bool4;

    #[inline]
    fn xx(self) -> Self {
        Bool2::broadcast(self.x())
    }

    #[inline]
    fn yx(self) -> Self {
        Bool2::set(self.y(), self.x())
    }

    #[inline]
    fn yy(self) -> Self {
        Bool2::broadcast(self.y())
    }

    #[inline]
    fn xxx(self) -> Self::Swizzle3 {
        Bool3::broadcast(self.x())
    }

    #[inline]
    fn xxy(self) -> Self::Swizzle3 {
        Bool3::set(self.x(), self.x(), self.y())
    }

    #[inline]
    fn xyx(self) -> Self::Swizzle3 {
        Bool3::set(self.x(), self.y(), self.x())
    }

    #[inline]
    fn xyy(self) -> Self::Swizzle3 {
        Bool3::set(self.x(), self.y(), self.y())
    }

    #[inline]
    fn yxx(self) -> Self::Swizzle3 {
        Bool3::set(self.y(), self.x(), self.x())
    }

    #[inline]
    fn yxy(self) -> Self::Swizzle3 {
        Bool3::set(self.y(), self.x(), self.y())
    }

    #[inline]
    fn yyx(self) -> Self::Swizzle3 {
        Bool3::set(self.y(), self.y(), self.x())
    }

    #[inline]
    fn yyy(self) -> Self::Swizzle3 {
        Bool3::broadcast(self.y())
    }

    #[inline]
    fn xxxx(self) -> Self::Swizzle4 {
        Bool4::broadcast(self.x())
    }

    #[inline]
    fn xxxy(self) -> Self::Swizzle4 {
        Bool4::set(self.x(), self.x(), self.x(), self.y())
    }

    #[inline]
    fn xxyx(self) -> Self::Swizzle4 {
        Bool4::set(self.x(), self.x(), self.y(), self.x())
    }

    #[inline]
    fn xxyy(self) -> Self::Swizzle4 {
        Bool4::set(self.x(), self.x(), self.y(), self.y())
    }

    #[inline]
    fn xyxx(self) -> Self::Swizzle4 {
        Bool4::set(self.x(), self.y(), self.x(), self.x())
    }

    #[inline]
    fn xyxy(self) -> Self::Swizzle4 {
        Bool4::set(self.x(), self.y(), self.x(), self.y())
    }

    #[inline]
    fn xyyx(self) -> Self::Swizzle4 {
        Bool4::set(self.x(), self.y(), self.y(), self.x())
    }

    #[inline]
    fn xyyy(self) -> Self::Swizzle4 {
        Bool4::set(self.x(), self.y(), self.y(), self.y())
    }

    #[inline]
    fn yxxx(self) -> Self::Swizzle4 {
        Bool4::set(self.y(), self.x(), self.x(), self.x())
    }

    #[inline]
    fn yxxy(self) -> Self::Swizzle4 {
        Bool4::set(self.y(), self.x(), self.x(), self.y())
    }

    fn yxyx(self) -> Self::Swizzle4 {
        Bool4::set(self.y(), self.x(), self.y(), self.x())
    }

    #[inline]
    fn yxyy(self) -> Self::Swizzle4 {
        Bool4::set(self.y(), self.x(), self.y(), self.y())
    }

    #[inline]
    fn yyxx(self) -> Self::Swizzle4 {
        Bool4::set(self.y(), self.y(), self.x(), self.x())
    }

    #[inline]
    fn yyxy(self) -> Self::Swizzle4 {
        Bool4::set(self.y(), self.y(), self.x(), self.y())
    }

    #[inline]
    fn yyyx(self) -> Self::Swizzle4 {
        Bool4::set(self.y(), self.y(), self.y(), self.x())
    }

    #[inline]
    fn yyyy(self) -> Self::Swizzle4 {
        Bool4::broadcast(self.y())
    }
}
