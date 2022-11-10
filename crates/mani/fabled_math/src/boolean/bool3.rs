use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use crate::{Bool2, Bool4};

use crate::math_trait::Swizzles3;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Bool3 {
    pub value: std::simd::Mask<i32, 4>,
}

impl Bool3 {
    #[inline]
    pub fn trunc_bool2(self) -> Bool2 {
        Bool2::set(self.x(), self.y())
    }

    #[inline(always)]
    pub fn set(x: bool, y: bool, z: bool) -> Bool3 {
        Bool3 {
            value: std::simd::mask32x4::from_array([x, y, z, false]),
        }
    }

    #[inline(always)]
    pub fn broadcast(val: bool) -> Bool3 {
        Bool3 {
            value: std::simd::mask32x4::from_array([val, val, val, false]),
        }
    }

    #[inline(always)]
    pub fn x(self) -> bool {
        let bool_mask = self.value.to_array();

        bool_mask[0]
    }

    #[inline(always)]
    pub fn y(self) -> bool {
        let bool_mask = self.value.to_array();

        bool_mask[1]
    }

    #[inline(always)]
    pub fn z(self) -> bool {
        let bool_mask = self.value.to_array();

        bool_mask[2]
    }

    #[inline]
    pub fn all(self) -> bool {
        let mut as_bool4 = Bool4 { value: self.value };

        unsafe {
            as_bool4.value.set_unchecked(2, true);
        }

        as_bool4.all()
    }

    #[inline]
    pub fn any(self) -> bool {
        self.value.any()
    }

    #[inline]
    pub fn to_primitive(self) -> [bool; 3] {
        let bool_mask = self.value.to_array();

        let z = bool_mask[2];

        [bool_mask[0], bool_mask[1], z]
    }

    #[inline]
    pub fn from_primitive(array: [bool; 3]) -> Bool3 {
        Bool3::set(array[0], array[1], array[2])
    }
}

impl Display for Bool3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bool3 (x : {}, y : {}, z : {})",
            self.x(),
            self.y(),
            self.z()
        )
    }
}

impl BitAnd<bool> for Bool3 {
    type Output = Bool3;

    #[inline]
    fn bitand(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool3 {
            value: self.value & splat_rhs,
        }
    }
}

impl BitAnd for Bool3 {
    type Output = Bool3;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bool3 {
            value: self.value & rhs.value,
        }
    }
}

impl BitAndAssign<bool> for Bool3 {
    #[inline]
    fn bitand_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value &= splat_rhs;
    }
}

impl BitAndAssign for Bool3 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl BitOr<bool> for Bool3 {
    type Output = Bool3;

    #[inline]
    fn bitor(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool3 {
            value: self.value | splat_rhs,
        }
    }
}

impl BitOr for Bool3 {
    type Output = Bool3;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bool3 {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign<bool> for Bool3 {
    #[inline]
    fn bitor_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value |= splat_rhs;
    }
}

impl BitOrAssign for Bool3 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl BitXor<bool> for Bool3 {
    type Output = Bool3;

    #[inline]
    fn bitxor(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool3 {
            value: self.value ^ splat_rhs,
        }
    }
}

impl BitXor for Bool3 {
    type Output = Bool3;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bool3 {
            value: self.value ^ rhs.value,
        }
    }
}

impl BitXorAssign<bool> for Bool3 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);
        self.value ^= splat_rhs;
    }
}

impl BitXorAssign for Bool3 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}


impl Swizzles3 for Bool3 {
    type Swizzle2 = Bool2;

    type Swizzle4 = Bool4;

    #[inline]
    fn xx(self) -> Self::Swizzle2 {
        Bool2::broadcast(self.x())
    }

    #[inline]
    fn xy(self) -> Self::Swizzle2 {
        Bool2::set(self.x(), self.y())
    }

    #[inline]
    fn xz(self) -> Self::Swizzle2 {
        Bool2::set(self.x(), self.z())
    }

    #[inline]
    fn yx(self) -> Self::Swizzle2 {
        Bool2::set(self.y(), self.x())
    }

    #[inline]
    fn yy(self) -> Self::Swizzle2 {
        Bool2::broadcast(self.y())
    }

    #[inline]
    fn yz(self) -> Self::Swizzle2 {
        Bool2::set(self.y(), self.z())
    }

    #[inline]
    fn zx(self) -> Self::Swizzle2 {
        Bool2::set(self.z(), self.x())
    }

    #[inline]
    fn zy(self) -> Self::Swizzle2 {
        Bool2::set(self.z(), self.y())
    }

    #[inline]
    fn zz(self) -> Self::Swizzle2 {
        Bool2::broadcast(self.z())
    }

    #[inline]
    fn xxx(self) -> Self {
        Bool3::broadcast(self.x())
    }

    #[inline]
    fn xxy(self) -> Self {
        let xxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxy) },
        }
    }

    #[inline]
    fn xxz(self) -> Self {
        let xxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxz) },
        }
    }

    #[inline]
    fn xyx(self) -> Self {
        let xyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyx) },
        }
    }

    #[inline]
    fn xyy(self) -> Self {
        let xyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyy) },
        }
    }

    #[inline]
    fn xzx(self) -> Self {
        let xzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzx) },
        }
    }

    #[inline]
    fn xzy(self) -> Self {
        let xzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzy) },
        }
    }

    #[inline]
    fn xzz(self) -> Self {
        let xzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzz) },
        }
    }

    #[inline]
    fn yxx(self) -> Self {
        let yxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxx) },
        }
    }

    #[inline]
    fn yxy(self) -> Self {
        let yxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxy) },
        }
    }

    #[inline]
    fn yxz(self) -> Self {
        let yxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxz) },
        }
    }

    #[inline]
    fn yyx(self) -> Self {
        let yyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyx) },
        }
    }

    #[inline]
    fn yyy(self) -> Self {
        Bool3::broadcast(self.y())
    }

    #[inline]
    fn yyz(self) -> Self {
        let yyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyz) },
        }
    }

    #[inline]
    fn yzx(self) -> Self {
        let yzx = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzx) },
        }
    }

    #[inline]
    fn yzy(self) -> Self {
        let yzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzy) },
        }
    }

    #[inline]
    fn yzz(self) -> Self {
        let yzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzz) },
        }
    }

    #[inline]
    fn zxx(self) -> Self {
        let zxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxx) },
        }
    }

    #[inline]
    fn zxy(self) -> Self {
        let zxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxy) },
        }
    }

    #[inline]
    fn zxz(self) -> Self {
        let zxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxz) },
        }
    }

    #[inline]
    fn zyx(self) -> Self {
        let zyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyx) },
        }
    }

    #[inline]
    fn zyy(self) -> Self {
        let zyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyy) },
        }
    }

    #[inline]
    fn zyz(self) -> Self {
        let zyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyz) },
        }
    }

    #[inline]
    fn zzx(self) -> Self {
        let zzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzx) },
        }
    }

    #[inline]
    fn zzy(self) -> Self {
        let zzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzy) },
        }
    }

    #[inline]
    fn zzz(self) -> Self {
        Bool3::broadcast(self.z())
    }

    #[inline]
    fn xxxx(self) -> Self::Swizzle4 {
        Bool4::broadcast(self.z())
    }

    #[inline]
    fn xxxy(self) -> Self::Swizzle4 {
        let xxxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxxy) },
        }
    }

    #[inline]
    fn xxxz(self) -> Self::Swizzle4 {
        let xxxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxxz) },
        }
    }

    #[inline]
    fn xxyx(self) -> Self::Swizzle4 {
        let xxyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxyx) },
        }
    }

    #[inline]
    fn xxyy(self) -> Self::Swizzle4 {
        let xxyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxyy) },
        }
    }

    #[inline]
    fn xxyz(self) -> Self::Swizzle4 {
        let xxyz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxyz) },
        }
    }

    #[inline]
    fn xxzx(self) -> Self::Swizzle4 {
        let xxzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxzx) },
        }
    }

    #[inline]
    fn xxzy(self) -> Self::Swizzle4 {
        let xxzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxzy) },
        }
    }

    #[inline]
    fn xxzz(self) -> Self::Swizzle4 {
        let xxzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxzz) },
        }
    }

    #[inline]
    fn xyxx(self) -> Self::Swizzle4 {
        let xyxx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyxx) },
        }
    }

    #[inline]
    fn xyxy(self) -> Self::Swizzle4 {
        let xyxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyxy) },
        }
    }

    #[inline]
    fn xyxz(self) -> Self::Swizzle4 {
        let xyxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyxz) },
        }
    }

    #[inline]
    fn xyyx(self) -> Self::Swizzle4 {
        let xyyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyyx) },
        }
    }

    #[inline]
    fn xyyy(self) -> Self::Swizzle4 {
        let xyyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyyy) },
        }
    }

    #[inline]
    fn xyyz(self) -> Self::Swizzle4 {
        let xyyz = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyyz) },
        }
    }

    #[inline]
    fn xyzx(self) -> Self::Swizzle4 {
        let xyzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyzx) },
        }
    }

    #[inline]
    fn xyzy(self) -> Self::Swizzle4 {
        let xyzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyzy) },
        }
    }

    #[inline]
    fn xyzz(self) -> Self::Swizzle4 {
        let xyzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyzz) },
        }
    }

    #[inline]
    fn xzxx(self) -> Self::Swizzle4 {
        let xzxx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzxx) },
        }
    }

    #[inline]
    fn xzxy(self) -> Self::Swizzle4 {
        let xzxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzxy) },
        }
    }

    #[inline]
    fn xzxz(self) -> Self::Swizzle4 {
        let xzxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzxz) },
        }
    }

    #[inline]
    fn xzyx(self) -> Self::Swizzle4 {
        let xzyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzyx) },
        }
    }

    #[inline]
    fn xzyy(self) -> Self::Swizzle4 {
        let xzyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzyy) },
        }
    }

    #[inline]
    fn xzyz(self) -> Self::Swizzle4 {
        let xzyz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzyz) },
        }
    }

    #[inline]
    fn xzzx(self) -> Self::Swizzle4 {
        let xzzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzzx) },
        }
    }

    #[inline]
    fn xzzy(self) -> Self::Swizzle4 {
        let xzzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzzy) },
        }
    }

    #[inline]
    fn xzzz(self) -> Self::Swizzle4 {
        let xzzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzzz) },
        }
    }

    #[inline]
    fn yxxx(self) -> Self::Swizzle4 {
        let yxxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxxx) },
        }
    }

    #[inline]
    fn yxxy(self) -> Self::Swizzle4 {
        let yxxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxxy) },
        }
    }

    #[inline]
    fn yxxz(self) -> Self::Swizzle4 {
        let yxxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxxz) },
        }
    }

    #[inline]
    fn yxyx(self) -> Self::Swizzle4 {
        let yxyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxyx) },
        }
    }

    #[inline]
    fn yxyy(self) -> Self::Swizzle4 {
        let yxyy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxyy) },
        }
    }

    #[inline]
    fn yxyz(self) -> Self::Swizzle4 {
        let yxyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxyz) },
        }
    }

    #[inline]
    fn yxzx(self) -> Self::Swizzle4 {
        let yxzx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxzx) },
        }
    }

    #[inline]
    fn yxzy(self) -> Self::Swizzle4 {
        let yxzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxzy) },
        }
    }

    #[inline]
    fn yxzz(self) -> Self::Swizzle4 {
        let yxzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxzz) },
        }
    }

    #[inline]
    fn yyxx(self) -> Self::Swizzle4 {
        let yyxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyxx) },
        }
    }

    #[inline]
    fn yyxy(self) -> Self::Swizzle4 {
        let yyxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyxy) },
        }
    }

    #[inline]
    fn yyxz(self) -> Self::Swizzle4 {
        let yyxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyxz) },
        }
    }

    #[inline]
    fn yyyx(self) -> Self::Swizzle4 {
        let yyyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyyx) },
        }
    }

    #[inline]
    fn yyyy(self) -> Self::Swizzle4 {
        Bool4::broadcast(self.y())
    }

    #[inline]
    fn yyyz(self) -> Self::Swizzle4 {
        let yyyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyyz) },
        }
    }

    #[inline]
    fn yyzx(self) -> Self::Swizzle4 {
        let yyzx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyzx) },
        }
    }

    #[inline]
    fn yyzy(self) -> Self::Swizzle4 {
        let yyzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyzy) },
        }
    }

    #[inline]
    fn yyzz(self) -> Self::Swizzle4 {
        let yyzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyzz) },
        }
    }

    #[inline]
    fn yzxx(self) -> Self::Swizzle4 {
        let yzxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzxx) },
        }
    }

    #[inline]
    fn yzxy(self) -> Self::Swizzle4 {
        let yzxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzxy) },
        }
    }

    #[inline]
    fn yzxz(self) -> Self::Swizzle4 {
        let yzxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzxz) },
        }
    }

    #[inline]
    fn yzyx(self) -> Self::Swizzle4 {
        let yzyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzyx) },
        }
    }

    #[inline]
    fn yzyy(self) -> Self::Swizzle4 {
        let yzyy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzyy) },
        }
    }

    #[inline]
    fn yzyz(self) -> Self::Swizzle4 {
        let yzyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzyz) },
        }
    }

    #[inline]
    fn yzzx(self) -> Self::Swizzle4 {
        let yzzx = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzzx) },
        }
    }

    #[inline]
    fn yzzy(self) -> Self::Swizzle4 {
        let yzzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzzy) },
        }
    }

    #[inline]
    fn yzzz(self) -> Self::Swizzle4 {
        let yzzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzzz) },
        }
    }

    #[inline]
    fn zxxx(self) -> Self::Swizzle4 {
        let zxxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxxx) },
        }
    }

    #[inline]
    fn zxxy(self) -> Self::Swizzle4 {
        let zxxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxxy) },
        }
    }

    #[inline]
    fn zxxz(self) -> Self::Swizzle4 {
        let zxxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxxz) },
        }
    }

    #[inline]
    fn zxyx(self) -> Self::Swizzle4 {
        let zxyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxyx) },
        }
    }

    #[inline]
    fn zxyy(self) -> Self::Swizzle4 {
        let zxyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxyy) },
        }
    }

    #[inline]
    fn zxyz(self) -> Self::Swizzle4 {
        let zxyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxyz) },
        }
    }

    #[inline]
    fn zxzx(self) -> Self::Swizzle4 {
        let zxzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxzx) },
        }
    }

    #[inline]
    fn zxzy(self) -> Self::Swizzle4 {
        let zxzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxzy) },
        }
    }

    #[inline]
    fn zxzz(self) -> Self::Swizzle4 {
        let zxzz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxzz) },
        }
    }

    #[inline]
    fn zyxx(self) -> Self::Swizzle4 {
        let zyxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyxx) },
        }
    }

    #[inline]
    fn zyxy(self) -> Self::Swizzle4 {
        let zyxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyxy) },
        }
    }

    #[inline]
    fn zyxz(self) -> Self::Swizzle4 {
        let zyxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyxz) },
        }
    }

    #[inline]
    fn zyyx(self) -> Self::Swizzle4 {
        let zyyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyyx) },
        }
    }

    #[inline]
    fn zyyy(self) -> Self::Swizzle4 {
        let zyyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyyy) },
        }
    }

    #[inline]
    fn zyyz(self) -> Self::Swizzle4 {
        let zyyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyyz) },
        }
    }

    #[inline]
    fn zyzx(self) -> Self::Swizzle4 {
        let zyzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyzx) },
        }
    }

    #[inline]
    fn zyzy(self) -> Self::Swizzle4 {
        let zyzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyzy) },
        }
    }

    #[inline]
    fn zyzz(self) -> Self::Swizzle4 {
        let zyzz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyzz) },
        }
    }

    #[inline]
    fn zzxx(self) -> Self::Swizzle4 {
        let zzxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzxx) },
        }
    }

    #[inline]
    fn zzxy(self) -> Self::Swizzle4 {
        let zzxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzxy) },
        }
    }

    #[inline]
    fn zzxz(self) -> Self::Swizzle4 {
        let zzxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzxz) },
        }
    }

    #[inline]
    fn zzyx(self) -> Self::Swizzle4 {
        let zzyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzyx) },
        }
    }

    #[inline]
    fn zzyy(self) -> Self::Swizzle4 {
        let zzyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzyy) },
        }
    }

    #[inline]
    fn zzyz(self) -> Self::Swizzle4 {
        let zzyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzyz) },
        }
    }

    #[inline]
    fn zzzx(self) -> Self::Swizzle4 {
        let zzzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzzx) },
        }
    }

    #[inline]
    fn zzzy(self) -> Self::Swizzle4 {
        let zzzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzzy) },
        }
    }

    #[inline]
    fn zzzz(self) -> Self::Swizzle4 {
        Bool4::broadcast(self.z())
    }
}
