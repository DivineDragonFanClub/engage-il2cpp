
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/encountraremonaydata/EncountRareMonayData.md")))]
#[::unity2::class(namespace = "App", name = "EncountRareMonayData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: encountraremonaydata :: EncountRareMonayData >)]
pub struct EncountRareMonayData {}

#[cfg(feature = "app-encountraremonaydata")]
#[::unity2::methods]
impl EncountRareMonayData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_NationLevel", args = 0)]
    pub fn get_nation_level(self) -> u16;

    #[method(name = "set_NationLevel", args = 1)]
    pub fn set_nation_level(self, value: u16) -> ();

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetIid", args = 2)]
    pub fn get_iid_2(
        difficulty: crate::app::difficulty::Difficulty,
        nation_level: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetDifficultyText", args = 1)]
    pub fn get_difficulty_text(
        difficulty: crate::app::difficulty::Difficulty,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-encountraremonaydata")]
impl EncountRareMonayData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EncountRareMonayData),
                ::core::stringify!(new),
            )
        });
        <Self as IEncountRareMonayDataMethods>::ctor(this);
        this
    }
}
