
use crate::app::mapimagecorebit::IMapImageCoreBit;
use crate::app::mapimagecorebit::MapImageCoreBit;
use crate::app::mapimagesightcore::IMapImageSightCore;
use crate::app::mapimagesightcore::MapImageSightCore;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagesight/MapImageSight.md")))]
#[::unity2::class(namespace = "App", name = "MapImageSight")]
#[parent(crate::app::mapimagesightcore::MapImageSightCore)]
pub struct MapImageSight {
    #[rename(name = "m_ForceType")]
    pub m_force_type: crate::app::force::Force_Type,
}

#[cfg(feature = "app-mapimagesight")]
#[::unity2::methods]
impl MapImageSight {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();
}

#[cfg(feature = "app-mapimagesight")]
impl MapImageSight {
    pub fn new(force_type: crate::app::force::Force_Type) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageSight),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageSightMethods>::ctor(this, force_type);
        this
    }
}
