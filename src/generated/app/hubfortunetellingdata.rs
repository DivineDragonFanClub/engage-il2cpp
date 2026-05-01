
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubfortunetellingdata/HubFortuneTellingData.md")))]
#[::unity2::class(namespace = "App", name = "HubFortuneTellingData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubfortunetellingdata :: HubFortuneTellingData >)]
pub struct HubFortuneTellingData {}

#[cfg(feature = "app-hubfortunetellingdata")]
#[::unity2::methods]
impl HubFortuneTellingData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_TextureName", args = 0)]
    pub fn get_texture_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_TextureName", args = 1)]
    pub fn set_texture_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PrimaryText", args = 0)]
    pub fn get_primary_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PrimaryText", args = 1)]
    pub fn set_primary_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PrimaryTextEx", args = 0)]
    pub fn get_primary_text_ex(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PrimaryTextEx", args = 1)]
    pub fn set_primary_text_ex(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ReverseText", args = 0)]
    pub fn get_reverse_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ReverseText", args = 1)]
    pub fn set_reverse_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ReverseTextEx", args = 0)]
    pub fn get_reverse_text_ex(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ReverseTextEx", args = 1)]
    pub fn set_reverse_text_ex(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubfortunetellingdata")]
impl HubFortuneTellingData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubFortuneTellingData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubFortuneTellingDataMethods>::ctor(this);
        this
    }
}
