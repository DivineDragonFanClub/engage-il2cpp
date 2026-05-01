
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititemlist/UnitItemList.md")))]
#[::unity2::class(namespace = "App", name = "UnitItemList")]
#[parent(crate::system::object::Object)]
pub struct UnitItemList {
    #[static_field]
    #[rename(name = "EngageItemMax")]
    pub engage_item_max: i32,
    #[static_field]
    #[rename(name = "ItemMax")]
    pub item_max: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_UnitItems")]
    pub m_unit_items: ::unity2::Array<crate::app::unititem::UnitItem>,
}

#[cfg(feature = "app-unititemlist")]
#[::unity2::methods]
impl UnitItemList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, from: crate::app::unititemlist::UnitItemList) -> ();

    #[method(name = "GetEmptyIndex", args = 0)]
    pub fn get_empty_index(self) -> i32;

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "Add", args = 1)]
    pub fn add_2(self, unit_item: crate::app::unititem::UnitItem) -> i32;

    #[method(name = "Move", args = 2)]
    pub fn r#move(self, from: i32, to: i32) -> ();

    #[method(name = "CloseUp", args = 0)]
    pub fn close_up(self) -> ();

    #[method(name = "Equip", args = 1)]
    pub fn equip(self, index: i32) -> ();

    #[method(name = "TakeOff", args = 1)]
    pub fn take_off(self, index: i32) -> bool;

    #[method(name = "PutOff", args = 2)]
    pub fn put_off(self, index: i32, closeup: bool) -> bool;

    #[method(name = "PutOffAll", args = 0)]
    pub fn put_off_all(self) -> ();

    #[method(name = "IsEquipped", args = 0)]
    pub fn is_equipped(self) -> bool;

    #[method(name = "GetEquipped", args = 0)]
    pub fn get_equipped(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetEquipped", args = 1)]
    pub fn get_equipped_2(self, index: i32) -> crate::app::unititem::UnitItem;

    #[method(name = "GetIndexEquipped", args = 0)]
    pub fn get_index_equipped(self) -> i32;

    #[method(name = "GetIndexEnchantable", args = 1)]
    pub fn get_index_enchantable(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "HasEnchanted", args = 0)]
    pub fn has_enchanted(self) -> bool;

    #[method(name = "GetHold", args = 1)]
    pub fn get_hold(self, unit: crate::app::unit::Unit) -> crate::app::unititem::UnitItem;

    #[method(name = "GetIndexHold", args = 1)]
    pub fn get_index_hold(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "HasRod", args = 2)]
    pub fn has_rod(self, unit: crate::app::unit::Unit, can_use_check: bool) -> bool;

    #[method(name = "HasHealRod", args = 2)]
    pub fn has_heal_rod(self, unit: crate::app::unit::Unit, can_use_check: bool) -> bool;

    #[method(name = "HasHealRodForOneself", args = 2)]
    pub fn has_heal_rod_for_oneself(
        self,
        unit: crate::app::unit::Unit,
        can_use_check: bool,
    ) -> bool;

    #[method(name = "HasSupportRod", args = 3)]
    pub fn has_support_rod(
        self,
        unit: crate::app::unit::Unit,
        can_use_check: bool,
        is_oneself: bool,
    ) -> bool;

    #[method(name = "HasInterferenceRod", args = 2)]
    pub fn has_interference_rod(self, unit: crate::app::unit::Unit, can_us_check: bool) -> bool;

    #[method(name = "HasCriticalWeapon", args = 2)]
    pub fn has_critical_weapon(self, unit: crate::app::unit::Unit, can_use_check: bool) -> bool;

    #[method(name = "HasEfficacyWeapon", args = 3)]
    pub fn has_efficacy_weapon(
        self,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        can_use_check: bool,
    ) -> bool;

    #[method(name = "HasDropItem", args = 0)]
    pub fn has_drop_item(self) -> bool;

    #[method(name = "HasItem", args = 1)]
    pub fn has_item(self, item: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "FindItem", args = 1)]
    pub fn find_item(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "PutEngageItem", args = 2)]
    pub fn put_engage_item(self, god_unit: crate::app::godunit::GodUnit, engaged: bool) -> ();

    #[method(name = "GetItemCount", args = 1)]
    pub fn get_item_count(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::app::unititem::UnitItem;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;
}

#[cfg(feature = "app-unititemlist")]
impl UnitItemList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitItemList),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitItemListMethods>::ctor(this);
        this
    }
}
