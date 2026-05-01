
use crate::app::mapdeploybitimage::IMapDeployBitImage;
use crate::app::mapdeploybitimage::MapDeployBitImage;
use crate::app::mapimagecorebit::IMapImageCoreBit;
use crate::app::mapimagecorebit::MapImageCoreBit;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeployactionimage/MapDeployActionImage.md")))]
#[::unity2::class(namespace = "App", name = "MapDeployActionImage")]
#[parent(crate::app::mapdeploybitimage::MapDeployBitImage)]
pub struct MapDeployActionImage {}

#[cfg(feature = "app-mapdeployactionimage")]
#[::unity2::methods]
impl MapDeployActionImage {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdeployactionimage")]
impl MapDeployActionImage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDeployActionImage),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDeployActionImageMethods>::ctor(this);
        this
    }
}
