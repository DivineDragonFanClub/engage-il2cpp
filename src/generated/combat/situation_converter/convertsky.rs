
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::combat::situation_converter::converterwithut::ConverterWithUt;
use crate::combat::situation_converter::converterwithut::IConverterWithUt;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertsky/ConvertSky.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConvertSky")]
#[parent(crate::combat::situation_converter::converterwithut::ConverterWithUt)]
pub struct ConvertSky {}

#[cfg(feature = "combat-situation_converter-convertsky")]
#[::unity2::methods]
impl ConvertSky {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "OnDamage", args = 0)]
    pub fn on_damage(self) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-convertsky")]
impl ConvertSky {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConvertSky),
                ::core::stringify!(new),
            )
        });
        <Self as IConvertSkyMethods>::ctor(this, data);
        this
    }
}
