
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chartdata/ChartData.md")))]
#[::unity2::class(namespace = "App", name = "ChartData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: chartdata :: ChartData >)]
pub struct ChartData {
    #[static_field]
    #[rename(name = "ITEM_COUNT")]
    pub item_count: i32,
}

#[cfg(feature = "app-chartdata")]
#[::unity2::methods]
impl ChartData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Chapter", args = 0)]
    pub fn get_chapter(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Chapter", args = 1)]
    pub fn set_chapter(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LevelN", args = 0)]
    pub fn get_level_n(self) -> u8;

    #[method(name = "set_LevelN", args = 1)]
    pub fn set_level_n(self, value: u8) -> ();

    #[method(name = "get_LevelH", args = 0)]
    pub fn get_level_h(self) -> u8;

    #[method(name = "set_LevelH", args = 1)]
    pub fn set_level_h(self, value: u8) -> ();

    #[method(name = "get_LevelL", args = 0)]
    pub fn get_level_l(self) -> u8;

    #[method(name = "set_LevelL", args = 1)]
    pub fn set_level_l(self, value: u8) -> ();

    #[method(name = "get_Jid", args = 0)]
    pub fn get_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Jid", args = 1)]
    pub fn set_jid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Items", args = 0)]
    pub fn get_items(self) -> ::unity2::Array<crate::app::chartdata::ChartData_Item>;

    #[method(name = "set_Items", args = 1)]
    pub fn set_items(self, value: ::unity2::Array<crate::app::chartdata::ChartData_Item>) -> ();

    #[method(name = "get_Item1", args = 0)]
    pub fn get_item1(self) -> crate::app::chartdata::ChartData_Item;

    #[method(name = "set_Item1", args = 1)]
    pub fn set_item1(self, value: crate::app::chartdata::ChartData_Item) -> ();

    #[method(name = "get_Item2", args = 0)]
    pub fn get_item2(self) -> crate::app::chartdata::ChartData_Item;

    #[method(name = "set_Item2", args = 1)]
    pub fn set_item2(self, value: crate::app::chartdata::ChartData_Item) -> ();

    #[method(name = "get_Item3", args = 0)]
    pub fn get_item3(self) -> crate::app::chartdata::ChartData_Item;

    #[method(name = "set_Item3", args = 1)]
    pub fn set_item3(self, value: crate::app::chartdata::ChartData_Item) -> ();

    #[method(name = "get_Item4", args = 0)]
    pub fn get_item4(self) -> crate::app::chartdata::ChartData_Item;

    #[method(name = "set_Item4", args = 1)]
    pub fn set_item4(self, value: crate::app::chartdata::ChartData_Item) -> ();

    #[method(name = "get_Item5", args = 0)]
    pub fn get_item5(self) -> crate::app::chartdata::ChartData_Item;

    #[method(name = "set_Item5", args = 1)]
    pub fn set_item5(self, value: crate::app::chartdata::ChartData_Item) -> ();

    #[method(name = "get_GodId", args = 0)]
    pub fn get_god_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_GodId", args = 1)]
    pub fn set_god_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetLevel", args = 1)]
    pub fn get_level(self, difficulty: crate::app::difficulty::Difficulty) -> i32;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-chartdata")]
impl ChartData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChartData),
                ::core::stringify!(new),
            )
        });
        <Self as IChartDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chartdata/ChartData_Item.md")))]
#[::unity2::class(namespace = "App", name = "ChartData.Item")]
#[parent(crate::system::object::Object)]
pub struct ChartData_Item {}

#[cfg(feature = "app-chartdata")]
#[::unity2::methods]
impl ChartData_Item {
    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-chartdata")]
impl ChartData_Item {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChartData_Item),
                ::core::stringify!(new),
            )
        });
        <Self as IChartData_ItemMethods>::ctor(this);
        this
    }
}
