
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::mapbasicmenu::IMapBasicMenu;
use crate::app::mapbasicmenu::MapBasicMenu;
use crate::app::mapbasicmenuitem::IMapBasicMenuItem;
use crate::app::mapbasicmenuitem::MapBasicMenuItem;
use crate::app::minimapbasicmenu::IMiniMapBasicMenu;
use crate::app::minimapbasicmenu::MiniMapBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubSystemMenu_ConfigItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubSystemMenu.ConfigItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SubSystemMenu_ConfigItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubSystemMenu_ConfigItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubSystemMenu_ConfigItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubSystemMenu_ConfigItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubSystemMenu_ConfigItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_StartMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.StartMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_StartMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_StartMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_StartMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_StartMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_StartMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubFriendMenu_RingListItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubFriendMenu.RingListItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SubFriendMenu_RingListItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubFriendMenu_RingListItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubFriendMenu_RingListItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubFriendMenu_RingListItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubFriendMenu_RingListItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_BackMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.BackMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_BackMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_BackMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_BackMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_BackMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_BackMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SaveMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SaveMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SaveMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SaveMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SaveMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SaveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SaveMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SelectionUnitMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SelectionUnitMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SelectionUnitMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SelectionUnitMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SelectionUnitMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SelectionUnitMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SelectionUnitMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubSystemMenu_ReportItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubSystemMenu.ReportItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SubSystemMenu_ReportItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubSystemMenu_ReportItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubSystemMenu_ReportItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubSystemMenu_ReportItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubSystemMenu_ReportItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SaveMapMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SaveMapMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SaveMapMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SaveMapMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SaveMapMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SaveMapMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SaveMapMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubFriendMenu_KizunaItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubFriendMenu.KizunaItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SubFriendMenu_KizunaItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubFriendMenu_KizunaItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubFriendMenu_KizunaItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubFriendMenu_KizunaItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubFriendMenu_KizunaItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubFriendMenu_NotebookItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubFriendMenu.NotebookItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SubFriendMenu_NotebookItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubFriendMenu_NotebookItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubFriendMenu_NotebookItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubFriendMenu_NotebookItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubFriendMenu_NotebookItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SystemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SystemMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SystemMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SystemMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SystemMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SystemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SystemMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_ResetMapMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.ResetMapMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_ResetMapMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_ResetMapMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_ResetMapMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_ResetMapMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_ResetMapMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubFriendMenu.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubFriendMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SortieTopMenu_SubFriendMenu {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubFriendMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::basicmenu::BasicMenu,
        parent_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    ) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubFriendMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubFriendMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubFriendMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_InventoryMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.InventoryMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_InventoryMenuItem {
    #[rename(name = "m_SortieTopMenuContent")]
    pub m_sortie_top_menu_content: crate::app::sortietopmenucontent::SortieTopMenuContent,
}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_InventoryMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        sortie_top_menu_content: crate::app::sortietopmenucontent::SortieTopMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_InventoryMenuItem {
    pub fn new(
        sortie_top_menu_content: crate::app::sortietopmenucontent::SortieTopMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_InventoryMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_InventoryMenuItemMethods>::ctor(this, sortie_top_menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_PositionChangeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.PositionChangeMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_PositionChangeMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_PositionChangeMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_PositionChangeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_PositionChangeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_PositionChangeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_ShopMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.ShopMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_ShopMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_ShopMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_ShopMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_ShopMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_ShopMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_FriendMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.FriendMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_FriendMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_FriendMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_FriendMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_FriendMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_FriendMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubSystemMenu.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubSystemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SortieTopMenu_SubSystemMenu {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubSystemMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::basicmenu::BasicMenu,
        parent_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    ) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubSystemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubSystemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubSystemMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubFriendMenu_RelianceItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubFriendMenu.RelianceItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SubFriendMenu_RelianceItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubFriendMenu_RelianceItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubFriendMenu_RelianceItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubFriendMenu_RelianceItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubFriendMenu_RelianceItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubSystemMenu_TutorialItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubSystemMenu.TutorialItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SubSystemMenu_TutorialItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubSystemMenu_TutorialItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubSystemMenu_TutorialItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubSystemMenu_TutorialItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubSystemMenu_TutorialItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_RelianceMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.RelianceMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_RelianceMenuItem {
    #[rename(name = "m_CanTalk")]
    pub m_can_talk: bool,
}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_RelianceMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_RelianceMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_RelianceMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_RelianceMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SubSystemMenu_ResetItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SubSystemMenu.ResetItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_SubSystemMenu_ResetItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SubSystemMenu_ResetItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SubSystemMenu_ResetItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SubSystemMenu_ResetItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SubSystemMenu_ResetItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_SortieTopMenuItemBase.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.SortieTopMenuItemBase")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct SortieTopMenu_SortieTopMenuItemBase {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_SortieTopMenuItemBase {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "HelpOn", args = 0)]
    pub fn help_on(self) -> ();

    #[method(name = "GoToBattle", args = 0)]
    pub fn go_to_battle(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BackTo", args = 1)]
    pub fn back_to(self, is_cancel: bool) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SaveMenuSelect", args = 0)]
    pub fn save_menu_select(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_SortieTopMenuItemBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_SortieTopMenuItemBase),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_SortieTopMenuItemBaseMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu_GodMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu.GodMenuItem")]
#[parent(crate::app::sortietopmenu::SortieTopMenu_SortieTopMenuItemBase)]
pub struct SortieTopMenu_GodMenuItem {}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu_GodMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu_GodMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu_GodMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenu_GodMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenu/SortieTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenu")]
#[parent(crate::app::minimapbasicmenu::MiniMapBasicMenu)]
pub struct SortieTopMenu {
    #[static_field]
    #[rename(name = "TUTID_SORTIE")]
    pub tutid_sortie: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TUTID_GOD")]
    pub tutid_god: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TUTID_GOD2")]
    pub tutid_god2: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TUTID_ENTRY_PANEL")]
    pub tutid_entry_panel: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ENGAGE_PLUS_ATTENTION_NETWORK")]
    pub engage_plus_attention_network: ::unity2::Il2CppString,
}

#[cfg(feature = "app-sortietopmenu")]
#[::unity2::methods]
impl SortieTopMenu {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        sortie_top_menu_content: crate::app::sortietopmenucontent::SortieTopMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTutorial", args = 0)]
    pub fn get_tutorial(self) -> ::unity2::Il2CppString;

    #[method(name = "CheckEngagePlus", args = 0)]
    pub fn check_engage_plus(self) -> ();

    #[method(name = "HelpOn", args = 0)]
    pub fn help_on(self) -> ();
}

#[cfg(feature = "app-sortietopmenu")]
impl SortieTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        sortie_top_menu_content: crate::app::sortietopmenucontent::SortieTopMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenuMethods>::ctor(this, menu_item_list, sortie_top_menu_content);
        this
    }
}
