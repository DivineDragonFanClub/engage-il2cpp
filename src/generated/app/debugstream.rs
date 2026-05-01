
use crate::app::stream_2::IStream_2;
use crate::app::stream_2::Stream_2;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugstream/DebugStream.md")))]
#[::unity2::class(namespace = "App", name = "DebugStream")]
#[parent(crate::app::stream_2::Stream_2)]
pub struct DebugStream {}

#[cfg(feature = "app-debugstream")]
#[::unity2::methods]
impl DebugStream {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, size: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, buffer: ::unity2::Array<u8>) -> ();

    #[method(name = "Save", args = 1)]
    pub fn save(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Load", args = 1)]
    pub fn load(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "WriteMembers", args = 1)]
    pub fn write_members(self, obj: crate::system::object::Object) -> ();

    #[method(name = "ReadMembers", args = 1)]
    pub fn read_members(self, obj: crate::system::object::Object) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete(name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-debugstream")]
impl DebugStream {
    pub fn new(size: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugStream),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugStreamMethods>::ctor(this, size);
        this
    }

    pub fn new_2(buffer: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugStream),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDebugStreamMethods>::ctor_2(this, buffer);
        this
    }
}
