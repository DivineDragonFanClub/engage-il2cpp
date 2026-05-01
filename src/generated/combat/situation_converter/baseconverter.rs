
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/baseconverter/BaseConverter.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "BaseConverter")]
#[parent(crate::system::object::Object)]
pub struct BaseConverter {}

#[cfg(feature = "combat-situation_converter-baseconverter")]
#[::unity2::methods]
impl BaseConverter {
    #[method(name = "get_DataSet", args = 0)]
    pub fn get_data_set(self) -> crate::combat::situation_converter::cameradataset::CameraDataSet;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        style: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "get_IsFirstAttack", args = 0)]
    pub fn get_is_first_attack(self) -> bool;

    #[method(name = "get_IsPlayerAttack", args = 0)]
    pub fn get_is_player_attack(self) -> bool;

    #[method(name = "NameToPosition", args = 1)]
    pub fn name_to_position(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-baseconverter")]
impl BaseConverter {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseConverter),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseConverterMethods>::ctor(this, data);
        this
    }
}
