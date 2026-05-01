
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugaccessorymenu/DebugAccessoryMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugAccessoryMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugAccessoryMenu {}

#[cfg(feature = "app-debugaccessorymenu")]
#[::unity2::methods]
impl DebugAccessoryMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugaccessorymenu")]
impl DebugAccessoryMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugAccessoryMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugAccessoryMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugaccessorymenu/DebugAccessoryMenu_AddAllMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugAccessoryMenu.AddAllMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugAccessoryMenu_AddAllMenuItem {}

#[cfg(feature = "app-debugaccessorymenu")]
#[::unity2::methods]
impl DebugAccessoryMenu_AddAllMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugaccessorymenu")]
impl DebugAccessoryMenu_AddAllMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugAccessoryMenu_AddAllMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugAccessoryMenu_AddAllMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugaccessorymenu/DebugAccessoryMenu_DeleteAllMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugAccessoryMenu.DeleteAllMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugAccessoryMenu_DeleteAllMenuItem {}

#[cfg(feature = "app-debugaccessorymenu")]
#[::unity2::methods]
impl DebugAccessoryMenu_DeleteAllMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugaccessorymenu")]
impl DebugAccessoryMenu_DeleteAllMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugAccessoryMenu_DeleteAllMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugAccessoryMenu_DeleteAllMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugaccessorymenu/DebugAccessoryMenu_AccessoryMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugAccessoryMenu.AccessoryMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugAccessoryMenu_AccessoryMenuItem {
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::accessorydata::AccessoryData,
}

#[cfg(feature = "app-debugaccessorymenu")]
#[::unity2::methods]
impl DebugAccessoryMenu_AccessoryMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, index: i32) -> ();

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-debugaccessorymenu")]
impl DebugAccessoryMenu_AccessoryMenuItem {
    pub fn new(index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugAccessoryMenu_AccessoryMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugAccessoryMenu_AccessoryMenuItemMethods>::ctor(this, index);
        this
    }
}
