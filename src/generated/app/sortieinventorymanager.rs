
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieinventorymanager/SortieInventoryManager_SelectionInfo.md")))]
#[::unity2::class(namespace = "App", name = "SortieInventoryManager.SelectionInfo")]
#[parent(crate::system::object::Object)]
pub struct SortieInventoryManager_SelectionInfo {}

#[cfg(feature = "app-sortieinventorymanager")]
#[::unity2::methods]
impl SortieInventoryManager_SelectionInfo {
    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "IsModeNone", args = 0)]
    pub fn is_mode_none(self) -> bool;

    #[method(name = "IsModeTrade", args = 0)]
    pub fn is_mode_trade(self) -> bool;

    #[method(name = "ResetMode", args = 0)]
    pub fn reset_mode(self) -> ();

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "get_Mode", args = 0)]
    pub fn get_mode(
        self,
    ) -> crate::app::sortieinventorymanager::SortieInventoryManager_SelectionInfo_Modes;

    #[method(name = "set_Mode", args = 1)]
    pub fn set_mode(
        self,
        value: crate::app::sortieinventorymanager::SortieInventoryManager_SelectionInfo_Modes,
    ) -> ();

    #[method(name = "get_IsUnit", args = 0)]
    pub fn get_is_unit(self) -> bool;

    #[method(name = "set_IsUnit", args = 1)]
    pub fn set_is_unit(self, value: bool) -> ();

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_OwnerItemIndex", args = 0)]
    pub fn get_owner_item_index(self) -> i32;

    #[method(name = "set_OwnerItemIndex", args = 1)]
    pub fn set_owner_item_index(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieinventorymanager")]
impl SortieInventoryManager_SelectionInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieInventoryManager_SelectionInfo),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieInventoryManager_SelectionInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieinventorymanager/SortieInventoryManager_ActiveWindow.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieInventoryManager_ActiveWindow {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieInventoryManager_ActiveWindow {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieInventoryManager.ActiveWindow";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieInventoryManager_ActiveWindow {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieInventoryManager_ActiveWindow {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn unit_item() -> Self {
        Self { value: 1 }
    }

    pub fn pool_item() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieinventorymanager/SortieInventoryManager_SelectionInfo_Modes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieInventoryManager_SelectionInfo_Modes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieInventoryManager_SelectionInfo_Modes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieInventoryManager.SelectionInfo.Modes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieInventoryManager_SelectionInfo_Modes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieInventoryManager_SelectionInfo_Modes {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn trade() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieinventorymanager/SortieInventoryManager_Modes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieInventoryManager_Modes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieInventoryManager_Modes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieInventoryManager.Modes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieInventoryManager_Modes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieInventoryManager_Modes {
    pub fn sortie() -> Self {
        Self { value: 0 }
    }

    pub fn transporter() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieinventorymanager/SortieInventoryManager.md")))]
#[::unity2::class(namespace = "App", name = "SortieInventoryManager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: sortieinventorymanager :: SortieInventoryManager >)]
pub struct SortieInventoryManager {
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::sortieinventorymanager::SortieInventoryManager_Modes,
    #[rename(name = "m_Selection")]
    pub m_selection: crate::app::sortieinventorymanager::SortieInventoryManager_SelectionInfo,
    #[rename(name = "m_MenuItemDisplayIndex")]
    pub m_menu_item_display_index: i32,
    #[rename(name = "m_UnitItemMenu")]
    pub m_unit_item_menu: crate::app::inventoryunititemmenu::InventoryUnitItemMenu,
    #[rename(name = "m_PoolItemMenu")]
    pub m_pool_item_menu: crate::app::inventorypoolitemmenu::InventoryPoolItemMenu,
    #[rename(name = "m_ActiveWindow")]
    pub m_active_window: crate::app::sortieinventorymanager::SortieInventoryManager_ActiveWindow,
    #[rename(name = "m_ItemInfo")]
    pub m_item_info: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-sortieinventorymanager")]
#[::unity2::methods]
impl SortieInventoryManager {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "IsModeSortie", args = 0)]
    pub fn is_mode_sortie(self) -> bool;

    #[method(name = "IsModeTransporter", args = 0)]
    pub fn is_mode_transporter(self) -> bool;

    #[method(name = "ResetMode", args = 0)]
    pub fn reset_mode(self) -> ();

    #[method(name = "UpdateUnitMenu", args = 1)]
    pub fn update_unit_menu(self, is_rebuild: bool) -> ();

    #[method(name = "UpdatePoolMenu", args = 1)]
    pub fn update_pool_menu(self, is_auto_select: bool) -> ();

    #[method(name = "SetMenuItemDisplayIndex", args = 1)]
    pub fn set_menu_item_display_index(self, menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "GetMenuItemDisplayIndexFromUnitItemListIndex", args = 2)]
    pub fn get_menu_item_display_index_from_unit_item_list_index(
        self,
        unit: crate::app::unit::Unit,
        unit_item_list_index: i32,
    ) -> i32;

    #[method(name = "SetItemInfo", args = 2)]
    pub fn set_item_info(
        self,
        unit: crate::app::unit::Unit,
        item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "get_Mode", args = 0)]
    pub fn get_mode(self) -> crate::app::sortieinventorymanager::SortieInventoryManager_Modes;

    #[method(name = "set_Mode", args = 1)]
    pub fn set_mode(
        self,
        value: crate::app::sortieinventorymanager::SortieInventoryManager_Modes,
    ) -> ();

    #[method(name = "get_Selection", args = 0)]
    pub fn get_selection(
        self,
    ) -> crate::app::sortieinventorymanager::SortieInventoryManager_SelectionInfo;

    #[method(name = "get_UnitItemMenu", args = 0)]
    pub fn get_unit_item_menu(self) -> crate::app::inventoryunititemmenu::InventoryUnitItemMenu;

    #[method(name = "set_UnitItemMenu", args = 1)]
    pub fn set_unit_item_menu(
        self,
        value: crate::app::inventoryunititemmenu::InventoryUnitItemMenu,
    ) -> ();

    #[method(name = "get_PoolItemMenu", args = 0)]
    pub fn get_pool_item_menu(self) -> crate::app::inventorypoolitemmenu::InventoryPoolItemMenu;

    #[method(name = "set_PoolItemMenu", args = 1)]
    pub fn set_pool_item_menu(
        self,
        value: crate::app::inventorypoolitemmenu::InventoryPoolItemMenu,
    ) -> ();

    #[method(name = "get_Active", args = 0)]
    pub fn get_active(
        self,
    ) -> crate::app::sortieinventorymanager::SortieInventoryManager_ActiveWindow;

    #[method(name = "set_Active", args = 1)]
    pub fn set_active(
        self,
        value: crate::app::sortieinventorymanager::SortieInventoryManager_ActiveWindow,
    ) -> ();

    #[method(name = "get_ItemInfo", args = 0)]
    pub fn get_item_info(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_ItemInfo", args = 1)]
    pub fn set_item_info_2(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_MenuItemDisplayIndex", args = 0)]
    pub fn get_menu_item_display_index(self) -> i32;

    #[method(name = "set_MenuItemDisplayIndex", args = 1)]
    pub fn set_menu_item_display_index_2(self, value: i32) -> ();

    #[method(name = "get_ItemDetailDisplayWithUnit", args = 0)]
    pub fn get_item_detail_display_with_unit(self) -> bool;

    #[method(name = "set_ItemDetailDisplayWithUnit", args = 1)]
    pub fn set_item_detail_display_with_unit(self, value: bool) -> ();
}

#[cfg(feature = "app-sortieinventorymanager")]
impl SortieInventoryManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieInventoryManager),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieInventoryManagerMethods>::ctor(this);
        this
    }
}
