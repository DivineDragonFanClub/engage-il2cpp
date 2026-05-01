
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagecorebit/MapImageCoreBit.md")))]
#[::unity2::class(namespace = "App", name = "MapImageCoreBit")]
#[parent(crate::system::object::Object)]
pub struct MapImageCoreBit {
    #[static_field]
    #[rename(name = "MaxCount")]
    pub max_count: i32,
    #[rename(name = "m_Images")]
    pub m_images: ::unity2::Array<u32>,
    #[static_field]
    #[rename(name = "s_Fill")]
    pub s_fill: ::unity2::Array<u32>,
}

#[cfg(feature = "app-mapimagecorebit")]
#[::unity2::methods]
impl MapImageCoreBit {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, src: crate::app::mapimagecorebit::MapImageCoreBit) -> ();

    #[method(name = "Fill", args = 0)]
    pub fn fill(self) -> ();

    #[method(name = "MergeFrom", args = 1)]
    pub fn merge_from(self, src: crate::app::mapimagecorebit::MapImageCoreBit) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set(self, x: i32, z: i32) -> ();

    #[method(name = "Get", args = 2)]
    pub fn get(self, x: i32, z: i32) -> bool;

    #[method(name = "Clr", args = 2)]
    pub fn clr(self, x: i32, z: i32) -> ();

    #[method(name = "Get", args = 3)]
    pub fn get_2(self, x: i32, z: i32, size: i32) -> bool;

    #[method(name = "Exists", args = 0)]
    pub fn exists(self) -> bool;

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapimagecorebit")]
impl MapImageCoreBit {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageCoreBit),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageCoreBitMethods>::ctor(this);
        this
    }
}
