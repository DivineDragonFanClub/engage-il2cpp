
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/scriptexecutioncontext/ScriptExecutionContext.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "ScriptExecutionContext")]
#[parent(crate::system::object::Object)]
pub struct ScriptExecutionContext {
    #[rename(name = "m_Processor")]
    pub m_processor: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
    #[rename(name = "m_Callback")]
    pub m_callback: crate::moon_sharp::interpreter::callbackfunction::CallbackFunction,
}

#[cfg(feature = "moon_sharp-interpreter-scriptexecutioncontext")]
#[::unity2::methods]
impl ScriptExecutionContext {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        p: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
        call_back_function: crate::moon_sharp::interpreter::callbackfunction::CallbackFunction,
        source_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        is_dynamic: bool,
    ) -> ();

    #[method(name = "get_IsDynamicExecution", args = 0)]
    pub fn get_is_dynamic_execution(self) -> bool;

    #[method(name = "set_IsDynamicExecution", args = 1)]
    pub fn set_is_dynamic_execution(self, value: bool) -> ();

    #[method(name = "get_CallingLocation", args = 0)]
    pub fn get_calling_location(
        self,
    ) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "set_CallingLocation", args = 1)]
    pub fn set_calling_location(
        self,
        value: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> ();

    #[method(name = "get_AdditionalData", args = 0)]
    pub fn get_additional_data(self) -> crate::system::object::Object;

    #[method(name = "set_AdditionalData", args = 1)]
    pub fn set_additional_data(self, value: crate::system::object::Object) -> ();

    #[method(name = "GetMetatable", args = 1)]
    pub fn get_metatable(
        self,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "GetMetamethod", args = 2)]
    pub fn get_metamethod(
        self,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        metamethod: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetMetamethodTailCall", args = 3)]
    pub fn get_metamethod_tail_call(
        self,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        metamethod: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetBinaryMetamethod", args = 3)]
    pub fn get_binary_metamethod(
        self,
        op1: crate::moon_sharp::interpreter::dynvalue::DynValue,
        op2: crate::moon_sharp::interpreter::dynvalue::DynValue,
        event_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetScript", args = 0)]
    pub fn get_script(self) -> crate::moon_sharp::interpreter::script::Script;

    #[method(name = "GetCallingCoroutine", args = 0)]
    pub fn get_calling_coroutine(self) -> crate::moon_sharp::interpreter::coroutine_2::Coroutine_2;

    #[method(name = "EmulateClassicCall", args = 3)]
    pub fn emulate_classic_call(
        self,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        function_name: ::unity2::Il2CppString,
        callback: crate::system::func_2::Func_2<
            crate::moon_sharp::interpreter::interop::lua_state_interop::luastate::LuaState,
            i32,
        >,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 2)]
    pub fn call(
        self,
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "EvaluateSymbol", args = 1)]
    pub fn evaluate_symbol(
        self,
        symref: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "EvaluateSymbolByName", args = 1)]
    pub fn evaluate_symbol_by_name(
        self,
        symbol: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "FindSymbolByName", args = 1)]
    pub fn find_symbol_by_name(
        self,
        symbol: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "get_CurrentGlobalEnv", args = 0)]
    pub fn get_current_global_env(self) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "PerformMessageDecorationBeforeUnwind", args = 2)]
    pub fn perform_message_decoration_before_unwind(
        self,
        message_handler: crate::moon_sharp::interpreter::dynvalue::DynValue,
        exception: crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException,
    ) -> ();

    #[method(name = "get_OwnerScript", args = 0)]
    pub fn get_owner_script(self) -> crate::moon_sharp::interpreter::script::Script;
}

#[cfg(feature = "moon_sharp-interpreter-scriptexecutioncontext")]
impl ScriptExecutionContext {
    pub fn new(
        p: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
        call_back_function: crate::moon_sharp::interpreter::callbackfunction::CallbackFunction,
        source_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        is_dynamic: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptExecutionContext),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptExecutionContextMethods>::ctor(
            this,
            p,
            call_back_function,
            source_ref,
            is_dynamic,
        );
        this
    }
}
