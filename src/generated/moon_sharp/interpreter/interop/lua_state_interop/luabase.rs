
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/lua_state_interop/luabase/LuaBase.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.LuaStateInterop",
    name = "LuaBase"
)]
#[parent(crate::system::object::Object)]
pub struct LuaBase {
    #[static_field]
    #[rename(name = "LUA_TNONE")]
    pub lua_tnone: i32,
    #[static_field]
    #[rename(name = "LUA_TNIL")]
    pub lua_tnil: i32,
    #[static_field]
    #[rename(name = "LUA_TBOOLEAN")]
    pub lua_tboolean: i32,
    #[static_field]
    #[rename(name = "LUA_TLIGHTUSERDATA")]
    pub lua_tlightuserdata: i32,
    #[static_field]
    #[rename(name = "LUA_TNUMBER")]
    pub lua_tnumber: i32,
    #[static_field]
    #[rename(name = "LUA_TSTRING")]
    pub lua_tstring: i32,
    #[static_field]
    #[rename(name = "LUA_TTABLE")]
    pub lua_ttable: i32,
    #[static_field]
    #[rename(name = "LUA_TFUNCTION")]
    pub lua_tfunction: i32,
    #[static_field]
    #[rename(name = "LUA_TUSERDATA")]
    pub lua_tuserdata: i32,
    #[static_field]
    #[rename(name = "LUA_TTHREAD")]
    pub lua_tthread: i32,
    #[static_field]
    #[rename(name = "LUA_MULTRET")]
    pub lua_multret: i32,
    #[static_field]
    #[rename(name = "LUA_INTFRMLEN")]
    pub lua_intfrmlen: ::unity2::Il2CppString,
}

#[cfg(feature = "moon_sharp-interpreter-interop-lua_state_interop-luabase")]
#[::unity2::methods]
impl LuaBase {
    #[method(name = "GetArgument", args = 2)]
    pub fn get_argument(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        pos: i32,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ArgAsType", args = 4)]
    pub fn arg_as_type(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        pos: i32,
        r#type: crate::moon_sharp::interpreter::datatype::DataType,
        allow_nil: bool,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "LuaType", args = 2)]
    pub fn lua_type(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> i32;

    #[method(name = "LuaLCheckLString", args = 3)]
    pub fn lua_l_check_l_string(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        arg_num: i32,
        l_2: u32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "LuaPushInteger", args = 2)]
    pub fn lua_push_integer(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        val: i32,
    ) -> ();

    #[method(name = "LuaToBoolean", args = 2)]
    pub fn lua_to_boolean(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> i32;

    #[method(name = "LuaToLString", args = 3)]
    pub fn lua_to_l_string(
        lua_state: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
        l: u32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "LuaToString", args = 2)]
    pub fn lua_to_string(
        lua_state: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "LuaLAddValue", args = 1)]
    pub fn lua_l_add_value(
        b: crate::moon_sharp::interpreter::interop::lua_state_interop::lualbuffer::LuaLBuffer,
    ) -> ();

    #[method(name = "LuaLAddLString", args = 3)]
    pub fn lua_l_add_l_string(
        b: crate::moon_sharp::interpreter::interop::lua_state_interop::lualbuffer::LuaLBuffer,
        s: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        p: u32,
    ) -> ();

    #[method(name = "LuaLAddString", args = 2)]
    pub fn lua_l_add_string(
        b: crate::moon_sharp::interpreter::interop::lua_state_interop::lualbuffer::LuaLBuffer,
        s: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "LuaLOptInteger", args = 3)]
    pub fn lua_l_opt_integer(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        pos: i32,
        def: i32,
    ) -> i32;

    #[method(name = "LuaLCheckInteger", args = 2)]
    pub fn lua_l_check_integer(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        pos: i32,
    ) -> i32;

    #[method(name = "LuaLArgCheck", args = 4)]
    pub fn lua_l_arg_check(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        condition: bool,
        arg_num: i32,
        message: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "LuaLCheckInt", args = 2)]
    pub fn lua_l_check_int(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        arg_num: i32,
    ) -> i32;

    #[method(name = "LuaGetTop", args = 1)]
    pub fn lua_get_top(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
    ) -> i32;

    #[method(name = "LuaLError", args = 3)]
    pub fn lua_l_error(
        lua_state: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        message: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> i32;

    #[method(name = "LuaLAddChar", args = 2)]
    pub fn lua_l_add_char(
        b: crate::moon_sharp::interpreter::interop::lua_state_interop::lualbuffer::LuaLBuffer,
        p: u16,
    ) -> ();

    #[method(name = "LuaLBuffInit", args = 2)]
    pub fn lua_l_buff_init(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        b: crate::moon_sharp::interpreter::interop::lua_state_interop::lualbuffer::LuaLBuffer,
    ) -> ();

    #[method(name = "LuaPushLiteral", args = 2)]
    pub fn lua_push_literal(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        literal_string: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "LuaLPushResult", args = 1)]
    pub fn lua_l_push_result(
        b: crate::moon_sharp::interpreter::interop::lua_state_interop::lualbuffer::LuaLBuffer,
    ) -> ();

    #[method(name = "LuaPushLString", args = 3)]
    pub fn lua_push_l_string(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        s: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        len: u32,
    ) -> ();

    #[method(name = "LuaLCheckStack", args = 3)]
    pub fn lua_l_check_stack(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        n: i32,
        message: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "LUA_QL", args = 1)]
    pub fn lua_ql(p: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "LuaPushNil", args = 1)]
    pub fn lua_push_nil(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
    ) -> ();

    #[method(name = "LuaAssert", args = 1)]
    pub fn lua_assert(p: bool) -> ();

    #[method(name = "LuaLTypeName", args = 2)]
    pub fn lua_l_type_name(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "LuaIsString", args = 2)]
    pub fn lua_is_string(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> i32;

    #[method(name = "LuaPop", args = 2)]
    pub fn lua_pop(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> ();

    #[method(name = "LuaGetTable", args = 2)]
    pub fn lua_get_table(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> ();

    #[method(name = "LuaLOptInt", args = 3)]
    pub fn lua_l_opt_int(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        pos: i32,
        def: i32,
    ) -> i32;

    #[method(name = "LuaLCheckString", args = 2)]
    pub fn lua_l_check_string(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr;

    #[method(name = "LuaLCheckStringStr", args = 2)]
    pub fn lua_l_check_string_str(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        p: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "LuaLArgError", args = 3)]
    pub fn lua_l_arg_error(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        arg: i32,
        p: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "LuaLCheckNumber", args = 2)]
    pub fn lua_l_check_number(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        pos: i32,
    ) -> f64;

    #[method(name = "LuaPushValue", args = 2)]
    pub fn lua_push_value(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        arg: i32,
    ) -> ();

    #[method(name = "LuaCall", args = 3)]
    pub fn lua_call(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
        nargs: i32,
        nresults: i32,
    ) -> ();

    #[method(name = "memcmp", args = 3)]
    pub fn memcmp(
        ptr1: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        ptr2: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        size: u32,
    ) -> i32;

    #[method(name = "memcmp", args = 3)]
    pub fn memcmp_2(
        ptr1: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        ptr2: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        size: i32,
    ) -> i32;

    #[method(name = "memchr", args = 3)]
    pub fn memchr(
        ptr: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        c: u16,
        count: u32,
    ) -> crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr;

    #[method(name = "strpbrk", args = 2)]
    pub fn strpbrk(
        str: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        charset: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
    ) -> crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr;

    #[method(name = "isalpha", args = 1)]
    pub fn isalpha(c: u16) -> bool;

    #[method(name = "iscntrl", args = 1)]
    pub fn iscntrl(c: u16) -> bool;

    #[method(name = "isdigit", args = 1)]
    pub fn isdigit(c: u16) -> bool;

    #[method(name = "islower", args = 1)]
    pub fn islower(c: u16) -> bool;

    #[method(name = "ispunct", args = 1)]
    pub fn ispunct(c: u16) -> bool;

    #[method(name = "isspace", args = 1)]
    pub fn isspace(c: u16) -> bool;

    #[method(name = "isupper", args = 1)]
    pub fn isupper(c: u16) -> bool;

    #[method(name = "isalnum", args = 1)]
    pub fn isalnum(c: u16) -> bool;

    #[method(name = "isxdigit", args = 1)]
    pub fn isxdigit(c: u16) -> bool;

    #[method(name = "isgraph", args = 1)]
    pub fn isgraph(c: u16) -> bool;

    #[method(name = "isalpha", args = 1)]
    pub fn isalpha_2(c: i32) -> bool;

    #[method(name = "iscntrl", args = 1)]
    pub fn iscntrl_2(c: i32) -> bool;

    #[method(name = "isdigit", args = 1)]
    pub fn isdigit_2(c: i32) -> bool;

    #[method(name = "islower", args = 1)]
    pub fn islower_2(c: i32) -> bool;

    #[method(name = "ispunct", args = 1)]
    pub fn ispunct_2(c: i32) -> bool;

    #[method(name = "isspace", args = 1)]
    pub fn isspace_2(c: i32) -> bool;

    #[method(name = "isupper", args = 1)]
    pub fn isupper_2(c: i32) -> bool;

    #[method(name = "isalnum", args = 1)]
    pub fn isalnum_2(c: i32) -> bool;

    #[method(name = "isgraph", args = 1)]
    pub fn isgraph_2(c: i32) -> bool;

    #[method(name = "tolower", args = 1)]
    pub fn tolower(c: u16) -> u16;

    #[method(name = "toupper", args = 1)]
    pub fn toupper(c: u16) -> u16;

    #[method(name = "tolower", args = 1)]
    pub fn tolower_2(c: i32) -> u16;

    #[method(name = "toupper", args = 1)]
    pub fn toupper_2(c: i32) -> u16;

    #[method(name = "strchr", args = 2)]
    pub fn strchr(
        str: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        c: u16,
    ) -> crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr;

    #[method(name = "strcpy", args = 2)]
    pub fn strcpy(
        dst: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        src: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
    ) -> crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr;

    #[method(name = "strncpy", args = 3)]
    pub fn strncpy(
        dst: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        src: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        length: i32,
    ) -> crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr;

    #[method(name = "strlen", args = 1)]
    pub fn strlen(
        str: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
    ) -> i32;

    #[method(name = "sprintf", args = 3)]
    pub fn sprintf(
        buffer: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        str: crate::moon_sharp::interpreter::interop::lua_state_interop::charptr::CharPtr,
        argv: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-lua_state_interop-luabase")]
impl LuaBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LuaBase),
                ::core::stringify!(new),
            )
        });
        <Self as ILuaBaseMethods>::ctor(this);
        this
    }
}
