
use crate::app::debugmenu::DebugMenu;
use crate::app::debugmenu::IDebugMenu;
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_RefineMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.RefineMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_RefineMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_RefineMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_RefineMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_RefineMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_RefineMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_RefineMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.RefineMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugUnitItemMenu_RefineMenu {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_RefineMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_RefineMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_RefineMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_RefineMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_NameMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.NameMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_NameMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_NameMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_NameMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_NameMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_NameMenuItemMethods>::ctor(this, unit, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_BaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.BaseMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugUnitItemMenu_SubMenu_BaseMenuItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_BaseMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetWidth", args = 0)]
    pub fn get_width(self) -> f32;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_BaseMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_BaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_BaseMenuItemMethods>::ctor(this, unit, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_RefineMenu_EvolveMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DebugUnitItemMenu.RefineMenu.EvolveMenuItem"
)]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_RefineMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_RefineMenu_EvolveMenuItem {
    #[rename(name = "m_DataIndex")]
    pub m_data_index: i32,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::itemevolvedata::ItemEvolveData,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_RefineMenu_EvolveMenuItem {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        data_index: i32,
        evolve_data: crate::app::itemevolvedata::ItemEvolveData,
    ) -> ();

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_RefineMenu_EvolveMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        data_index: i32,
        evolve_data: crate::app::itemevolvedata::ItemEvolveData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_RefineMenu_EvolveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_RefineMenu_EvolveMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
            data_index,
            evolve_data,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_EnchantMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.EnchantMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_FlagMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_EnchantMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_EnchantMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "SetFlag", args = 1)]
    pub fn set_flag(self, enable: bool) -> ();

    #[method(name = "IsFlag", args = 0)]
    pub fn is_flag(self) -> bool;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_EnchantMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_EnchantMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_EnchantMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_RefineMenu_RefineMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DebugUnitItemMenu.RefineMenu.RefineMenuItem"
)]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_RefineMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_RefineMenu_RefineMenuItem {
    #[rename(name = "m_Level")]
    pub m_level: i32,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::itemrefinedata::ItemRefineData,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_RefineMenu_RefineMenuItem {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        refine_level: i32,
        refine_data: crate::app::itemrefinedata::ItemRefineData,
    ) -> ();

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "GetParameterText", args = 1)]
    pub fn get_parameter_text(self, value: i32) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_RefineMenu_RefineMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        refine_level: i32,
        refine_data: crate::app::itemrefinedata::ItemRefineData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_RefineMenu_RefineMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_RefineMenu_RefineMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
            refine_level,
            refine_data,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_ItemListMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.ItemListMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugUnitItemMenu_ItemListMenu {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_ItemListMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_ItemListMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_ItemListMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_ItemListMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_EngraveMenu_BaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.EngraveMenu.BaseMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugUnitItemMenu_EngraveMenu_BaseMenuItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_EngraveMenu_BaseMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_EngraveMenu_BaseMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_EngraveMenu_BaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_EngraveMenu_BaseMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_EditItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.EditItemMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugUnitItemMenu_EditItemMenuItem {
    #[static_field]
    #[rename(name = "MinIndex")]
    pub min_index: i32,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_EditItemMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetFontColor", args = 0)]
    pub fn get_font_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPrevIndex", args = 2)]
    pub fn get_prev_index(self, index: i32, is_trigger: bool) -> i32;

    #[method(name = "GetNextIndex", args = 2)]
    pub fn get_next_index(self, index: i32, is_trigger: bool) -> i32;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_EditItemMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_EditItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_EditItemMenuItemMethods>::ctor(this, unit, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_EngraveMenu_EngraveMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DebugUnitItemMenu.EngraveMenu.EngraveMenuItem"
)]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_EngraveMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_EngraveMenu_EngraveMenuItem {
    #[rename(name = "m_GodData")]
    pub m_god_data: crate::app::goddata::GodData,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_EngraveMenu_EngraveMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        god_data: crate::app::goddata::GodData,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_EngraveMenu_EngraveMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        god_data: crate::app::goddata::GodData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_EngraveMenu_EngraveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_EngraveMenu_EngraveMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
            god_data,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_DropMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.DropMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_FlagMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_DropMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_DropMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "SetFlag", args = 1)]
    pub fn set_flag(self, enable: bool) -> ();

    #[method(name = "IsFlag", args = 0)]
    pub fn is_flag(self) -> bool;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_DropMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_DropMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_DropMenuItemMethods>::ctor(this, unit, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_EquipMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.EquipMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_FlagMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_EquipMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_EquipMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "SetFlag", args = 1)]
    pub fn set_flag(self, enable: bool) -> ();

    #[method(name = "IsFlag", args = 0)]
    pub fn is_flag(self) -> bool;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_EquipMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_EquipMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_EquipMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_RefineMenu_BaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.RefineMenu.BaseMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugUnitItemMenu_RefineMenu_BaseMenuItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_RefineMenu_BaseMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnWidth2", args = 0)]
    pub fn get_column_width2(self) -> f32;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_RefineMenu_BaseMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_RefineMenu_BaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_RefineMenu_BaseMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_ItemBaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.ItemBaseMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_ItemBaseMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_ItemBaseMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_ItemBaseMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_ItemBaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_ItemBaseMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_EngraveMenu_ClearEngraveMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DebugUnitItemMenu.EngraveMenu.ClearEngraveMenuItem"
)]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_EngraveMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_EngraveMenu_ClearEngraveMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_EngraveMenu_ClearEngraveMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_EngraveMenu_ClearEngraveMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_EngraveMenu_ClearEngraveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_EngraveMenu_ClearEngraveMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_PriceMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.PriceMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_ItemBaseMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_PriceMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_PriceMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_PriceMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_PriceMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_PriceMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_SellingMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.SellingMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_ItemBaseMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_SellingMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_SellingMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_SellingMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_SellingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_SellingMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_FlagMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.FlagMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_FlagMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_FlagMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "SetFlag", args = 1)]
    pub fn set_flag(self, enable: bool) -> ();

    #[method(name = "IsFlag", args = 0)]
    pub fn is_flag(self) -> bool;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_FlagMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_FlagMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_FlagMenuItemMethods>::ctor(this, unit, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_EnchantHashMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DebugUnitItemMenu.SubMenu.EnchantHashMenuItem"
)]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_ItemBaseMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_EnchantHashMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_EnchantHashMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_EnchantHashMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_EnchantHashMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_EnchantHashMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugUnitItemMenu_SubMenu {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_UnitDebugMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.UnitDebugMenu")]
#[parent(crate::app::debugmenu::DebugMenu)]
pub struct DebugUnitItemMenu_UnitDebugMenu {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_UnitDebugMenu {
    #[method(name = "SetUnit", args = 1)]
    pub fn set_unit(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::debugunititemmenu::DebugUnitItemMenu_UnitDebugMenu;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_UnitDebugMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_UnitDebugMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_UnitDebugMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_ItemListMenu_ItemMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DebugUnitItemMenu.ItemListMenu.ItemMenuItem"
)]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugUnitItemMenu_ItemListMenu_ItemMenuItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_IsEnable")]
    pub m_is_enable: bool,
}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_ItemListMenu_ItemMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        index: i32,
    ) -> ();

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_ItemListMenu_ItemMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        index: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_ItemListMenu_ItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_ItemListMenu_ItemMenuItemMethods>::ctor(
            this, unit, unit_item, index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_EngraveMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.EngraveMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugUnitItemMenu_EngraveMenu {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_EngraveMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_EngraveMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_EngraveMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_EngraveMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugUnitItemMenu {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu {
    #[method(name = "UpdateEquip", args = 2)]
    pub fn update_equip(
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Postprocess", args = 1)]
    pub fn postprocess(unit: crate::app::unit::Unit) -> ();

    #[method(name = "IsValid", args = 1)]
    pub fn is_valid(item: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "IsValid", args = 1)]
    pub fn is_valid_2(unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugunititemmenu/DebugUnitItemMenu_SubMenu_EngraveMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugUnitItemMenu.SubMenu.EngraveMenuItem")]
#[parent(crate::app::debugunititemmenu::DebugUnitItemMenu_SubMenu_BaseMenuItem)]
pub struct DebugUnitItemMenu_SubMenu_EngraveMenuItem {}

#[cfg(feature = "app-debugunititemmenu")]
#[::unity2::methods]
impl DebugUnitItemMenu_SubMenu_EngraveMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugunititemmenu")]
impl DebugUnitItemMenu_SubMenu_EngraveMenuItem {
    pub fn new(unit: crate::app::unit::Unit, unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUnitItemMenu_SubMenu_EngraveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUnitItemMenu_SubMenu_EngraveMenuItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
        );
        this
    }
}
