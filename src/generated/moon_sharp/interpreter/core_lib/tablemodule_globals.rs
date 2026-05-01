
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/tablemodule_globals/TableModule_Globals.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.CoreLib",
    name = "TableModule_Globals"
)]
#[parent(crate::system::object::Object)]
pub struct TableModule_Globals {}

#[cfg(feature = "moon_sharp-interpreter-core_lib-tablemodule_globals")]
#[::unity2::methods]
impl TableModule_Globals {
    #[method(name = "unpack", args = 2)]
    pub fn unpack(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "pack", args = 2)]
    pub fn pack(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-tablemodule_globals")]
impl TableModule_Globals {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TableModule_Globals),
                ::core::stringify!(new),
            )
        });
        <Self as ITableModule_GlobalsMethods>::ctor(this);
        this
    }
}
