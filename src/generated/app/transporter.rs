
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/transporter/Transporter_Data.md")))]
#[::unity2::class(namespace = "App", name = "Transporter.Data")]
#[parent(crate::system::object::Object)]
pub struct Transporter_Data {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-transporter")]
#[::unity2::methods]
impl Transporter_Data {
    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_UnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-transporter")]
impl Transporter_Data {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Transporter_Data),
                ::core::stringify!(new),
            )
        });
        <Self as ITransporter_DataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/transporter/Transporter.md")))]
#[::unity2::class(namespace = "App", name = "Transporter")]
#[parent(crate::system::object::Object)]
pub struct Transporter {
    #[static_field]
    #[rename(name = "MaxOld")]
    pub max_old: i32,
    #[static_field]
    #[rename(name = "MaxData")]
    pub max_data: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "s_Data")]
    pub s_data: ::unity2::Array<crate::app::transporter::Transporter_Data>,
}

#[cfg(feature = "app-transporter")]
#[::unity2::methods]
impl Transporter {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset() -> ();

    #[method(name = "IsAvailable", args = 0)]
    pub fn is_available() -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get(index: i32) -> crate::app::transporter::Transporter_Data;

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index(unit_item: crate::app::unititem::UnitItem) -> i32;

    #[method(name = "CanAdd", args = 0)]
    pub fn can_add() -> bool;

    #[method(name = "TryGetLowestItemIndex", args = 2)]
    pub fn try_get_lowest_item_index(lowest_index: i32, lowest_worth: u64) -> bool;

    #[method(name = "GetEmptyIndex", args = 0)]
    pub fn get_empty_index() -> i32;

    #[method(name = "Discard", args = 1)]
    pub fn discard(unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(unit_item: crate::app::unititem::UnitItem) -> i32;

    #[method(name = "Add", args = 1)]
    pub fn add_2(item_data: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "Sub", args = 1)]
    pub fn sub(index: i32) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete(index: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "GetCount", args = 0)]
    pub fn get_count() -> i32;

    #[method(name = "DeleteItem", args = 1)]
    pub fn delete_item(data: crate::app::itemdata::ItemData) -> ();

    #[method(name = "GetItemCount", args = 1)]
    pub fn get_item_count(data: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "GetTypeItemCount", args = 1)]
    pub fn get_type_item_count(r#type: crate::app::itemdata::ItemData_UseTypes) -> i32;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "IsRare", args = 1)]
    pub fn is_rare(unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "CalcWorth", args = 1)]
    pub fn calc_worth(unit_item: crate::app::unititem::UnitItem) -> u64;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-transporter")]
impl Transporter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Transporter),
                ::core::stringify!(new),
            )
        });
        <Self as ITransporterMethods>::ctor(this);
        this
    }
}
