
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::mapbasicmenuitem::IMapBasicMenuItem;
use crate::app::mapbasicmenuitem::MapBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_InventoryItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.InventoryItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_InventoryItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_InventoryItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_InventoryItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_InventoryItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_InventoryItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SaveItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SaveItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SaveItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SaveItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SaveItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SaveItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SaveItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_NextChapterItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.NextChapterItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_NextChapterItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_NextChapterItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_NextChapterItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_NextChapterItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_NextChapterItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubFriendMenu_NotebookItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubFriendMenu.NotebookItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SubFriendMenu_NotebookItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubFriendMenu_NotebookItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubFriendMenu_NotebookItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubFriendMenu_NotebookItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubFriendMenu_NotebookItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_NextItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.NextItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_NextItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_NextItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_NextItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_NextItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_NextItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubSystemMenu_TutorialItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubSystemMenu.TutorialItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SubSystemMenu_TutorialItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubSystemMenu_TutorialItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubSystemMenu_TutorialItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubSystemMenu_TutorialItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubSystemMenu_TutorialItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_GmapItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.GmapItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_GmapItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_GmapItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_GmapItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_GmapItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_GmapItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_HubMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.HubMenuItem")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct HubMenu_HubMenuItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_HubMenuItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SetHelpVisible", args = 1)]
    pub fn set_help_visible(self, active: bool) -> ();

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_HubMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_HubMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_HubMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubSystemMenu_ResetItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubSystemMenu.ResetItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SubSystemMenu_ResetItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubSystemMenu_ResetItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubSystemMenu_ResetItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubSystemMenu_ResetItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubSystemMenu_ResetItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubFriendMenu_RingListItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubFriendMenu.RingListItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SubFriendMenu_RingListItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubFriendMenu_RingListItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubFriendMenu_RingListItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubFriendMenu_RingListItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubFriendMenu_RingListItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubFriendMenu_ProfileCardItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubFriendMenu.ProfileCardItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SubFriendMenu_ProfileCardItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubFriendMenu_ProfileCardItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubFriendMenu_ProfileCardItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubFriendMenu_ProfileCardItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubFriendMenu_ProfileCardItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubFriendMenu_KizunaItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubFriendMenu.KizunaItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SubFriendMenu_KizunaItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubFriendMenu_KizunaItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubFriendMenu_KizunaItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubFriendMenu_KizunaItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubFriendMenu_KizunaItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubSystemMenu_ConfigItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubSystemMenu.ConfigItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SubSystemMenu_ConfigItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubSystemMenu_ConfigItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubSystemMenu_ConfigItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubSystemMenu_ConfigItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubSystemMenu_ConfigItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct HubMenu {
    #[static_field]
    #[rename(name = "currentMenuSelect")]
    pub current_menu_select: crate::app::basicmenuselect::BasicMenuSelect,
}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init() -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "OnResume", args = 0)]
    pub fn on_resume(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_FriendItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.FriendItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_FriendItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_FriendItem {
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

#[cfg(feature = "app-hubmenu")]
impl HubMenu_FriendItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_FriendItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_FriendItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubSystemMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubSystemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct HubMenu_SubSystemMenu {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubSystemMenu {
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
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubSystemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubSystemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubSystemMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_MaterialListItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.MaterialListItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_MaterialListItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_MaterialListItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_MaterialListItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_MaterialListItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_MaterialListItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubFriendMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubFriendMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct HubMenu_SubFriendMenu {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubFriendMenu {
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
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubFriendMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubFriendMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubFriendMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_GodItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.GodItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_GodItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_GodItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_GodItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_GodItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_GodItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_MapInfoItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.MapInfoItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_MapInfoItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_MapInfoItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_MapInfoItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_MapInfoItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_MapInfoItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SubFriendMenu_RelianceItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SubFriendMenu.RelianceItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SubFriendMenu_RelianceItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SubFriendMenu_RelianceItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SubFriendMenu_RelianceItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SubFriendMenu_RelianceItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SubFriendMenu_RelianceItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmenu/HubMenu_SystemItem.md")))]
#[::unity2::class(namespace = "App", name = "HubMenu.SystemItem")]
#[parent(crate::app::hubmenu::HubMenu_HubMenuItem)]
pub struct HubMenu_SystemItem {}

#[cfg(feature = "app-hubmenu")]
#[::unity2::methods]
impl HubMenu_SystemItem {
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

#[cfg(feature = "app-hubmenu")]
impl HubMenu_SystemItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMenu_SystemItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMenu_SystemItemMethods>::ctor(this);
        this
    }
}
