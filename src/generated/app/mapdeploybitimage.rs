
use crate::app::mapimagecorebit::IMapImageCoreBit;
use crate::app::mapimagecorebit::MapImageCoreBit;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploybitimage/MapDeployBitImage.md")))]
#[::unity2::class(namespace = "App", name = "MapDeployBitImage")]
#[parent(crate::app::mapimagecorebit::MapImageCoreBit)]
pub struct MapDeployBitImage {
    #[rename(name = "m_Changed")]
    pub m_changed: bool,
}

#[cfg(feature = "app-mapdeploybitimage")]
#[::unity2::methods]
impl MapDeployBitImage {
    #[method(name = "get_Display", args = 0)]
    pub fn get_display(
        self,
    ) -> crate::app::mapdeploytemplate_1::MapDeployTemplate_1_DisplayType<
        crate::app::mapdeploy::MapDeploy,
    >;

    #[method(name = "set_Display", args = 1)]
    pub fn set_display(
        self,
        value: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_DisplayType<
            crate::app::mapdeploy::MapDeploy,
        >,
    ) -> ();

    #[method(name = "TrySet", args = 2)]
    pub fn try_set(self, x: i32, z: i32) -> ();

    #[method(name = "TryClear", args = 0)]
    pub fn try_clear(self) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdeploybitimage")]
impl MapDeployBitImage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDeployBitImage),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDeployBitImageMethods>::ctor(this);
        this
    }
}
