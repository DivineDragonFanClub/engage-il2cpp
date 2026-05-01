
use crate::system::io::stream::IStream;
use crate::system::io::stream::Stream;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/io/undisposablestream/UndisposableStream.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.IO", name = "UndisposableStream")]
#[parent(crate::system::io::stream::Stream)]
pub struct UndisposableStream {
    #[rename(name = "m_Stream")]
    pub m_stream: crate::system::io::stream::Stream,
}

#[cfg(feature = "moon_sharp-interpreter-io-undisposablestream")]
#[::unity2::methods]
impl UndisposableStream {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, stream: crate::system::io::stream::Stream) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose(self, disposing: bool) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "get_CanRead", args = 0)]
    pub fn get_can_read(self) -> bool;

    #[method(name = "get_CanSeek", args = 0)]
    pub fn get_can_seek(self) -> bool;

    #[method(name = "get_CanWrite", args = 0)]
    pub fn get_can_write(self) -> bool;

    #[method(name = "Flush", args = 0)]
    pub fn flush(self) -> ();

    #[method(name = "get_Length", args = 0)]
    pub fn get_length(self) -> i64;

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> i64;

    #[method(name = "set_Position", args = 1)]
    pub fn set_position(self, value: i64) -> ();

    #[method(name = "Read", args = 3)]
    pub fn read(self, buffer: ::unity2::Array<u8>, offset: i32, count: i32) -> i32;

    #[method(name = "SetLength", args = 1)]
    pub fn set_length(self, value: i64) -> ();

    #[method(name = "Write", args = 3)]
    pub fn write(self, buffer: ::unity2::Array<u8>, offset: i32, count: i32) -> ();

    #[method(name = "get_CanTimeout", args = 0)]
    pub fn get_can_timeout(self) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ReadByte", args = 0)]
    pub fn read_byte(self) -> i32;

    #[method(name = "get_ReadTimeout", args = 0)]
    pub fn get_read_timeout(self) -> i32;

    #[method(name = "set_ReadTimeout", args = 1)]
    pub fn set_read_timeout(self, value: i32) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "WriteByte", args = 1)]
    pub fn write_byte(self, value: u8) -> ();

    #[method(name = "get_WriteTimeout", args = 0)]
    pub fn get_write_timeout(self) -> i32;

    #[method(name = "set_WriteTimeout", args = 1)]
    pub fn set_write_timeout(self, value: i32) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-io-undisposablestream")]
impl UndisposableStream {
    pub fn new(stream: crate::system::io::stream::Stream) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UndisposableStream),
                ::core::stringify!(new),
            )
        });
        <Self as IUndisposableStreamMethods>::ctor(this, stream);
        this
    }
}
