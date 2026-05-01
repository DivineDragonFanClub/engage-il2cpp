
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/errorhandlingmodule/ErrorHandlingModule.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.CoreLib",
    name = "ErrorHandlingModule"
)]
#[parent(crate::system::object::Object)]
pub struct ErrorHandlingModule {}

#[cfg(feature = "moon_sharp-interpreter-core_lib-errorhandlingmodule")]
#[::unity2::methods]
impl ErrorHandlingModule {
    #[method(name = "pcall", args = 2)]
    pub fn pcall(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetErrorHandlerStrategy", args = 4)]
    pub fn set_error_handler_strategy(
        func_name: ::unity2::Il2CppString,
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        handler_before_unwind: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MakeReturnTuple", args = 2)]
    pub fn make_return_tuple(
        retstatus: bool,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "pcall_continuation", args = 2)]
    pub fn pcall_continuation(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "pcall_onerror", args = 2)]
    pub fn pcall_onerror(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "xpcall", args = 2)]
    pub fn xpcall(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-errorhandlingmodule")]
impl ErrorHandlingModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ErrorHandlingModule),
                ::core::stringify!(new),
            )
        });
        <Self as IErrorHandlingModuleMethods>::ctor(this);
        this
    }
}
