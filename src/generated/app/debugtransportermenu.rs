
use crate::app::labelitem::ILabelItem;
use crate::app::labelitem::LabelItem;
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::stringitem::IStringItem;
use crate::app::stringitem::StringItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransportermenu/DebugTransporterMenu_DeleteAllMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterMenu.DeleteAllMenuItem")]
#[parent(crate::app::debugtransportermenu::DebugTransporterMenu_BaseMenuItem)]
pub struct DebugTransporterMenu_DeleteAllMenuItem {}

#[cfg(feature = "app-debugtransportermenu")]
#[::unity2::methods]
impl DebugTransporterMenu_DeleteAllMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugtransportermenu")]
impl DebugTransporterMenu_DeleteAllMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterMenu_DeleteAllMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterMenu_DeleteAllMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransportermenu/DebugTransporterMenu_AddAllMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterMenu.AddAllMenuItem")]
#[parent(crate::app::debugtransportermenu::DebugTransporterMenu_BaseMenuItem)]
pub struct DebugTransporterMenu_AddAllMenuItem {}

#[cfg(feature = "app-debugtransportermenu")]
#[::unity2::methods]
impl DebugTransporterMenu_AddAllMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetNameEnglish", args = 0)]
    pub fn get_name_english(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugtransportermenu")]
impl DebugTransporterMenu_AddAllMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterMenu_AddAllMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterMenu_AddAllMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransportermenu/DebugTransporterMenu_AddByKindMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterMenu.AddByKindMenuItem")]
#[parent(crate::app::debugtransportermenu::DebugTransporterMenu_BaseMenuItem)]
pub struct DebugTransporterMenu_AddByKindMenuItem {
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::itemdata::ItemData_Kinds,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-debugtransportermenu")]
#[::unity2::methods]
impl DebugTransporterMenu_AddByKindMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, kind: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugtransportermenu")]
impl DebugTransporterMenu_AddByKindMenuItem {
    pub fn new(kind: crate::app::itemdata::ItemData_Kinds) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterMenu_AddByKindMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterMenu_AddByKindMenuItemMethods>::ctor(this, kind);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransportermenu/DebugTransporterMenu_LowestItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterMenu.LowestItemMenuItem")]
#[parent(crate::app::debugtransportermenu::DebugTransporterMenu_BaseMenuItem)]
pub struct DebugTransporterMenu_LowestItemMenuItem {}

#[cfg(feature = "app-debugtransportermenu")]
#[::unity2::methods]
impl DebugTransporterMenu_LowestItemMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetNameEnglish", args = 0)]
    pub fn get_name_english(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugtransportermenu")]
impl DebugTransporterMenu_LowestItemMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterMenu_LowestItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterMenu_LowestItemMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransportermenu/DebugTransporterMenu_TitleItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterMenu.TitleItem")]
#[parent(crate::app::labelitem::LabelItem)]
pub struct DebugTransporterMenu_TitleItem {}

#[cfg(feature = "app-debugtransportermenu")]
#[::unity2::methods]
impl DebugTransporterMenu_TitleItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugtransportermenu")]
impl DebugTransporterMenu_TitleItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterMenu_TitleItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterMenu_TitleItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransportermenu/DebugTransporterMenu_FillAllMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterMenu.FillAllMenuItem")]
#[parent(crate::app::debugtransportermenu::DebugTransporterMenu_BaseMenuItem)]
pub struct DebugTransporterMenu_FillAllMenuItem {}

#[cfg(feature = "app-debugtransportermenu")]
#[::unity2::methods]
impl DebugTransporterMenu_FillAllMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugtransportermenu")]
impl DebugTransporterMenu_FillAllMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterMenu_FillAllMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterMenu_FillAllMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransportermenu/DebugTransporterMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugTransporterMenu {}

#[cfg(feature = "app-debugtransportermenu")]
#[::unity2::methods]
impl DebugTransporterMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugtransportermenu")]
impl DebugTransporterMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtransportermenu/DebugTransporterMenu_BaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugTransporterMenu.BaseMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugTransporterMenu_BaseMenuItem {}

#[cfg(feature = "app-debugtransportermenu")]
#[::unity2::methods]
impl DebugTransporterMenu_BaseMenuItem {
    #[method(name = "CanAddToTransport", args = 1)]
    pub fn can_add_to_transport(self, item: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "AddItems", args = 0)]
    pub fn add_items(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugtransportermenu")]
impl DebugTransporterMenu_BaseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTransporterMenu_BaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTransporterMenu_BaseMenuItemMethods>::ctor(this);
        this
    }
}
