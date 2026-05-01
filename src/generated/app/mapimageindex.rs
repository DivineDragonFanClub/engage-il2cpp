
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimageindex/MapImageIndex.md")))]
#[::unity2::class(namespace = "App", name = "MapImageIndex")]
#[parent(crate::system::object::Object)]
pub struct MapImageIndex {}

#[cfg(feature = "app-mapimageindex")]
#[::unity2::methods]
impl MapImageIndex {
    #[method(name = "GetIndex", args = 2)]
    pub fn get_index(x: i32, z: i32) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapimageindex")]
impl MapImageIndex {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageIndex),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageIndexMethods>::ctor(this);
        this
    }
}
