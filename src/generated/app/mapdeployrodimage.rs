
use crate::app::mapdeploybitimage::IMapDeployBitImage;
use crate::app::mapdeploybitimage::MapDeployBitImage;
use crate::app::mapimagecorebit::IMapImageCoreBit;
use crate::app::mapimagecorebit::MapImageCoreBit;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeployrodimage/MapDeployRodImage.md")))]
#[::unity2::class(namespace = "App", name = "MapDeployRodImage")]
#[parent(crate::app::mapdeploybitimage::MapDeployBitImage)]
pub struct MapDeployRodImage {}

#[cfg(feature = "app-mapdeployrodimage")]
#[::unity2::methods]
impl MapDeployRodImage {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdeployrodimage")]
impl MapDeployRodImage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDeployRodImage),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDeployRodImageMethods>::ctor(this);
        this
    }
}
