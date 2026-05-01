
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitaccessorylist/UnitAccessoryList.md")))]
#[::unity2::class(namespace = "App", name = "UnitAccessoryList")]
#[parent(crate::system::object::Object)]
pub struct UnitAccessoryList {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_UnitAccessorys")]
    pub m_unit_accessorys: ::unity2::Array<crate::app::unitaccessory::UnitAccessory>,
}

#[cfg(feature = "app-unitaccessorylist")]
#[::unity2::methods]
impl UnitAccessoryList {
    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> crate::app::unitaccessory::UnitAccessory;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item_2(
        self,
        kind: crate::app::accessorydata::AccessoryData_Kinds,
    ) -> crate::app::unitaccessory::UnitAccessory;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist(self, accessory: crate::app::accessorydata::AccessoryData) -> bool;

    #[method(name = "Clear", args = 1)]
    pub fn clear_2(self, index: i32) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear_3(self, kind: crate::app::accessorydata::AccessoryData_Kinds) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        accessory: crate::app::accessorydata::AccessoryData,
        kind: crate::app::accessorydata::AccessoryData_Kinds,
    ) -> bool;

    #[method(name = "Add", args = 2)]
    pub fn add_2(self, accessory: crate::app::accessorydata::AccessoryData, index: i32) -> bool;

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, list: crate::app::unitaccessorylist::UnitAccessoryList) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg(feature = "app-unitaccessorylist")]
impl UnitAccessoryList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitAccessoryList),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitAccessoryListMethods>::ctor(this);
        this
    }
}
