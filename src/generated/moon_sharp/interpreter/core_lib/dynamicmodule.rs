
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/dynamicmodule/DynamicModule_DynamicExprWrapper.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.CoreLib",
    name = "DynamicModule.DynamicExprWrapper"
)]
#[parent(crate::system::object::Object)]
pub struct DynamicModule_DynamicExprWrapper {
    #[rename(name = "Expr")]
    pub expr: crate::moon_sharp::interpreter::dynamicexpression::DynamicExpression,
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-dynamicmodule")]
#[::unity2::methods]
impl DynamicModule_DynamicExprWrapper {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-dynamicmodule")]
impl DynamicModule_DynamicExprWrapper {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynamicModule_DynamicExprWrapper),
                ::core::stringify!(new),
            )
        });
        <Self as IDynamicModule_DynamicExprWrapperMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/dynamicmodule/DynamicModule.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.CoreLib", name = "DynamicModule")]
#[parent(crate::system::object::Object)]
pub struct DynamicModule {}

#[cfg(feature = "moon_sharp-interpreter-core_lib-dynamicmodule")]
#[::unity2::methods]
impl DynamicModule {
    #[method(name = "MoonSharpInit", args = 2)]
    pub fn moon_sharp_init(
        global_table: crate::moon_sharp::interpreter::table::Table,
        string_table: crate::moon_sharp::interpreter::table::Table,
    ) -> ();

    #[method(name = "eval", args = 2)]
    pub fn eval(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "prepare", args = 2)]
    pub fn prepare(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-dynamicmodule")]
impl DynamicModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynamicModule),
                ::core::stringify!(new),
            )
        });
        <Self as IDynamicModuleMethods>::ctor(this);
        this
    }
}
