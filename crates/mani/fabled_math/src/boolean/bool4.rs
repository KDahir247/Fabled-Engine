use crate::{Bool2, Bool3};

use crate::math_trait::Swizzles4;

use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign},
};


#[derive(Copy, Clone, Default)]
pub struct Bool4 {
    pub value: std::simd::Mask<i32, 4>,
}

impl Bool4 {
    #[inline]
    pub fn trunc_bool3(self) -> Bool3 {
        Bool3::set(self.x(), self.y(), self.z())
    }

    #[inline(always)]
    pub fn set(x: bool, y: bool, z: bool, w: bool) -> Bool4 {
        Bool4 {
            value: std::simd::mask32x4::from_array([x, y, z, w]),
        }
    }

    #[inline]
    pub fn broadcast(val: bool) -> Bool4 {
        Bool4 {
            value: std::simd::mask32x4::from_array([val; 4]),
        }
    }

    #[inline]
    pub fn all(self) -> bool {
        self.value.all()
    }

    #[inline]
    pub fn any(self) -> bool {
        self.value.any()
    }

    #[inline]
    pub fn to_primitive(self) -> [bool; 4] {
        self.value.to_array()
    }

    #[inline]
    pub fn from_primitive(array: [bool; 4]) -> Bool4 {
        Bool4 {
            value: std::simd::mask32x4::from_array(array),
        }
    }

    #[inline]
    pub fn x(self) -> bool {
        let bool_mask = self.value.to_array();

        bool_mask[0]
    }

    #[inline]
    pub fn y(self) -> bool {
        let bool_mask = self.value.to_array();

        bool_mask[1]
    }

    #[inline]
    pub fn z(self) -> bool {
        let bool_mask = self.value.to_array();

        bool_mask[2]
    }

    #[inline]
    pub fn w(self) -> bool {
        let bool_mask = self.value.to_array();

        bool_mask[3]
    }
}

impl Display for Bool4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bool4 (x : {}, y : {}, z : {}, w : {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}

impl BitAnd<bool> for Bool4 {
    type Output = Bool4;

    #[inline]
    fn bitand(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool4 {
            value: self.value & splat_rhs,
        }
    }
}

impl BitAnd for Bool4 {
    type Output = Bool4;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bool4 {
            value: self.value & rhs.value,
        }
    }
}

impl BitAndAssign<bool> for Bool4 {
    #[inline]
    fn bitand_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value &= splat_rhs;
    }
}

impl BitAndAssign for Bool4 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl BitOr<bool> for Bool4 {
    type Output = Bool4;

    #[inline]
    fn bitor(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool4 {
            value: self.value | splat_rhs,
        }
    }
}

impl BitOr for Bool4 {
    type Output = Bool4;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bool4 {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign<bool> for Bool4 {
    #[inline]
    fn bitor_assign(&mut self, rhs: bool) {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        self.value |= splat_rhs;
    }
}

impl BitOrAssign for Bool4 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl BitXor<bool> for Bool4 {
    type Output = Bool4;

    #[inline]
    fn bitxor(self, rhs: bool) -> Self::Output {
        let splat_rhs = std::simd::mask32x4::splat(rhs);

        Bool4 {
            value: self.value ^ splat_rhs,
        }
    }
}

impl BitXor for Bool4 {
    type Output = Bool4;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bool4 {
            value: self.value ^ rhs.value,
        }
    }
}

impl BitXorAssign<bool> for Bool4 {
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

impl Swizzles4 for Bool4 {
    type Swizzle2 = Bool2;

    type Swizzle3 = Bool3;

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
        Bool2::set(self.x(), self.y())
    }

    #[inline]
    fn xw(self) -> Self::Swizzle2 {
        Bool2::set(self.x(), self.w())
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
    fn yw(self) -> Self::Swizzle2 {
        Bool2::set(self.y(), self.w())
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
    fn zw(self) -> Self::Swizzle2 {
        Bool2::set(self.z(), self.w())
    }

    #[inline]
    fn wx(self) -> Self::Swizzle2 {
        Bool2::set(self.w(), self.x())
    }

    #[inline]
    fn wy(self) -> Self::Swizzle2 {
        Bool2::set(self.w(), self.y())
    }

    #[inline]
    fn wz(self) -> Self::Swizzle2 {
        Bool2::set(self.w(), self.z())
    }

    #[inline]
    fn ww(self) -> Self::Swizzle2 {
        Bool2::broadcast(self.w())
    }

    #[inline]
    fn xxx(self) -> Self::Swizzle3 {
        Bool3::broadcast(self.x())
    }

    #[inline]
    fn xxy(self) -> Self::Swizzle3 {
        let xxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxy) },
        }
    }

    #[inline]
    fn xxz(self) -> Self::Swizzle3 {
        let xxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxz) },
        }
    }

    #[inline]
    fn xxw(self) -> Self::Swizzle3 {
        let xxw = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxw) },
        }
    }

    #[inline]
    fn xyx(self) -> Self::Swizzle3 {
        let xyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyx) },
        }
    }

    #[inline]
    fn xyy(self) -> Self::Swizzle3 {
        let xyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyy) },
        }
    }

    #[inline]
    fn xyz(self) -> Self::Swizzle3 {
        let xyz = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyz) },
        }
    }

    #[inline]
    fn xyw(self) -> Self::Swizzle3 {
        let xyw = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyw) },
        }
    }

    #[inline]
    fn xzx(self) -> Self::Swizzle3 {
        let xzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzx) },
        }
    }

    #[inline]
    fn xzy(self) -> Self::Swizzle3 {
        let xzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzy) },
        }
    }

    #[inline]
    fn xzz(self) -> Self::Swizzle3 {
        let xzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzz) },
        }
    }

    #[inline]
    fn xzw(self) -> Self::Swizzle3 {
        let xzw = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzw) },
        }
    }

    #[inline]
    fn xwx(self) -> Self::Swizzle3 {
        let xwx = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwx) },
        }
    }

    #[inline]
    fn xwy(self) -> Self::Swizzle3 {
        let xwy = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwy) },
        }
    }

    #[inline]
    fn xwz(self) -> Self::Swizzle3 {
        let xwz = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwz) },
        }
    }

    #[inline]
    fn xww(self) -> Self::Swizzle3 {
        let xww = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xww) },
        }
    }

    #[inline]
    fn yxx(self) -> Self::Swizzle3 {
        let yxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxx) },
        }
    }

    #[inline]
    fn yxy(self) -> Self::Swizzle3 {
        let yxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxy) },
        }
    }

    #[inline]
    fn yxz(self) -> Self::Swizzle3 {
        let yxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxz) },
        }
    }

    #[inline]
    fn yxw(self) -> Self::Swizzle3 {
        let yxw = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxw) },
        }
    }

    #[inline]
    fn yyx(self) -> Self::Swizzle3 {
        let yyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyx) },
        }
    }

    #[inline]
    fn yyy(self) -> Self::Swizzle3 {
        Bool3::broadcast(self.y())
    }

    #[inline]
    fn yyz(self) -> Self::Swizzle3 {
        let yyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyz) },
        }
    }

    #[inline]
    fn yyw(self) -> Self::Swizzle3 {
        let yyw = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyw) },
        }
    }

    #[inline]
    fn yzx(self) -> Self::Swizzle3 {
        let yzx = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzx) },
        }
    }

    #[inline]
    fn yzy(self) -> Self::Swizzle3 {
        let yzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzy) },
        }
    }

    #[inline]
    fn yzz(self) -> Self::Swizzle3 {
        let yzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzz) },
        }
    }

    #[inline]
    fn yzw(self) -> Self::Swizzle3 {
        let yzw = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzw) },
        }
    }

    #[inline]
    fn ywx(self) -> Self::Swizzle3 {
        let ywx = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywx) },
        }
    }

    #[inline]
    fn ywy(self) -> Self::Swizzle3 {
        let ywy = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywy) },
        }
    }

    #[inline]
    fn ywz(self) -> Self::Swizzle3 {
        let ywz = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywz) },
        }
    }

    #[inline]
    fn yww(self) -> Self::Swizzle3 {
        let yww = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yww) },
        }
    }

    #[inline]
    fn zxx(self) -> Self::Swizzle3 {
        let zxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxx) },
        }
    }

    #[inline]
    fn zxy(self) -> Self::Swizzle3 {
        let zxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxy) },
        }
    }

    #[inline]
    fn zxz(self) -> Self::Swizzle3 {
        let zxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxz) },
        }
    }

    #[inline]
    fn zxw(self) -> Self::Swizzle3 {
        let zxw = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxw) },
        }
    }

    #[inline]
    fn zyx(self) -> Self::Swizzle3 {
        let zyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyx) },
        }
    }

    #[inline]
    fn zyy(self) -> Self::Swizzle3 {
        let zyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyy) },
        }
    }

    #[inline]
    fn zyz(self) -> Self::Swizzle3 {
        let zyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyz) },
        }
    }

    #[inline]
    fn zyw(self) -> Self::Swizzle3 {
        let zyw = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyw) },
        }
    }

    #[inline]
    fn zzx(self) -> Self::Swizzle3 {
        let zzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzx) },
        }
    }

    #[inline]
    fn zzy(self) -> Self::Swizzle3 {
        let zzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzy) },
        }
    }

    #[inline]
    fn zzz(self) -> Self::Swizzle3 {
        Bool3::broadcast(self.z())
    }

    #[inline]
    fn zzw(self) -> Self::Swizzle3 {
        let zzw = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzw) },
        }
    }

    #[inline]
    fn zwx(self) -> Self::Swizzle3 {
        let zwx = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwx) },
        }
    }

    #[inline]
    fn zwy(self) -> Self::Swizzle3 {
        let zwy = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwy) },
        }
    }

    #[inline]
    fn zwz(self) -> Self::Swizzle3 {
        let zwz = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwz) },
        }
    }

    #[inline]
    fn zww(self) -> Self::Swizzle3 {
        let zww = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zww) },
        }
    }

    #[inline]
    fn wxx(self) -> Self::Swizzle3 {
        let wxx = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxx) },
        }
    }

    #[inline]
    fn wxy(self) -> Self::Swizzle3 {
        let wxy = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxy) },
        }
    }

    #[inline]
    fn wxz(self) -> Self::Swizzle3 {
        let wxz = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxz) },
        }
    }

    #[inline]
    fn wxw(self) -> Self::Swizzle3 {
        let wxw = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxw) },
        }
    }

    #[inline]
    fn wyx(self) -> Self::Swizzle3 {
        let wyx = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyx) },
        }
    }

    #[inline]
    fn wyy(self) -> Self::Swizzle3 {
        let wyy = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyy) },
        }
    }

    #[inline]
    fn wyz(self) -> Self::Swizzle3 {
        let wyz = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyz) },
        }
    }

    #[inline]
    fn wyw(self) -> Self::Swizzle3 {
        let wyw = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 3, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyw) },
        }
    }

    #[inline]
    fn wzx(self) -> Self::Swizzle3 {
        let wzx = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzx) },
        }
    }

    #[inline]
    fn wzy(self) -> Self::Swizzle3 {
        let wzy = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzy) },
        }
    }

    #[inline]
    fn wzz(self) -> Self::Swizzle3 {
        let wzz = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzz) },
        }
    }

    #[inline]
    fn wzw(self) -> Self::Swizzle3 {
        let wzw = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 3, 3]);


        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzw) },
        }
    }

    #[inline]
    fn wwx(self) -> Self::Swizzle3 {
        let wwx = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 0, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwx) },
        }
    }

    #[inline]
    fn wwy(self) -> Self::Swizzle3 {
        let wwy = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 1, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwy) },
        }
    }

    #[inline]
    fn wwz(self) -> Self::Swizzle3 {
        let wwz = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 2, 3]);

        Bool3 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwz) },
        }
    }

    #[inline]
    fn www(self) -> Self::Swizzle3 {
        Bool3::broadcast(self.w())
    }


    #[inline]
    fn xxxx(self) -> Self {
        Bool4::broadcast(self.x())
    }

    #[inline]
    fn xxxy(self) -> Self {
        let xxxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxxy) },
        }
    }

    #[inline]
    fn xxxz(self) -> Self {
        let xxxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxxz) },
        }
    }

    #[inline]
    fn xxxw(self) -> Self {
        let xxxw = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxxw) },
        }
    }

    #[inline]
    fn xxyx(self) -> Self {
        let xxyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxyx) },
        }
    }

    #[inline]
    fn xxyy(self) -> Self {
        let xxyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxyy) },
        }
    }

    #[inline]
    fn xxyz(self) -> Self {
        let xxyz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxyz) },
        }
    }

    #[inline]
    fn xxyw(self) -> Self {
        let xxyw = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxyw) },
        }
    }

    #[inline]
    fn xxzx(self) -> Self {
        let xxzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxzx) },
        }
    }

    #[inline]
    fn xxzy(self) -> Self {
        let xxzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxzy) },
        }
    }

    #[inline]
    fn xxzz(self) -> Self {
        let xxzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 2,]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxzz) },
        }
    }

    #[inline]
    fn xxzw(self) -> Self {
        let xxzw = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxzw) },
        }
    }

    #[inline]
    fn xxwx(self) -> Self {
        let xxwx = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxwx) },
        }
    }

    #[inline]
    fn xxwy(self) -> Self {
        let xxwy = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxwy) },
        }
    }

    #[inline]
    fn xxwz(self) -> Self {
        let xxwz = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxwz) },
        }
    }

    #[inline]
    fn xxww(self) -> Self {
        let xxww = std::simd::simd_swizzle!(self.value.to_int(), [0, 0, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xxww) },
        }
    }

    #[inline]
    fn xyxx(self) -> Self {
        let xyxx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyxx) },
        }
    }

    #[inline]
    fn xyxy(self) -> Self {
        let xyxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyxy) },
        }
    }

    #[inline]
    fn xyxz(self) -> Self {
        let xyxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyxz) },
        }
    }

    #[inline]
    fn xyxw(self) -> Self {
        let xyxw = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyxw) },
        }
    }

    #[inline]
    fn xyyx(self) -> Self {
        let xyyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyyx) },
        }
    }

    #[inline]
    fn xyyy(self) -> Self {
        let xyyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyyy) },
        }
    }

    #[inline]
    fn xyyz(self) -> Self {
        let xyyz = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyyz) },
        }
    }

    #[inline]
    fn xyyw(self) -> Self {
        let xyyw = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyyw) },
        }
    }

    #[inline]
    fn xyzx(self) -> Self {
        let xyzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyzx) },
        }
    }

    #[inline]
    fn xyzy(self) -> Self {
        let xyzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyzy) },
        }
    }

    #[inline]
    fn xyzz(self) -> Self {
        let xyzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyzz) },
        }
    }

    #[inline]
    fn xywx(self) -> Self {
        let xywx = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xywx) },
        }
    }

    #[inline]
    fn xywy(self) -> Self {
        let xywy = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xywy) },
        }
    }

    #[inline]
    fn xywz(self) -> Self {
        let xywz = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xywz) },
        }
    }

    #[inline]
    fn xyww(self) -> Self {
        let xyww = std::simd::simd_swizzle!(self.value.to_int(), [0, 1, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xyww) },
        }
    }

    #[inline]
    fn xzxx(self) -> Self {
        let xzxx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzxx) },
        }
    }

    #[inline]
    fn xzxy(self) -> Self {
        let xzxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzxy) },
        }
    }

    #[inline]
    fn xzxz(self) -> Self {
        let xzxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzxz) },
        }
    }

    #[inline]
    fn xzxw(self) -> Self {
        let xzxw = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzxw) },
        }
    }

    #[inline]
    fn xzyx(self) -> Self {
        let xzyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzyx) },
        }
    }

    #[inline]
    fn xzyy(self) -> Self {
        let xzyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzyy) },
        }
    }

    #[inline]
    fn xzyz(self) -> Self {
        let xzyz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzyz) },
        }
    }

    #[inline]
    fn xzyw(self) -> Self {
        let xzyw = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzyw) },
        }
    }

    #[inline]
    fn xzzx(self) -> Self {
        let xzzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzzx) },
        }
    }

    #[inline]
    fn xzzy(self) -> Self {
        let xzzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzzy) },
        }
    }

    #[inline]
    fn xzzz(self) -> Self {
        let xzzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzzz) },
        }
    }

    #[inline]
    fn xzzw(self) -> Self {
        let xzzw = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzzw) },
        }
    }

    #[inline]
    fn xzwx(self) -> Self {
        let xzwx = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzwx) },
        }
    }

    #[inline]
    fn xzwy(self) -> Self {
        let xzwy = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzwy) },
        }
    }

    #[inline]
    fn xzwz(self) -> Self {
        let xzwz = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzwz) },
        }
    }

    #[inline]
    fn xzww(self) -> Self {
        let xzww = std::simd::simd_swizzle!(self.value.to_int(), [0, 2, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xzww) },
        }
    }

    #[inline]
    fn xwxx(self) -> Self {
        let xwxx = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwxx) },
        }
    }

    #[inline]
    fn xwxy(self) -> Self {
        let xwxy = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwxy) },
        }
    }

    #[inline]
    fn xwxz(self) -> Self {
        let xwxz = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwxz) },
        }
    }

    #[inline]
    fn xwxw(self) -> Self {
        let xwxw = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwxw) },
        }
    }

    #[inline]
    fn xwyx(self) -> Self {
        let xwyx = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwyx) },
        }
    }

    #[inline]
    fn xwyy(self) -> Self {
        let xwyy = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwyy) },
        }
    }

    #[inline]
    fn xwyz(self) -> Self {
        let xwyz = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwyz) },
        }
    }

    #[inline]
    fn xwyw(self) -> Self {
        let xwyw = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwyw) },
        }
    }

    #[inline]
    fn xwzx(self) -> Self {
        let xwzx = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwzx) },
        }
    }

    #[inline]
    fn xwzy(self) -> Self {
        let xwzy = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwzy) },
        }
    }

    #[inline]
    fn xwzz(self) -> Self {
        let xwzz = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwzz) },
        }
    }

    #[inline]
    fn xwzw(self) -> Self {
        let xwzw = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwzw) },
        }
    }

    #[inline]
    fn xwwx(self) -> Self {
        let xwwx = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwwx) },
        }
    }

    #[inline]
    fn xwwy(self) -> Self {
        let xwwy = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwwy) },
        }
    }

    #[inline]
    fn xwwz(self) -> Self {
        let xwwz = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwwz) },
        }
    }

    #[inline]
    fn xwww(self) -> Self {
        let xwww = std::simd::simd_swizzle!(self.value.to_int(), [0, 3, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(xwww) },
        }
    }

    #[inline]
    fn yxxx(self) -> Self {
        let yxxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxxx) },
        }
    }

    #[inline]
    fn yxxy(self) -> Self {
        let yxxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxxy) },
        }
    }

    #[inline]
    fn yxxz(self) -> Self {
        let yxxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxxz) },
        }
    }

    #[inline]
    fn yxxw(self) -> Self {
        let yxxw = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxxw) },
        }
    }

    #[inline]
    fn yxyx(self) -> Self {
        let yxyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxyx) },
        }
    }

    #[inline]
    fn yxyy(self) -> Self {
        let yxyy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxyy) },
        }
    }

    #[inline]
    fn yxyz(self) -> Self {
        let yxyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxyz) },
        }
    }

    #[inline]
    fn yxyw(self) -> Self {
        let yxyw = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxyw) },
        }
    }

    #[inline]
    fn yxzx(self) -> Self {
        let yxzx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxzx) },
        }
    }

    #[inline]
    fn yxzy(self) -> Self {
        let yxzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxzy) },
        }
    }

    #[inline]
    fn yxzz(self) -> Self {
        let yxzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxzz) },
        }
    }

    #[inline]
    fn yxzw(self) -> Self {
        let yxzw = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxzw) },
        }
    }

    #[inline]
    fn yxwx(self) -> Self {
        let yxwx = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxwx) },
        }
    }

    #[inline]
    fn yxwy(self) -> Self {
        let yxwy = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxwy) },
        }
    }

    #[inline]
    fn yxwz(self) -> Self {
        let yxwz = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxwz) },
        }
    }

    #[inline]
    fn yxww(self) -> Self {
        let yxww = std::simd::simd_swizzle!(self.value.to_int(), [1, 0, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yxww) },
        }
    }

    #[inline]
    fn yyxx(self) -> Self {
        let yyxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyxx) },
        }
    }

    #[inline]
    fn yyxy(self) -> Self {
        let yyxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyxy) },
        }
    }

    #[inline]
    fn yyxz(self) -> Self {
        let yyxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyxz) },
        }
    }

    #[inline]
    fn yyxw(self) -> Self {
        let yyxw = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyxw) },
        }
    }

    #[inline]
    fn yyyx(self) -> Self {
        let yyyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyyx) },
        }
    }

    #[inline]
    fn yyyy(self) -> Self {
        Bool4::broadcast(self.y())
    }

    #[inline]
    fn yyyz(self) -> Self {
        let yyyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyyz) },
        }
    }

    #[inline]
    fn yyyw(self) -> Self {
        let yyyw = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyyw) },
        }
    }

    #[inline]
    fn yyzx(self) -> Self {
        let yyzx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyzx) },
        }
    }

    #[inline]
    fn yyzy(self) -> Self {
        let yyzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyzy) },
        }
    }

    #[inline]
    fn yyzz(self) -> Self {
        let yyzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyzz) },
        }
    }

    #[inline]
    fn yyzw(self) -> Self {
        let yyzw = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyzw) },
        }
    }

    #[inline]
    fn yywx(self) -> Self {
        let yywx = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yywx) },
        }
    }

    #[inline]
    fn yywy(self) -> Self {
        let yywy = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yywy) },
        }
    }

    #[inline]
    fn yywz(self) -> Self {
        let yywz = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yywz) },
        }
    }

    #[inline]
    fn yyww(self) -> Self {
        let yyww = std::simd::simd_swizzle!(self.value.to_int(), [1, 1, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yyww) },
        }
    }

    #[inline]
    fn yzxx(self) -> Self {
        let yzxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzxx) },
        }
    }

    #[inline]
    fn yzxy(self) -> Self {
        let yzxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzxy) },
        }
    }

    #[inline]
    fn yzxz(self) -> Self {
        let yzxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzxz) },
        }
    }

    #[inline]
    fn yzxw(self) -> Self {
        let yzxw = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzxw) },
        }
    }

    #[inline]
    fn yzyx(self) -> Self {
        let yzyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzyx) },
        }
    }

    #[inline]
    fn yzyy(self) -> Self {
        let yzyy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzyy) },
        }
    }

    #[inline]
    fn yzyz(self) -> Self {
        let yzyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzyz) },
        }
    }

    #[inline]
    fn yzyw(self) -> Self {
        let yzyw = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzyw) },
        }
    }

    #[inline]
    fn yzzx(self) -> Self {
        let yzyw = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzyw) },
        }
    }

    #[inline]
    fn yzzy(self) -> Self {
        let yzzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzzy) },
        }
    }

    #[inline]
    fn yzzz(self) -> Self {
        let yzzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzzz) },
        }
    }

    #[inline]
    fn yzzw(self) -> Self {
        let yzzw = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzzw) },
        }
    }

    #[inline]
    fn yzwx(self) -> Self {
        let yzwx = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzwx) },
        }
    }

    #[inline]
    fn yzwy(self) -> Self {
        let yzwy = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzwy) },
        }
    }

    #[inline]
    fn yzwz(self) -> Self {
        let yzwz = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzwz) },
        }
    }

    #[inline]
    fn yzww(self) -> Self {
        let yzww = std::simd::simd_swizzle!(self.value.to_int(), [1, 2, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(yzww) },
        }
    }

    #[inline]
    fn ywxx(self) -> Self {
        let ywxx = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywxx) },
        }
    }

    #[inline]
    fn ywxy(self) -> Self {
        let ywxy = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywxy) },
        }
    }

    #[inline]
    fn ywxz(self) -> Self {
        let ywxz = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywxz) },
        }
    }

    #[inline]
    fn ywxw(self) -> Self {
        let ywxw = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywxw) },
        }
    }

    #[inline]
    fn ywyx(self) -> Self {
        let ywyx = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywyx) },
        }
    }

    #[inline]
    fn ywyy(self) -> Self {
        let ywyy = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywyy) },
        }
    }

    #[inline]
    fn ywyz(self) -> Self {
        let ywyz = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywyz) },
        }
    }

    #[inline]
    fn ywyw(self) -> Self {
        let ywyw = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywyw) },
        }
    }

    #[inline]
    fn ywzx(self) -> Self {
        let ywzx = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywzx) },
        }
    }

    #[inline]
    fn ywzy(self) -> Self {
        let ywzy = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywzy) },
        }
    }

    #[inline]
    fn ywzz(self) -> Self {
        let ywzz = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywzz) },
        }
    }

    #[inline]
    fn ywzw(self) -> Self {
        let ywzw = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywzw) },
        }
    }

    #[inline]
    fn ywwx(self) -> Self {
        let ywwx = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywwx) },
        }
    }

    #[inline]
    fn ywwy(self) -> Self {
        let ywwy = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywwy) },
        }
    }

    #[inline]
    fn ywwz(self) -> Self {
        let ywwz = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywwz) },
        }
    }

    #[inline]
    fn ywww(self) -> Self {
        let ywww = std::simd::simd_swizzle!(self.value.to_int(), [1, 3, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(ywww) },
        }
    }

    #[inline]
    fn zxxx(self) -> Self {
        let zxxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxxx) },
        }
    }

    #[inline]
    fn zxxy(self) -> Self {
        let zxxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxxy) },
        }
    }

    #[inline]
    fn zxxz(self) -> Self {
        let zxxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxxz) },
        }
    }

    #[inline]
    fn zxxw(self) -> Self {
        let zxxw = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxxw) },
        }
    }

    #[inline]
    fn zxyx(self) -> Self {
        let zxyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxyx) },
        }
    }

    #[inline]
    fn zxyy(self) -> Self {
        let zxyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxyy) },
        }
    }

    #[inline]
    fn zxyz(self) -> Self {
        let zxyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxyz) },
        }
    }

    #[inline]
    fn zxyw(self) -> Self {
        let zxyw = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxyw) },
        }
    }

    #[inline]
    fn zxzx(self) -> Self {
        let zxzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxzx) },
        }
    }

    #[inline]
    fn zxzy(self) -> Self {
        let zxzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxzy) },
        }
    }

    #[inline]
    fn zxzz(self) -> Self {
        let zxzz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxzz) },
        }
    }

    #[inline]
    fn zxzw(self) -> Self {
        let zxzw = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxzw) },
        }
    }

    #[inline]
    fn zxwx(self) -> Self {
        let zxwx = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxwx) },
        }
    }

    #[inline]
    fn zxwy(self) -> Self {
        let zxwy = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxwy) },
        }
    }

    #[inline]
    fn zxwz(self) -> Self {
        let zxwz = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxwz) },
        }
    }

    #[inline]
    fn zxww(self) -> Self {
        let zxww = std::simd::simd_swizzle!(self.value.to_int(), [2, 0, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zxww) },
        }
    }

    #[inline]
    fn zyxx(self) -> Self {
        let zyxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyxx) },
        }
    }

    #[inline]
    fn zyxy(self) -> Self {
        let zyxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyxy) },
        }
    }

    #[inline]
    fn zyxz(self) -> Self {
        let zyxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyxz) },
        }
    }

    #[inline]
    fn zyxw(self) -> Self {
        let zyxw = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyxw) },
        }
    }

    #[inline]
    fn zyyx(self) -> Self {
        let zyyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyyx) },
        }
    }

    #[inline]
    fn zyyy(self) -> Self {
        let zyyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyyy) },
        }
    }

    #[inline]
    fn zyyz(self) -> Self {
        let zyyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyyz) },
        }
    }

    #[inline]
    fn zyyw(self) -> Self {
        let zyyw = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyyw) },
        }
    }

    #[inline]
    fn zyzx(self) -> Self {
        let zyzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyzx) },
        }
    }

    #[inline]
    fn zyzy(self) -> Self {
        let zyzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyzy) },
        }
    }

    #[inline]
    fn zyzz(self) -> Self {
        let zyzz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyzz) },
        }
    }

    #[inline]
    fn zyzw(self) -> Self {
        let zyzw = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyzw) },
        }
    }

    #[inline]
    fn zywx(self) -> Self {
        let zywx = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zywx) },
        }
    }

    #[inline]
    fn zywy(self) -> Self {
        let zywy = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zywy) },
        }
    }

    #[inline]
    fn zywz(self) -> Self {
        let zywz = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zywz) },
        }
    }

    #[inline]
    fn zyww(self) -> Self {
        let zyww = std::simd::simd_swizzle!(self.value.to_int(), [2, 1, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zyww) },
        }
    }

    #[inline]
    fn zzxx(self) -> Self {
        let zzxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzxx) },
        }
    }

    #[inline]
    fn zzxy(self) -> Self {
        let zzxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzxy) },
        }
    }

    #[inline]
    fn zzxz(self) -> Self {
        let zzxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzxz) },
        }
    }

    #[inline]
    fn zzxw(self) -> Self {
        let zzxw = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzxw) },
        }
    }

    #[inline]
    fn zzyx(self) -> Self {
        let zzyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzyx) },
        }
    }

    #[inline]
    fn zzyy(self) -> Self {
        let zzyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzyy) },
        }
    }

    #[inline]
    fn zzyz(self) -> Self {
        let zzyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzyz) },
        }
    }

    #[inline]
    fn zzyw(self) -> Self {
        let zzyw = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzyw) },
        }
    }

    #[inline]
    fn zzzx(self) -> Self {
        let zzzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzzx) },
        }
    }

    #[inline]
    fn zzzy(self) -> Self {
        let zzzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzzy) },
        }
    }

    #[inline]
    fn zzzz(self) -> Self {
        Bool4::broadcast(self.y())
    }

    #[inline]
    fn zzzw(self) -> Self {
        let zzzw = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzzw) },
        }
    }

    #[inline]
    fn zzwx(self) -> Self {
        let zzwx = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzwx) },
        }
    }

    #[inline]
    fn zzwy(self) -> Self {
        let zzwy = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzwy) },
        }
    }

    #[inline]
    fn zzwz(self) -> Self {
        let zzwz = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzwz) },
        }
    }

    #[inline]
    fn zzww(self) -> Self {
        let zzww = std::simd::simd_swizzle!(self.value.to_int(), [2, 2, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zzww) },
        }
    }

    #[inline]
    fn zwxx(self) -> Self {
        let zwxx = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwxx) },
        }
    }

    #[inline]
    fn zwxy(self) -> Self {
        let zwxy = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwxy) },
        }
    }

    #[inline]
    fn zwxz(self) -> Self {
        let zwxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwxz) },
        }
    }

    #[inline]
    fn zwxw(self) -> Self {
        let zwxz = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwxz) },
        }
    }

    #[inline]
    fn zwyx(self) -> Self {
        let zwyx = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwyx) },
        }
    }

    #[inline]
    fn zwyy(self) -> Self {
        let zwyy = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwyy) },
        }
    }

    #[inline]
    fn zwyz(self) -> Self {
        let zwyz = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwyz) },
        }
    }

    #[inline]
    fn zwyw(self) -> Self {
        let zwyw = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwyw) },
        }
    }

    #[inline]
    fn zwzx(self) -> Self {
        let zwzx = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwzx) },
        }
    }

    #[inline]
    fn zwzy(self) -> Self {
        let zwzy = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwzy) },
        }
    }

    #[inline]
    fn zwzz(self) -> Self {
        let zwzz = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwzz) },
        }
    }

    #[inline]
    fn zwzw(self) -> Self {
        let zwzw = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwzw) },
        }
    }

    #[inline]
    fn zwwx(self) -> Self {
        let zwwx = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwwx) },
        }
    }

    #[inline]
    fn zwwy(self) -> Self {
        let zwwy = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwwy) },
        }
    }

    #[inline]
    fn zwwz(self) -> Self {
        let zwwz = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwwz) },
        }
    }

    #[inline]
    fn zwww(self) -> Self {
        let zwww = std::simd::simd_swizzle!(self.value.to_int(), [2, 3, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(zwww) },
        }
    }

    #[inline]
    fn wxxx(self) -> Self {
        let wxxx = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxxx) },
        }
    }

    #[inline]
    fn wxxy(self) -> Self {
        let wxxy = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxxy) },
        }
    }

    #[inline]
    fn wxxz(self) -> Self {
        let wxxz = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxxz) },
        }
    }

    #[inline]
    fn wxxw(self) -> Self {
        let wxxw = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxxw) },
        }
    }

    #[inline]
    fn wxyx(self) -> Self {
        let wxyx = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxyx) },
        }
    }

    #[inline]
    fn wxyy(self) -> Self {
        let wxyy = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxyy) },
        }
    }

    #[inline]
    fn wxyz(self) -> Self {
        let wxyz = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxyz) },
        }
    }

    #[inline]
    fn wxyw(self) -> Self {
        let wxyw = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxyw) },
        }
    }

    #[inline]
    fn wxzx(self) -> Self {
        let wxzx = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxzx) },
        }
    }

    #[inline]
    fn wxzy(self) -> Self {
        let wxzy = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxzy) },
        }
    }

    #[inline]
    fn wxzz(self) -> Self {
        let wxzz = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxzz) },
        }
    }

    #[inline]
    fn wxzw(self) -> Self {
        let wxzw = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxzw) },
        }
    }

    #[inline]
    fn wxwx(self) -> Self {
        let wxwx = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxwx) },
        }
    }

    #[inline]
    fn wxwy(self) -> Self {
        let wxwy = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxwy) },
        }
    }

    #[inline]
    fn wxwz(self) -> Self {
        let wxwz = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxwz) },
        }
    }

    #[inline]
    fn wxww(self) -> Self {
        let wxww = std::simd::simd_swizzle!(self.value.to_int(), [3, 0, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wxww) },
        }
    }

    #[inline]
    fn wyxx(self) -> Self {
        let wyxx = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyxx) },
        }
    }

    #[inline]
    fn wyxy(self) -> Self {
        let wyxy = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyxy) },
        }
    }

    #[inline]
    fn wyxz(self) -> Self {
        let wyxz = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyxz) },
        }
    }

    #[inline]
    fn wyxw(self) -> Self {
        let wyxw = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyxw) },
        }
    }

    #[inline]
    fn wyyx(self) -> Self {
        let wyyx = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyyx) },
        }
    }

    #[inline]
    fn wyyy(self) -> Self {
        let wyyy = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyyy) },
        }
    }

    #[inline]
    fn wyyz(self) -> Self {
        let wyyz = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyyz) },
        }
    }

    #[inline]
    fn wyyw(self) -> Self {
        let wyyw = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyyw) },
        }
    }

    #[inline]
    fn wyzx(self) -> Self {
        let wyzx = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyzx) },
        }
    }

    #[inline]
    fn wyzy(self) -> Self {
        let wyzy = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyzy) },
        }
    }

    #[inline]
    fn wyzz(self) -> Self {
        let wyzz = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyzz) },
        }
    }

    #[inline]
    fn wyzw(self) -> Self {
        let wyzw = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyzw) },
        }
    }

    #[inline]
    fn wywx(self) -> Self {
        let wywx = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wywx) },
        }
    }

    #[inline]
    fn wywy(self) -> Self {
        let wywy = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wywy) },
        }
    }

    #[inline]
    fn wywz(self) -> Self {
        let wywz = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wywz) },
        }
    }

    #[inline]
    fn wyww(self) -> Self {
        let wyww = std::simd::simd_swizzle!(self.value.to_int(), [3, 1, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wyww) },
        }
    }

    #[inline]
    fn wzxx(self) -> Self {
        let wzxx = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzxx) },
        }
    }

    #[inline]
    fn wzxy(self) -> Self {
        let wzxy = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzxy) },
        }
    }

    #[inline]
    fn wzxz(self) -> Self {
        let wzxz = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzxz) },
        }
    }

    #[inline]
    fn wzxw(self) -> Self {
        let wzxw = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzxw) },
        }
    }

    #[inline]
    fn wzyx(self) -> Self {
        let wzyx = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzyx) },
        }
    }

    #[inline]
    fn wzyy(self) -> Self {
        let wzyy = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzyy) },
        }
    }

    #[inline]
    fn wzyz(self) -> Self {
        let wzyz = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzyz) },
        }
    }

    #[inline]
    fn wzyw(self) -> Self {
        let wzyw = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzyw) },
        }
    }

    #[inline]
    fn wzzx(self) -> Self {
        let wzzx = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzzx) },
        }
    }

    #[inline]
    fn wzzy(self) -> Self {
        let wzzy = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzzy) },
        }
    }

    #[inline]
    fn wzzz(self) -> Self {
        let wzzz = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzzz) },
        }
    }

    #[inline]
    fn wzzw(self) -> Self {
        let wzzw = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzzw) },
        }
    }

    #[inline]
    fn wzwx(self) -> Self {
        let wzwx = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzwx) },
        }
    }

    #[inline]
    fn wzwy(self) -> Self {
        let wzwy = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzwy) },
        }
    }

    #[inline]
    fn wzwz(self) -> Self {
        let wzwz = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzwz) },
        }
    }

    #[inline]
    fn wzww(self) -> Self {
        let wzww = std::simd::simd_swizzle!(self.value.to_int(), [3, 2, 3, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wzww) },
        }
    }

    #[inline]
    fn wwxx(self) -> Self {
        let wwxx = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 0, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwxx) },
        }
    }

    #[inline]
    fn wwxy(self) -> Self {
        let wwxy = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 0, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwxy) },
        }
    }

    #[inline]
    fn wwxz(self) -> Self {
        let wwxz = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 0, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwxz) },
        }
    }

    #[inline]
    fn wwxw(self) -> Self {
        let wwxw = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 0, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwxw) },
        }
    }

    #[inline]
    fn wwyx(self) -> Self {
        let wwyx = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 1, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwyx) },
        }
    }

    #[inline]
    fn wwyy(self) -> Self {
        let wwyy = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 1, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwyy) },
        }
    }

    #[inline]
    fn wwyz(self) -> Self {
        let wwyz = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 1, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwyz) },
        }
    }

    #[inline]
    fn wwyw(self) -> Self {
        let wwyw = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 1, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwyw) },
        }
    }

    #[inline]
    fn wwzx(self) -> Self {
        let wwzx = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 2, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwzx) },
        }
    }

    #[inline]
    fn wwzy(self) -> Self {
        let wwzy = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 2, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwzy) },
        }
    }

    #[inline]
    fn wwzz(self) -> Self {
        let wwzz = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 2, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwzz) },
        }
    }

    #[inline]
    fn wwzw(self) -> Self {
        let wwzw = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 2, 3]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwzw) },
        }
    }

    #[inline]
    fn wwwx(self) -> Self {
        let wwwx = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 3, 0]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwwx) },
        }
    }

    #[inline]
    fn wwwy(self) -> Self {
        let wwwy = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 3, 1]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwwy) },
        }
    }

    #[inline]
    fn wwwz(self) -> Self {
        let wwwz = std::simd::simd_swizzle!(self.value.to_int(), [3, 3, 3, 2]);

        Bool4 {
            value: unsafe { std::simd::mask32x4::from_int_unchecked(wwwz) },
        }
    }

    #[inline]
    fn wwww(self) -> Self {
        Bool4::broadcast(self.w())
    }
}
