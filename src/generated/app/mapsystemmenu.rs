
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubFriendMenu_KizunaItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubFriendMenu.KizunaItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubFriendMenu_KizunaItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubFriendMenu_KizunaItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubFriendMenu_KizunaItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubFriendMenu_KizunaItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubFriendMenu_KizunaItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubSystemMenu_SurrenderItem_ConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSystemMenu.SubSystemMenu.SurrenderItem.ConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MapSystemMenu_SubSystemMenu_SurrenderItem_ConfirmDialogItemYes {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubSystemMenu_SurrenderItem_ConfirmDialogItemYes {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubSystemMenu_SurrenderItem_ConfirmDialogItemYes {
    pub fn new(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubSystemMenu_SurrenderItem_ConfirmDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubSystemMenu_SurrenderItem_ConfirmDialogItemYesMethods>::ctor(
            this, text,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_TurnItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.TurnItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_TurnItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_TurnItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_TurnItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_TurnItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_TurnItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu")]
#[parent(crate::app::minimapbasicmenu::MiniMapBasicMenu)]
pub struct MapSystemMenu {
    #[static_field]
    #[rename(name = "m_Select")]
    pub m_select: crate::app::basicmenuselect::BasicMenuSelect,
}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "ResetSelect", args = 0)]
    pub fn reset_select() -> ();

    #[method(name = "GetRewindAttribute", args = 0)]
    pub fn get_rewind_attribute() -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

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

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "HelpOn", args = 0)]
    pub fn help_on(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        sortie_top_menu_content: crate::app::sortietopmenucontent::SortieTopMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenuMethods>::ctor(this, menu_item_list, sortie_top_menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubSystemMenu_RestartItem_ConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapSystemMenu.SubSystemMenu.RestartItem.ConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MapSystemMenu_SubSystemMenu_RestartItem_ConfirmDialogItemYes {
    #[rename(name = "m_Target")]
    pub m_target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes,
}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubSystemMenu_RestartItem_ConfirmDialogItemYes {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubSystemMenu_RestartItem_ConfirmDialogItemYes {
    pub fn new(target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubSystemMenu_RestartItem_ConfirmDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubSystemMenu_RestartItem_ConfirmDialogItemYesMethods>::ctor(
            this, target,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubFriendMenu_NotebookItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubFriendMenu.NotebookItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubFriendMenu_NotebookItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubFriendMenu_NotebookItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubFriendMenu_NotebookItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubFriendMenu_NotebookItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubFriendMenu_NotebookItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubSystemMenu_ResetItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubSystemMenu.ResetItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubSystemMenu_ResetItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubSystemMenu_ResetItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubSystemMenu_ResetItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubSystemMenu_ResetItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubSystemMenu_ResetItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubFriendMenu_RelianceItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubFriendMenu.RelianceItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubFriendMenu_RelianceItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubFriendMenu_RelianceItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubFriendMenu_RelianceItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubFriendMenu_RelianceItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubFriendMenu_RelianceItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubSystemMenu_ConfigItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubSystemMenu.ConfigItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubSystemMenu_ConfigItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubSystemMenu_ConfigItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubSystemMenu_ConfigItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubSystemMenu_ConfigItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubSystemMenu_ConfigItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_FriendMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.FriendMenuItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_FriendMenuItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_FriendMenuItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_FriendMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_FriendMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_FriendMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubFriendMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubFriendMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MapSystemMenu_SubFriendMenu {
    #[static_field]
    #[rename(name = "m_parentMenu")]
    pub m_parent_menu: crate::app::mapsystemmenu::MapSystemMenu,
}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubFriendMenu {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubFriendMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubFriendMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubFriendMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_RewindMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.RewindMenuItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_RewindMenuItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_RewindMenuItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_RewindMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_RewindMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_RewindMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_MapSystemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.MapSystemMenuItem")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct MapSystemMenu_MapSystemMenuItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_MapSystemMenuItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "HelpOn", args = 0)]
    pub fn help_on(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_MapSystemMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_MapSystemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_MapSystemMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_TemporarySaveItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.TemporarySaveItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_TemporarySaveItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_TemporarySaveItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_TemporarySaveItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_TemporarySaveItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_TemporarySaveItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SystemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SystemMenuItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SystemMenuItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SystemMenuItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SystemMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SystemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SystemMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubSystemMenu_RestartItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubSystemMenu.RestartItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubSystemMenu_RestartItem {
    #[rename(name = "m_Target")]
    pub m_target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes,
}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubSystemMenu_RestartItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes) -> ();

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
}

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubSystemMenu_RestartItem {
    pub fn new(target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubSystemMenu_RestartItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubSystemMenu_RestartItemMethods>::ctor(this, target);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_UnitListItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.UnitListItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_UnitListItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_UnitListItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_UnitListItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_UnitListItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_UnitListItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubSystemMenu_TutorialItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubSystemMenu.TutorialItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubSystemMenu_TutorialItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubSystemMenu_TutorialItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubSystemMenu_TutorialItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubSystemMenu_TutorialItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubSystemMenu_TutorialItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubSystemMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubSystemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MapSystemMenu_SubSystemMenu {
    #[static_field]
    #[rename(name = "m_parentMenu")]
    pub m_parent_menu: crate::app::mapsystemmenu::MapSystemMenu,
}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubSystemMenu {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubSystemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubSystemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubSystemMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubSystemMenu_SurrenderItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubSystemMenu.SurrenderItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubSystemMenu_SurrenderItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubSystemMenu_SurrenderItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubSystemMenu_SurrenderItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubSystemMenu_SurrenderItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubSystemMenu_SurrenderItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_SubFriendMenu_RingListItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.SubFriendMenu.RingListItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_SubFriendMenu_RingListItem {}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_SubFriendMenu_RingListItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_SubFriendMenu_RingListItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_SubFriendMenu_RingListItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_SubFriendMenu_RingListItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsystemmenu/MapSystemMenu_OrderItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSystemMenu.OrderItem")]
#[parent(crate::app::mapsystemmenu::MapSystemMenu_MapSystemMenuItem)]
pub struct MapSystemMenu_OrderItem {
    #[rename(name = "m_SortieTopMenuContent")]
    pub m_sortie_top_menu_content: crate::app::sortietopmenucontent::SortieTopMenuContent,
}

#[cfg(feature = "app-mapsystemmenu")]
#[::unity2::methods]
impl MapSystemMenu_OrderItem {
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

#[cfg(feature = "app-mapsystemmenu")]
impl MapSystemMenu_OrderItem {
    pub fn new(
        sortie_top_menu_content: crate::app::sortietopmenucontent::SortieTopMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSystemMenu_OrderItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSystemMenu_OrderItemMethods>::ctor(this, sortie_top_menu_content);
        this
    }
}
