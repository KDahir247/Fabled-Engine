pub struct StdLib;

impl StdLib {
    pub const COROUTINE: mlua::StdLib = mlua::StdLib::COROUTINE;

    pub const TABLE: mlua::StdLib = mlua::StdLib::TABLE;

    pub const IO: mlua::StdLib = mlua::StdLib::IO;

    pub const OS: mlua::StdLib = mlua::StdLib::OS;

    pub const STRING: mlua::StdLib = mlua::StdLib::STRING;

    pub const UTF8: mlua::StdLib = mlua::StdLib::UTF8;

    pub const MATH: mlua::StdLib = mlua::StdLib::MATH;

    pub const PACKAGE: mlua::StdLib = mlua::StdLib::PACKAGE;

    pub const DEBUG: mlua::StdLib = mlua::StdLib::DEBUG;

    pub const NONE: mlua::StdLib = mlua::StdLib::NONE;

    pub const ALL: mlua::StdLib = mlua::StdLib::ALL;

    pub const ALL_SAFE: mlua::StdLib = mlua::StdLib::ALL_SAFE;
}
