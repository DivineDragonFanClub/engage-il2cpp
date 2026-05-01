
use crate::moon_sharp::interpreter::core_lib::io::fileuserdatabase::FileUserDataBase;
use crate::moon_sharp::interpreter::core_lib::io::fileuserdatabase::IFileUserDataBase;
use crate::moon_sharp::interpreter::refidobject::IRefIdObject;
use crate::moon_sharp::interpreter::refidobject::RefIdObject;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/io/streamfileuserdatabase/StreamFileUserDataBase.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.CoreLib.IO",
    name = "StreamFileUserDataBase"
)]
#[parent(crate::moon_sharp::interpreter::core_lib::io::fileuserdatabase::FileUserDataBase)]
pub struct StreamFileUserDataBase {
    #[rename(name = "m_Stream")]
    pub m_stream: crate::system::io::stream::Stream,
    #[rename(name = "m_Closed")]
    pub m_closed: bool,
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-io-streamfileuserdatabase")]
#[::unity2::methods]
impl StreamFileUserDataBase {
    #[method(name = "CheckFileIsNotClosed", args = 0)]
    pub fn check_file_is_not_closed(self) -> ();

    #[method(name = "Eof", args = 0)]
    pub fn eof(self) -> bool;

    #[method(name = "ReadLine", args = 0)]
    pub fn read_line(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadToEnd", args = 0)]
    pub fn read_to_end(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadBuffer", args = 1)]
    pub fn read_buffer(self, p: i32) -> ::unity2::Il2CppString;

    #[method(name = "Peek", args = 0)]
    pub fn peek(self) -> u16;

    #[method(name = "Write", args = 1)]
    pub fn write(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ::unity2::Il2CppString;

    #[method(name = "flush", args = 0)]
    pub fn flush(self) -> bool;

    #[method(name = "seek", args = 2)]
    pub fn seek(self, whence: ::unity2::Il2CppString, offset: i64) -> i64;

    #[method(name = "setvbuf", args = 1)]
    pub fn setvbuf(self, mode: ::unity2::Il2CppString) -> bool;

    #[method(name = "isopen", args = 0)]
    pub fn isopen(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-io-streamfileuserdatabase")]
impl StreamFileUserDataBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StreamFileUserDataBase),
                ::core::stringify!(new),
            )
        });
        <Self as IStreamFileUserDataBaseMethods>::ctor(this);
        this
    }
}
