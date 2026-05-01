
use crate::app::mapimageindex::IMapImageIndex;
use crate::app::mapimageindex::MapImageIndex;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagecore_1/MapImageCore_1.md")))]
#[::unity2::class(namespace = "App", name = "MapImageCore`1")]
pub struct MapImageCore_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "ImageSize")]
    pub image_size: i32,
    #[rename(name = "m_Images")]
    pub m_images: ::unity2::Array<T0>,
}

#[cfg(feature = "app-mapimagecore_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MapImageCore_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, v: T0) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, src: crate::app::mapimagecore_1::MapImageCore_1<T0>) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set(self, index: i32, v: T0) -> ();

    #[method(name = "Set", args = 3)]
    pub fn set_2(self, x: i32, z: i32, v: T0) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, index: i32) -> T0;

    #[method(name = "Get", args = 2)]
    pub fn get_2(self, x: i32, z: i32) -> T0;

    #[method(name = "Add", args = 2)]
    pub fn add(self, index: i32, v: T0) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_2(self, x: i32, z: i32, v: T0) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "get_Images", args = 0)]
    pub fn get_images(self) -> ::unity2::Array<T0>;
}

#[cfg(feature = "app-mapimagecore_1")]
impl<T0: ::unity2::ClassIdentity> MapImageCore_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageCore_1),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageCore_1Methods<T0>>::ctor(this);
        this
    }
}
