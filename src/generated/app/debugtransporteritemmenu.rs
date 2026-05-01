
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransporteritemmenu/DebugTransporterItemMenu_SelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterItemMenu.SelectMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugTransporterItemMenu_SelectMenuItem {
    #[rename(name = "m_TransporterIndex")]
    pub m_transporter_index: i32,
    #[rename(name = "m_ItemIndex")]
    pub m_item_index: i32,
}

#[cfg(feature = "app-debugtransporteritemmenu")]
#[::unity2::methods]
impl DebugTransporterItemMenu_SelectMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, transporter_index: i32, item_index: i32) -> ();

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

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugtransporteritemmenu")]
impl DebugTransporterItemMenu_SelectMenuItem {
    pub fn new(transporter_index: i32, item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterItemMenu_SelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterItemMenu_SelectMenuItemMethods>::ctor(
            this,
            transporter_index,
            item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransporteritemmenu/DebugTransporterItemMenu_EditMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterItemMenu.EditMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugTransporterItemMenu_EditMenuItem {
    #[rename(name = "m_TransporterIndex")]
    pub m_transporter_index: i32,
}

#[cfg(feature = "app-debugtransporteritemmenu")]
#[::unity2::methods]
impl DebugTransporterItemMenu_EditMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, index: i32) -> ();

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnWidth2", args = 0)]
    pub fn get_column_width2(self) -> f32;

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnAlign0", args = 0)]
    pub fn get_column_align0(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetColumnAlign1", args = 0)]
    pub fn get_column_align1(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetColumnAlign2", args = 0)]
    pub fn get_column_align2(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-debugtransporteritemmenu")]
impl DebugTransporterItemMenu_EditMenuItem {
    pub fn new(index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterItemMenu_EditMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterItemMenu_EditMenuItemMethods>::ctor(this, index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransporteritemmenu/DebugTransporterItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterItemMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugTransporterItemMenu {}

#[cfg(feature = "app-debugtransporteritemmenu")]
#[::unity2::methods]
impl DebugTransporterItemMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugtransporteritemmenu")]
impl DebugTransporterItemMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterItemMenuMethods>::ctor(this);
        this
    }
}
