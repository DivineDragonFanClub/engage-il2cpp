
use crate::system::io::stream::IStream;
use crate::system::io::stream::Stream;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/io/memorystream/MemoryStream.md")))]
#[::unity2::class(namespace = "System.IO", name = "MemoryStream")]
#[parent(crate::system::io::stream::Stream)]
pub struct MemoryStream {
    #[rename(name = "_buffer")]
    pub buffer: ::unity2::Array<u8>,
    #[rename(name = "_origin")]
    pub origin: i32,
    #[rename(name = "_expandable")]
    pub expandable: bool,
    #[rename(name = "_writable")]
    pub writable: bool,
    #[rename(name = "_exposable")]
    pub exposable: bool,
    #[rename(name = "_isOpen")]
    pub is_open: bool,
}

#[cfg(feature = "system-io-memorystream")]
#[::unity2::methods]
impl MemoryStream {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, buffer: ::unity2::Array<u8>) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(self, buffer: ::unity2::Array<u8>, writable: bool) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_5(self, buffer: ::unity2::Array<u8>, index: i32, count: i32) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_6(
        self,
        buffer: ::unity2::Array<u8>,
        index: i32,
        count: i32,
        writable: bool,
        publicly_visible: bool,
    ) -> ();

    #[method(name = "get_CanRead", args = 0)]
    pub fn get_can_read(self) -> bool;

    #[method(name = "get_CanSeek", args = 0)]
    pub fn get_can_seek(self) -> bool;

    #[method(name = "get_CanWrite", args = 0)]
    pub fn get_can_write(self) -> bool;

    #[method(name = "EnsureWriteable", args = 0)]
    pub fn ensure_writeable(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose(self, disposing: bool) -> ();

    #[method(name = "EnsureCapacity", args = 1)]
    pub fn ensure_capacity(self, value: i32) -> bool;

    #[method(name = "Flush", args = 0)]
    pub fn flush(self) -> ();

    #[method(name = "GetBuffer", args = 0)]
    pub fn get_buffer(self) -> ::unity2::Array<u8>;

    #[method(name = "InternalGetBuffer", args = 0)]
    pub fn internal_get_buffer(self) -> ::unity2::Array<u8>;

    #[method(name = "InternalGetPosition", args = 0)]
    pub fn internal_get_position(self) -> i32;

    #[method(name = "InternalReadInt32", args = 0)]
    pub fn internal_read_int32(self) -> i32;

    #[method(name = "InternalEmulateRead", args = 1)]
    pub fn internal_emulate_read(self, count: i32) -> i32;

    #[method(name = "get_Capacity", args = 0)]
    pub fn get_capacity(self) -> i32;

    #[method(name = "set_Capacity", args = 1)]
    pub fn set_capacity(self, value: i32) -> ();

    #[method(name = "get_Length", args = 0)]
    pub fn get_length(self) -> i64;

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> i64;

    #[method(name = "set_Position", args = 1)]
    pub fn set_position(self, value: i64) -> ();

    #[method(name = "Read", args = 3)]
    pub fn read(self, buffer: ::unity2::Array<u8>, offset: i32, count: i32) -> i32;

    #[method(name = "ReadByte", args = 0)]
    pub fn read_byte(self) -> i32;

    #[method(name = "SetLength", args = 1)]
    pub fn set_length(self, value: i64) -> ();

    #[method(name = "ToArray", args = 0)]
    pub fn to_array(self) -> ::unity2::Array<u8>;

    #[method(name = "Write", args = 3)]
    pub fn write(self, buffer: ::unity2::Array<u8>, offset: i32, count: i32) -> ();

    #[method(name = "WriteByte", args = 1)]
    pub fn write_byte(self, value: u8) -> ();
}

#[cfg(feature = "system-io-memorystream")]
impl MemoryStream {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MemoryStream),
                ::core::stringify!(new),
            )
        });
        <Self as IMemoryStreamMethods>::ctor(this);
        this
    }

    pub fn new_2(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MemoryStream),
                ::core::stringify!(new_2),
            )
        });
        <Self as IMemoryStreamMethods>::ctor_2(this, capacity);
        this
    }

    pub fn new_3(buffer: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MemoryStream),
                ::core::stringify!(new_3),
            )
        });
        <Self as IMemoryStreamMethods>::ctor_3(this, buffer);
        this
    }

    pub fn new_4(buffer: ::unity2::Array<u8>, writable: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MemoryStream),
                ::core::stringify!(new_4),
            )
        });
        <Self as IMemoryStreamMethods>::ctor_4(this, buffer, writable);
        this
    }

    pub fn new_5(buffer: ::unity2::Array<u8>, index: i32, count: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MemoryStream),
                ::core::stringify!(new_5),
            )
        });
        <Self as IMemoryStreamMethods>::ctor_5(this, buffer, index, count);
        this
    }

    pub fn new_6(
        buffer: ::unity2::Array<u8>,
        index: i32,
        count: i32,
        writable: bool,
        publicly_visible: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MemoryStream),
                ::core::stringify!(new_6),
            )
        });
        <Self as IMemoryStreamMethods>::ctor_6(
            this,
            buffer,
            index,
            count,
            writable,
            publicly_visible,
        );
        this
    }
}
