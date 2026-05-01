
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideprizedata/DragonRidePrizeData.md")))]
#[::unity2::class(namespace = "App", name = "DragonRidePrizeData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: dragonrideprizedata :: DragonRidePrizeData >)]
pub struct DragonRidePrizeData {}

#[cfg(feature = "app-dragonrideprizedata")]
#[::unity2::methods]
impl DragonRidePrizeData {
    #[method(name = "get_ItemList", args = 0)]
    pub fn get_item_list(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_ItemList", args = 1)]
    pub fn set_item_list(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Group", args = 0)]
    pub fn get_group(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Group", args = 1)]
    pub fn set_group(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PieceOfBond", args = 0)]
    pub fn get_piece_of_bond(self) -> i32;

    #[method(name = "set_PieceOfBond", args = 1)]
    pub fn set_piece_of_bond(self, value: i32) -> ();

    #[method(name = "get_ItemCount", args = 0)]
    pub fn get_item_count(self) -> i32;

    #[method(name = "set_ItemCount", args = 1)]
    pub fn set_item_count(self, value: i32) -> ();

    #[method(name = "get_Item1", args = 0)]
    pub fn get_item1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Item1", args = 1)]
    pub fn set_item1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Item2", args = 0)]
    pub fn get_item2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Item2", args = 1)]
    pub fn set_item2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Item3", args = 0)]
    pub fn get_item3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Item3", args = 1)]
    pub fn set_item3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Item4", args = 0)]
    pub fn get_item4(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Item4", args = 1)]
    pub fn set_item4(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Item5", args = 0)]
    pub fn get_item5(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Item5", args = 1)]
    pub fn set_item5(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Item6", args = 0)]
    pub fn get_item6(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Item6", args = 1)]
    pub fn set_item6(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ItemTypeCount", args = 0)]
    pub fn get_item_type_count(self) -> i32;

    #[method(name = "set_ItemTypeCount", args = 1)]
    pub fn set_item_type_count(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();
}

#[cfg(feature = "app-dragonrideprizedata")]
impl DragonRidePrizeData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRidePrizeData),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRidePrizeDataMethods>::ctor(this);
        this
    }
}
