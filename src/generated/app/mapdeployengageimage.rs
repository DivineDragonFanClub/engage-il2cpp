
use crate::app::mapdeploybitimage::IMapDeployBitImage;
use crate::app::mapdeploybitimage::MapDeployBitImage;
use crate::app::mapimagecorebit::IMapImageCoreBit;
use crate::app::mapimagecorebit::MapImageCoreBit;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeployengageimage/MapDeployEngageImage.md")))]
#[::unity2::class(namespace = "App", name = "MapDeployEngageImage")]
#[parent(crate::app::mapdeploybitimage::MapDeployBitImage)]
pub struct MapDeployEngageImage {}

#[cfg(feature = "app-mapdeployengageimage")]
#[::unity2::methods]
impl MapDeployEngageImage {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdeployengageimage")]
impl MapDeployEngageImage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDeployEngageImage),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDeployEngageImageMethods>::ctor(this);
        this
    }
}
