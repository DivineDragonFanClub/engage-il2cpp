
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/encountweaponcategorydata/EncountWeaponCategoryData.md")))]
#[::unity2::class(namespace = "App", name = "EncountWeaponCategoryData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: encountweaponcategorydata :: EncountWeaponCategoryData >)]
pub struct EncountWeaponCategoryData {}

#[cfg(feature = "app-encountweaponcategorydata")]
#[::unity2::methods]
impl EncountWeaponCategoryData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RankConditionMore", args = 0)]
    pub fn get_rank_condition_more(self) -> u8;

    #[method(name = "set_RankConditionMore", args = 1)]
    pub fn set_rank_condition_more(self, value: u8) -> ();

    #[method(name = "get_RankConditionLess", args = 0)]
    pub fn get_rank_condition_less(self) -> u8;

    #[method(name = "set_RankConditionLess", args = 1)]
    pub fn set_rank_condition_less(self, value: u8) -> ();

    #[method(name = "get_Percentage", args = 0)]
    pub fn get_percentage(self) -> u8;

    #[method(name = "set_Percentage", args = 1)]
    pub fn set_percentage(self, value: u8) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-encountweaponcategorydata")]
impl EncountWeaponCategoryData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EncountWeaponCategoryData),
                ::core::stringify!(new),
            )
        });
        <Self as IEncountWeaponCategoryDataMethods>::ctor(this);
        this
    }
}
