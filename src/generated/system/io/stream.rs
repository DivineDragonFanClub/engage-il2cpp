
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/io/stream/Stream.md")))]
#[::unity2::class(namespace = "System.IO", name = "Stream")]
pub struct Stream {
    #[static_field]
    #[rename(name = "Null")]
    pub null: crate::system::io::stream::Stream,
}

#[cfg(feature = "system-io-stream")]
#[::unity2::methods]
impl Stream {
    #[method(name = "get_CanRead", args = 0)]
    pub fn get_can_read(self) -> bool;

    #[method(name = "get_CanSeek", args = 0)]
    pub fn get_can_seek(self) -> bool;

    #[method(name = "get_CanTimeout", args = 0)]
    pub fn get_can_timeout(self) -> bool;

    #[method(name = "get_CanWrite", args = 0)]
    pub fn get_can_write(self) -> bool;

    #[method(name = "get_Length", args = 0)]
    pub fn get_length(self) -> i64;

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> i64;

    #[method(name = "set_Position", args = 1)]
    pub fn set_position(self, value: i64) -> ();

    #[method(name = "get_ReadTimeout", args = 0)]
    pub fn get_read_timeout(self) -> i32;

    #[method(name = "set_ReadTimeout", args = 1)]
    pub fn set_read_timeout(self, value: i32) -> ();

    #[method(name = "get_WriteTimeout", args = 0)]
    pub fn get_write_timeout(self) -> i32;

    #[method(name = "set_WriteTimeout", args = 1)]
    pub fn set_write_timeout(self, value: i32) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();

    #[method(name = "Flush", args = 0)]
    pub fn flush(self) -> ();

    #[method(name = "SetLength", args = 1)]
    pub fn set_length(self, value: i64) -> ();

    #[method(name = "Read", args = 3)]
    pub fn read(self, buffer: ::unity2::Array<u8>, offset: i32, count: i32) -> i32;

    #[method(name = "ReadByte", args = 0)]
    pub fn read_byte(self) -> i32;

    #[method(name = "Write", args = 3)]
    pub fn write(self, buffer: ::unity2::Array<u8>, offset: i32, count: i32) -> ();

    #[method(name = "WriteByte", args = 1)]
    pub fn write_byte(self, value: u8) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-io-stream")]
impl Stream {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Stream),
                ::core::stringify!(new),
            )
        });
        <Self as IStreamMethods>::ctor(this);
        this
    }
}
