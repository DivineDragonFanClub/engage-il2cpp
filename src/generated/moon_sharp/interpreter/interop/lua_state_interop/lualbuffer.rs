
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/lua_state_interop/lualbuffer/LuaLBuffer.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.LuaStateInterop",
    name = "LuaLBuffer"
)]
#[parent(crate::system::object::Object)]
pub struct LuaLBuffer {}

#[cfg(feature = "moon_sharp-interpreter-interop-lua_state_interop-lualbuffer")]
#[::unity2::methods]
impl LuaLBuffer {
    #[method(name = "get_LuaState", args = 0)]
    pub fn get_lua_state(
        self,
    ) -> crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState;

    #[method(name = "set_LuaState", args = 1)]
    pub fn set_lua_state(
        self,
        value: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-lua_state_interop-lualbuffer")]
impl LuaLBuffer {
    pub fn new(
        l: crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LuaLBuffer),
                ::core::stringify!(new),
            )
        });
        <Self as ILuaLBufferMethods>::ctor(this, l);
        this
    }
}
