
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertchasearrow/ConvertChaseArrow.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConvertChaseArrow")]
#[parent(crate::combat::situation_converter::baseconverter::BaseConverter)]
pub struct ConvertChaseArrow {
    #[rename(name = "m_ShootCount")]
    pub m_shoot_count: i32,
}

#[cfg(feature = "combat-situation_converter-convertchasearrow")]
#[::unity2::methods]
impl ConvertChaseArrow {
    #[method(name = "get_IsSecondAttack", args = 0)]
    pub fn get_is_second_attack(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "get_DoChase", args = 0)]
    pub fn get_do_chase(self) -> bool;

    #[method(name = "set_DoChase", args = 1)]
    pub fn set_do_chase(self, value: bool) -> ();

    #[method(name = "get_IsEnd", args = 0)]
    pub fn get_is_end(self) -> bool;

    #[method(name = "set_IsEnd", args = 1)]
    pub fn set_is_end(self, value: bool) -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        situation: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-convertchasearrow")]
impl ConvertChaseArrow {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConvertChaseArrow),
                ::core::stringify!(new),
            )
        });
        <Self as IConvertChaseArrowMethods>::ctor(this, data);
        this
    }
}
