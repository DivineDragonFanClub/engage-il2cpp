
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/stringmodule/StringModule.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.CoreLib", name = "StringModule")]
#[parent(crate::system::object::Object)]
pub struct StringModule {
    #[static_field]
    #[rename(name = "BASE64_DUMP_HEADER")]
    pub base64_dump_header: ::unity2::Il2CppString,
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-stringmodule")]
#[::unity2::methods]
impl StringModule {
    #[method(name = "MoonSharpInit", args = 2)]
    pub fn moon_sharp_init(
        global_table: crate::moon_sharp::interpreter::table::Table,
        string_table: crate::moon_sharp::interpreter::table::Table,
    ) -> ();

    #[method(name = "dump", args = 2)]
    pub fn dump(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "char", args = 2)]
    pub fn char(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "byte", args = 2)]
    pub fn byte(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "unicode", args = 2)]
    pub fn unicode(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Unicode2Ascii", args = 1)]
    pub fn unicode2_ascii(i: i32) -> i32;

    #[method(name = "PerformByteLike", args = 4)]
    pub fn perform_byte_like(
        vs: crate::moon_sharp::interpreter::dynvalue::DynValue,
        vi: crate::moon_sharp::interpreter::dynvalue::DynValue,
        vj: crate::moon_sharp::interpreter::dynvalue::DynValue,
        filter: crate::system::func_2::Func_2<i32, i32>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "len", args = 2)]
    pub fn len(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "lower", args = 2)]
    pub fn lower(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "upper", args = 2)]
    pub fn upper(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "rep", args = 2)]
    pub fn rep(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "reverse", args = 2)]
    pub fn reverse(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "sub", args = 2)]
    pub fn sub(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "startsWith", args = 2)]
    pub fn starts_with(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "endsWith", args = 2)]
    pub fn ends_with(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "contains", args = 2)]
    pub fn contains(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-stringmodule")]
impl StringModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringModule),
                ::core::stringify!(new),
            )
        });
        <Self as IStringModuleMethods>::ctor(this);
        this
    }
}
