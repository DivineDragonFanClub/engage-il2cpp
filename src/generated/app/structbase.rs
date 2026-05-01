
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structbase/StructBase.md")))]
#[::unity2::class(namespace = "App", name = "StructBase")]
#[parent(crate::system::object::Object)]
pub struct StructBase {
    #[rename(name = "Index")]
    pub index: i32,
    #[rename(name = "Hash")]
    pub hash: i32,
    #[rename(name = "Key")]
    pub key: ::unity2::Il2CppString,
}

#[cfg(feature = "app-structbase")]
#[::unity2::methods]
impl StructBase {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = "OnCompletedEnd", args = 0)]
    pub fn on_completed_end(self) -> ();

    #[method(name = "OnRelease", args = 0)]
    pub fn on_release(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetIndex", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-structbase")]
impl StructBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructBase),
                ::core::stringify!(new),
            )
        });
        <Self as IStructBaseMethods>::ctor(this);
        this
    }
}
