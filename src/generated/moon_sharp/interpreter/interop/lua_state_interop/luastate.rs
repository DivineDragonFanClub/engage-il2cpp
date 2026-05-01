
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/lua_state_interop/luastate/LuaState.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.LuaStateInterop",
    name = "LuaState"
)]
#[parent(crate::system::object::Object)]
pub struct LuaState {
    #[rename(name = "m_Stack")]
    pub m_stack: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::dynvalue::DynValue,
    >,
}

#[cfg(feature = "moon_sharp-interpreter-interop-lua_state_interop-luastate")]
#[::unity2::methods]
impl LuaState {
    #[method(name = "get_ExecutionContext", args = 0)]
    pub fn get_execution_context(
        self,
    ) -> crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext;

    #[method(name = "set_ExecutionContext", args = 1)]
    pub fn set_execution_context(
        self,
        value: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> ();

    #[method(name = "get_FunctionName", args = 0)]
    pub fn get_function_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FunctionName", args = 1)]
    pub fn set_function_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        function_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Top", args = 1)]
    pub fn top(self, pos: i32) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "At", args = 1)]
    pub fn at(self, pos: i32) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "Push", args = 1)]
    pub fn push(self, v: crate::moon_sharp::interpreter::dynvalue::DynValue) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetTopArray", args = 1)]
    pub fn get_top_array(
        self,
        num: i32,
    ) -> ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>;

    #[method(name = "GetReturnValue", args = 1)]
    pub fn get_return_value(
        self,
        retvals: i32,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Discard", args = 1)]
    pub fn discard(self, nargs: i32) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-lua_state_interop-luastate")]
impl LuaState {
    pub fn new(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        function_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LuaState),
                ::core::stringify!(new),
            )
        });
        <Self as ILuaStateMethods>::ctor(this, execution_context, args, function_name);
        this
    }
}
