
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmapicondata/HubMapIconData.md")))]
#[::unity2::class(namespace = "App", name = "HubMapIconData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubmapicondata :: HubMapIconData >)]
pub struct HubMapIconData {}

#[cfg(feature = "app-hubmapicondata")]
#[::unity2::methods]
impl HubMapIconData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_DisposName", args = 0)]
    pub fn get_dispos_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DisposName", args = 1)]
    pub fn set_dispos_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IconName", args = 0)]
    pub fn get_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IconName", args = 1)]
    pub fn set_icon_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LargeScale", args = 0)]
    pub fn get_large_scale(self) -> f32;

    #[method(name = "set_LargeScale", args = 1)]
    pub fn set_large_scale(self, value: f32) -> ();

    #[method(name = "get_SmallScale", args = 0)]
    pub fn get_small_scale(self) -> f32;

    #[method(name = "set_SmallScale", args = 1)]
    pub fn set_small_scale(self, value: f32) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "TryGetByIconName", args = 1)]
    pub fn try_get_by_icon_name(
        icon_name: ::unity2::Il2CppString,
    ) -> crate::app::hubmapicondata::HubMapIconData;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmapicondata")]
impl HubMapIconData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMapIconData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMapIconDataMethods>::ctor(this);
        this
    }
}
