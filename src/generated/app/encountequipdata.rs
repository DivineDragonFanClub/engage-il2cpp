
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/encountequipdata/EncountEquipData.md")))]
#[::unity2::class(namespace = "App", name = "EncountEquipData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: encountequipdata :: EncountEquipData >)]
pub struct EncountEquipData {
    #[static_field]
    #[rename(name = "RankNone")]
    pub rank_none: i32,
    #[rename(name = "m_LowerRank")]
    pub m_lower_rank: i32,
    #[rename(name = "m_UpperRank")]
    pub m_upper_rank: i32,
}

#[cfg(feature = "app-encountequipdata")]
#[::unity2::methods]
impl EncountEquipData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Category", args = 0)]
    pub fn get_category(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Category", args = 1)]
    pub fn set_category(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Percentage", args = 0)]
    pub fn get_percentage(self) -> u8;

    #[method(name = "set_Percentage", args = 1)]
    pub fn set_percentage(self, value: u8) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetKindCount", args = 1)]
    pub fn get_kind_count(self, kind: crate::app::itemdata::ItemData_Kinds) -> i32;

    #[method(name = "GetKindCount", args = 1)]
    pub fn get_kind_count_2(
        self,
        kinds: ::unity2::Array<crate::app::itemdata::ItemData_Kinds>,
    ) -> i32;

    #[method(name = "IsContains", args = 1)]
    pub fn is_contains(self, rank: i32) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-encountequipdata")]
impl EncountEquipData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EncountEquipData),
                ::core::stringify!(new),
            )
        });
        <Self as IEncountEquipDataMethods>::ctor(this);
        this
    }
}
