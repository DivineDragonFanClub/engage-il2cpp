
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorypoolitemmenuitem/InventoryPoolItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventoryPoolItemMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct InventoryPoolItemMenuItem {
    #[rename(name = "m_OwnerItemIndex")]
    pub m_owner_item_index: i32,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_SortIndex")]
    pub m_sort_index: i32,
}

#[cfg(feature = "app-inventorypoolitemmenuitem")]
#[::unity2::methods]
impl InventoryPoolItemMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, unit: crate::app::unit::Unit, owner_item_index: i32, sort_index: i32) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "IsEffective", args = 0)]
    pub fn is_effective(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetRecieverUnit", args = 0)]
    pub fn get_reciever_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetSortIndex", args = 0)]
    pub fn get_sort_index(self) -> i32;

    #[method(name = "GetMenuItemKind", args = 0)]
    pub fn get_menu_item_kind(
        self,
    ) -> crate::app::inventorypoolitemmenuitem::InventoryPoolItemMenuItem_PoolItemKind;

    #[method(name = "AddSelection", args = 0)]
    pub fn add_selection(self) -> ();

    #[method(name = "HoldSelection", args = 0)]
    pub fn hold_selection(self) -> ();

    #[method(name = "GoToTrade", args = 0)]
    pub fn go_to_trade(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GoToSubMenu", args = 0)]
    pub fn go_to_sub_menu(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "DirectTrade", args = 0)]
    pub fn direct_trade(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-inventorypoolitemmenuitem")]
impl InventoryPoolItemMenuItem {
    pub fn new(unit: crate::app::unit::Unit, owner_item_index: i32, sort_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventoryPoolItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventoryPoolItemMenuItemMethods>::ctor(this, unit, owner_item_index, sort_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorypoolitemmenuitem/InventoryPoolItemMenuItem_PoolItemKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct InventoryPoolItemMenuItem_PoolItemKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for InventoryPoolItemMenuItem_PoolItemKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "InventoryPoolItemMenuItem.PoolItemKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for InventoryPoolItemMenuItem_PoolItemKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl InventoryPoolItemMenuItem_PoolItemKind {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn none() -> Self {
        Self { value: 1 }
    }

    pub fn blank() -> Self {
        Self { value: 2 }
    }
}
