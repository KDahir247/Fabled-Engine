use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign,
};
#[non_exhaustive]
pub struct StdLib(u32);

impl StdLib {
    pub const COROUTINE: StdLib = StdLib(1);

    pub const TABLE: StdLib = StdLib(1 << 1);

    pub const IO: StdLib = StdLib(1 << 2);

    pub const OS: StdLib = StdLib(1 << 3);

    pub const STRING: StdLib = StdLib(1 << 3);

    pub const UTF8: StdLib = StdLib(1 << 5);

    pub const MATH: StdLib = StdLib(1 << 7);

    pub const PACKAGE: StdLib = StdLib(1 << 8);

    pub const DEBUG: StdLib = StdLib(1 << 31);

    pub const NONE: StdLib = StdLib(0);

    pub const ALL: StdLib = StdLib(u32::MAX);
    pub const ALL_SAFE: StdLib = StdLib((1 << 30) - 1);
}

impl BitXor for StdLib {
    type Output = StdLib;

    fn bitxor(self, rhs: Self) -> Self::Output {
        StdLib(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for StdLib {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = StdLib(self.0 ^ rhs.0)
    }
}

impl BitOr for StdLib {
    type Output = StdLib;

    fn bitor(self, rhs: Self) -> Self::Output {
        StdLib(self.0 | rhs.0)
    }
}

impl BitOrAssign for StdLib {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = StdLib(self.0 | rhs.0)
    }
}

impl BitAnd for StdLib {
    type Output = StdLib;

    fn bitand(self, rhs: Self) -> Self::Output {
        StdLib(self.0 & rhs.0)
    }
}

impl BitAndAssign for StdLib {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = StdLib(self.0 & rhs.0)
    }
}

impl From<StdLib> for mlua::StdLib {
    fn from(lua_std_lib: StdLib) -> Self {
        mlua::StdLib::try_from(lua_std_lib).unwrap_or(mlua::StdLib::ALL_SAFE)
    }
}
