
use crate::app::mapimagecorebit::IMapImageCoreBit;
use crate::app::mapimagecorebit::MapImageCoreBit;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagesightcore/MapImageSightCore.md")))]
#[::unity2::class(namespace = "App", name = "MapImageSightCore")]
#[parent(crate::app::mapimagecorebit::MapImageCoreBit)]
pub struct MapImageSightCore {
    #[static_field]
    #[rename(name = "s_SightOcclusion")]
    pub s_sight_occlusion: crate::app::gameparam::GameParam_Holder,
}

#[cfg(feature = "app-mapimagesightcore")]
#[::unity2::methods]
impl MapImageSightCore {
    #[method(name = "Set", args = 3)]
    pub fn set(self, sx: i32, sz: i32, sight: i32) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set_2(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ClearInSight", args = 0)]
    pub fn clear_in_sight(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapimagesightcore")]
impl MapImageSightCore {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageSightCore),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageSightCoreMethods>::ctor(this);
        this
    }
}
