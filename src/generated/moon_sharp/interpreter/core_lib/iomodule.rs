
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/iomodule/IoModule.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.CoreLib", name = "IoModule")]
#[parent(crate::system::object::Object)]
pub struct IoModule {}

#[cfg(feature = "moon_sharp-interpreter-core_lib-iomodule")]
#[::unity2::methods]
impl IoModule {
    #[method(name = "MoonSharpInit", args = 2)]
    pub fn moon_sharp_init(
        global_table: crate::moon_sharp::interpreter::table::Table,
        io_table: crate::moon_sharp::interpreter::table::Table,
    ) -> ();

    #[method(name = "__index_callback", args = 2)]
    pub fn index_callback(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetStandardFile", args = 2)]
    pub fn get_standard_file(
        s: crate::moon_sharp::interpreter::script::Script,
        file: crate::moon_sharp::interpreter::platforms::standardfiletype::StandardFileType,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetStandardFile", args = 3)]
    pub fn set_standard_file(
        s: crate::moon_sharp::interpreter::script::Script,
        file: crate::moon_sharp::interpreter::platforms::standardfiletype::StandardFileType,
        options_stream: crate::system::io::stream::Stream,
    ) -> ();

    #[method(name = "GetDefaultFile", args = 2)]
    pub fn get_default_file(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        file: crate::moon_sharp::interpreter::platforms::standardfiletype::StandardFileType,
    ) -> crate::moon_sharp::interpreter::core_lib::io::fileuserdatabase::FileUserDataBase;

    #[method(name = "SetDefaultFile", args = 3)]
    pub fn set_default_file(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        file: crate::moon_sharp::interpreter::platforms::standardfiletype::StandardFileType,
        file_handle : crate :: moon_sharp :: interpreter :: core_lib :: io :: fileuserdatabase :: FileUserDataBase,
    ) -> ();

    #[method(name = "SetDefaultFile", args = 3)]
    pub fn set_default_file_2(
        script: crate::moon_sharp::interpreter::script::Script,
        file: crate::moon_sharp::interpreter::platforms::standardfiletype::StandardFileType,
        file_handle : crate :: moon_sharp :: interpreter :: core_lib :: io :: fileuserdatabase :: FileUserDataBase,
    ) -> ();

    #[method(name = "SetDefaultFile", args = 3)]
    pub fn set_default_file_3(
        script: crate::moon_sharp::interpreter::script::Script,
        file: crate::moon_sharp::interpreter::platforms::standardfiletype::StandardFileType,
        stream: crate::system::io::stream::Stream,
    ) -> ();

    #[method(name = "close", args = 2)]
    pub fn close(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "flush", args = 2)]
    pub fn flush(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "input", args = 2)]
    pub fn input(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "output", args = 2)]
    pub fn output(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "HandleDefaultStreamSetter", args = 3)]
    pub fn handle_default_stream_setter(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        default_files : crate :: moon_sharp :: interpreter :: platforms :: standardfiletype :: StandardFileType,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "lines", args = 2)]
    pub fn lines(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "open", args = 2)]
    pub fn open(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "type", args = 2)]
    pub fn r#type(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "read", args = 2)]
    pub fn read(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "write", args = 2)]
    pub fn write(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "tmpfile", args = 2)]
    pub fn tmpfile(
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-iomodule")]
impl IoModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(IoModule),
                ::core::stringify!(new),
            )
        });
        <Self as IIoModuleMethods>::ctor(this);
        this
    }
}
