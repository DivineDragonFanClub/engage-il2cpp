
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/memory/Memory.md")))]
#[::unity2::class(namespace = "App", name = "Memory")]
#[parent(crate::system::object::Object)]
pub struct Memory {
    #[static_field]
    #[rename(name = "Max")]
    pub max: i32,
    #[static_field]
    #[rename(name = "s_Fill")]
    pub s_fill: ::unity2::Array<u8>,
}

#[cfg(feature = "app-memory")]
#[::unity2::methods]
impl Memory {
    #[method(name = "Initialized", args = 0)]
    pub fn initialized() -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(values: ::unity2::Array<u8>) -> ();

    #[method(name = "Clear", args = 3)]
    pub fn clear_2(values: ::unity2::Array<u8>, index: i32, count: i32) -> ();

    #[method(name = "Fill", args = 1)]
    pub fn fill(values: ::unity2::Array<u8>) -> ();

    #[method(name = "Fill", args = 3)]
    pub fn fill_2(values: ::unity2::Array<u8>, index: i32, count: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-memory")]
impl Memory {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Memory),
                ::core::stringify!(new),
            )
        });
        <Self as IMemoryMethods>::ctor(this);
        this
    }
}
