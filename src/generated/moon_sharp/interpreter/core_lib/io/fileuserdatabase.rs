
use crate::moon_sharp::interpreter::refidobject::IRefIdObject;
use crate::moon_sharp::interpreter::refidobject::RefIdObject;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/io/fileuserdatabase/FileUserDataBase.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.CoreLib.IO",
    name = "FileUserDataBase"
)]
#[parent(crate::moon_sharp::interpreter::refidobject::RefIdObject)]
pub struct FileUserDataBase {}

#[cfg(feature = "moon_sharp-interpreter-core_lib-io-fileuserdatabase")]
#[::unity2::methods]
impl FileUserDataBase {
    #[method(name = "lines", args = 2)]
    pub fn lines(
        self,
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "read", args = 2)]
    pub fn read(
        self,
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "write", args = 2)]
    pub fn write(
        self,
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "close", args = 2)]
    pub fn close(
        self,
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "IsNumericChar", args = 2)]
    pub fn is_numeric_char(self, c: u16, num_as_far: ::unity2::Il2CppString) -> bool;

    #[method(name = "Eof", args = 0)]
    pub fn eof(self) -> bool;

    #[method(name = "ReadLine", args = 0)]
    pub fn read_line(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadBuffer", args = 1)]
    pub fn read_buffer(self, p: i32) -> ::unity2::Il2CppString;

    #[method(name = "ReadToEnd", args = 0)]
    pub fn read_to_end(self) -> ::unity2::Il2CppString;

    #[method(name = "Peek", args = 0)]
    pub fn peek(self) -> u16;

    #[method(name = "Write", args = 1)]
    pub fn write_2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "isopen", args = 0)]
    pub fn isopen(self) -> bool;

    #[method(name = "Close", args = 0)]
    pub fn close_2(self) -> ::unity2::Il2CppString;

    #[method(name = "flush", args = 0)]
    pub fn flush(self) -> bool;

    #[method(name = "seek", args = 2)]
    pub fn seek(self, whence: ::unity2::Il2CppString, offset: i64) -> i64;

    #[method(name = "setvbuf", args = 1)]
    pub fn setvbuf(self, mode: ::unity2::Il2CppString) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-io-fileuserdatabase")]
impl FileUserDataBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FileUserDataBase),
                ::core::stringify!(new),
            )
        });
        <Self as IFileUserDataBaseMethods>::ctor(this);
        this
    }
}
