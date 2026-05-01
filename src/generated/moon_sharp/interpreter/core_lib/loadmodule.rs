
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/loadmodule/LoadModule.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.CoreLib", name = "LoadModule")]
#[parent(crate::system::object::Object)]
pub struct LoadModule {
    #[static_field]
    #[rename(name = "require")]
    pub require: ::unity2::Il2CppString,
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-loadmodule")]
#[::unity2::methods]
impl LoadModule {
    #[method(name = "MoonSharpInit", args = 2)]
    pub fn moon_sharp_init(
        global_table: crate::moon_sharp::interpreter::table::Table,
        io_table: crate::moon_sharp::interpreter::table::Table,
    ) -> ();

    #[method(name = "load", args = 2)]
    pub fn load(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "loadsafe", args = 2)]
    pub fn loadsafe(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "load_impl", args = 3)]
    pub fn load_impl(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        default_env: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "loadfile", args = 2)]
    pub fn loadfile(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "loadfilesafe", args = 2)]
    pub fn loadfilesafe(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "loadfile_impl", args = 3)]
    pub fn loadfile_impl(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        default_env: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetSafeDefaultEnv", args = 1)]
    pub fn get_safe_default_env(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "dofile", args = 2)]
    pub fn dofile(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "__require_clr_impl", args = 2)]
    pub fn require_clr_impl(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-loadmodule")]
impl LoadModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LoadModule),
                ::core::stringify!(new),
            )
        });
        <Self as ILoadModuleMethods>::ctor(this);
        this
    }
}
