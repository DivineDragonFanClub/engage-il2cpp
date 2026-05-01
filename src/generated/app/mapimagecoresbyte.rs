
use crate::app::mapimagecore_1::IMapImageCore_1;
use crate::app::mapimagecore_1::MapImageCore_1;
use crate::app::mapimageindex::IMapImageIndex;
use crate::app::mapimageindex::MapImageIndex;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagecoresbyte/MapImageCoreSbyte.md")))]
#[::unity2::class(namespace = "App", name = "MapImageCoreSbyte")]
# [parent (crate :: app :: mapimagecore_1 :: MapImageCore_1 < i8 >)]
pub struct MapImageCoreSbyte {}

#[cfg(feature = "app-mapimagecoresbyte")]
#[::unity2::methods]
impl MapImageCoreSbyte {
    #[method(name = "Add", args = 2)]
    pub fn add(self, index: i32, v: i8) -> ();

    #[method(name = "GetMin", args = 0)]
    pub fn get_min(self) -> i32;

    #[method(name = "GetMax", args = 0)]
    pub fn get_max(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapimagecoresbyte")]
impl MapImageCoreSbyte {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageCoreSbyte),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageCoreSbyteMethods>::ctor(this);
        this
    }
}
