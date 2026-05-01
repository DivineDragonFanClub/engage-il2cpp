
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::combat::situation_converter::converterwithut::ConverterWithUt;
use crate::combat::situation_converter::converterwithut::IConverterWithUt;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertcrosscut/ConvertCrossCut.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConvertCrossCut")]
#[parent(crate::combat::situation_converter::converterwithut::ConverterWithUt)]
pub struct ConvertCrossCut {
    #[rename(name = "m_IsEnded")]
    pub m_is_ended: bool,
}

#[cfg(feature = "combat-situation_converter-convertcrosscut")]
#[::unity2::methods]
impl ConvertCrossCut {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        situation: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnBegin", args = 0)]
    pub fn on_begin(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnApproach", args = 0)]
    pub fn on_approach(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnAttack", args = 0)]
    pub fn on_attack(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnDamage", args = 0)]
    pub fn on_damage(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnEnd", args = 0)]
    pub fn on_end(self) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-convertcrosscut")]
impl ConvertCrossCut {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConvertCrossCut),
                ::core::stringify!(new),
            )
        });
        <Self as IConvertCrossCutMethods>::ctor(this, data);
        this
    }
}
