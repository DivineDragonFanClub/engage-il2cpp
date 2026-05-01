
use crate::app::debugmenu::DebugMenu;
use crate::app::debugmenu::IDebugMenu;
use crate::app::labelitem::ILabelItem;
use crate::app::labelitem::LabelItem;
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::stringitem::IStringItem;
use crate::app::stringitem::StringItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_RingItemBase.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.RingItemBase")]
#[parent(crate::app::debugunitringmenu::DebugUnitRingMenu_UnitMenuItem)]
pub struct DebugUnitRingMenu_RingItemBase {}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_RingItemBase {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnWidth2", args = 0)]
    pub fn get_column_width2(self) -> f32;
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_RingItemBase {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_RingItemBase),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_RingItemBaseMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_EmblemRingItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.EmblemRingItem")]
#[parent(crate::app::debugunitringmenu::DebugUnitRingMenu_RingItemBase)]
pub struct DebugUnitRingMenu_EmblemRingItem {
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_EmblemRingItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_EmblemRingItem {
    pub fn new(unit: crate::app::unit::Unit, god_unit: crate::app::godunit::GodUnit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_EmblemRingItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_EmblemRingItemMethods>::ctor(this, unit, god_unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_EmblemRingLabelItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.EmblemRingLabelItem")]
#[parent(crate::app::debugunitringmenu::DebugUnitRingMenu_RingLabelItemBase)]
pub struct DebugUnitRingMenu_EmblemRingLabelItem {}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_EmblemRingLabelItem {
    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_EmblemRingLabelItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_EmblemRingLabelItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_EmblemRingLabelItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_ClearRingItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.ClearRingItem")]
#[parent(crate::app::debugunitringmenu::DebugUnitRingMenu_UnitMenuItem)]
pub struct DebugUnitRingMenu_ClearRingItem {}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_ClearRingItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_ClearRingItem {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_ClearRingItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_ClearRingItemMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_UnitMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.UnitMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugUnitRingMenu_UnitMenuItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_UnitMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_UnitMenuItem {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_UnitMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_UnitMenuItemMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_CommonRingItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.CommonRingItem")]
#[parent(crate::app::debugunitringmenu::DebugUnitRingMenu_RingItemBase)]
pub struct DebugUnitRingMenu_CommonRingItem {
    #[rename(name = "m_Ring")]
    pub m_ring: crate::app::unitring::UnitRing,
}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_CommonRingItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, ring: crate::app::unitring::UnitRing) -> ();

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_CommonRingItem {
    pub fn new(unit: crate::app::unit::Unit, ring: crate::app::unitring::UnitRing) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_CommonRingItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_CommonRingItemMethods>::ctor(this, unit, ring);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_RingLabelItemBase.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.RingLabelItemBase")]
#[parent(crate::app::labelitem::LabelItem)]
pub struct DebugUnitRingMenu_RingLabelItemBase {}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_RingLabelItemBase {
    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnWidth2", args = 0)]
    pub fn get_column_width2(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_RingLabelItemBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_RingLabelItemBase),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_RingLabelItemBaseMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_CurrentRingLabelItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.CurrentRingLabelItem")]
#[parent(crate::app::labelitem::LabelItem)]
pub struct DebugUnitRingMenu_CurrentRingLabelItem {}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_CurrentRingLabelItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_CurrentRingLabelItem {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_CurrentRingLabelItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_CurrentRingLabelItemMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugUnitRingMenu {
    #[static_field]
    #[rename(name = "ColumnWidth0")]
    pub column_width0: f32,
    #[static_field]
    #[rename(name = "ColumnWidth1")]
    pub column_width1: f32,
    #[static_field]
    #[rename(name = "ColumnWidth2")]
    pub column_width2: f32,
}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu {
    #[method(name = "CanCreate", args = 1)]
    pub fn can_create(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_UnitRingMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.UnitRingMenu")]
#[parent(crate::app::debugmenu::DebugMenu)]
pub struct DebugUnitRingMenu_UnitRingMenu {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_UnitRingMenu {
    #[method(name = "Build", args = 1)]
    pub fn build(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OnRebuild", args = 0)]
    pub fn on_rebuild(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_UnitRingMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_UnitRingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_UnitRingMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunitringmenu/DebugUnitRingMenu_CommonRingLabelItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitRingMenu.CommonRingLabelItem")]
#[parent(crate::app::debugunitringmenu::DebugUnitRingMenu_RingLabelItemBase)]
pub struct DebugUnitRingMenu_CommonRingLabelItem {}

#[cfg(feature = "app-debugunitringmenu")]
#[::unity2::methods]
impl DebugUnitRingMenu_CommonRingLabelItem {
    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunitringmenu")]
impl DebugUnitRingMenu_CommonRingLabelItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitRingMenu_CommonRingLabelItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitRingMenu_CommonRingLabelItemMethods>::ctor(this);
        this
    }
}
