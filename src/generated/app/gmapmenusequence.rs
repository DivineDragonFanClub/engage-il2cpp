
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::mapbasicmenuitem::IMapBasicMenuItem;
use crate::app::mapbasicmenuitem::MapBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: gmapmenusequence :: GmapMenuSequence >)]
pub struct GmapMenuSequence {
    #[rename(name = "m_OpenTopCallback")]
    pub m_open_top_callback: crate::system::action::Action,
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::system::action::Action,
    #[rename(name = "m_CloseCallback")]
    pub m_close_callback: crate::system::action::Action,
}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        open_top_callback: crate::system::action::Action,
        decide_callback: crate::system::action::Action,
        close_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "OpenTopCallback", args = 0)]
    pub fn open_top_callback(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        open_top_callback: crate::system::action::Action,
        decide_callback: crate::system::action::Action,
        close_callback: crate::system::action::Action,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "OpenTopMenu", args = 0)]
    pub fn open_top_menu(self) -> ();

    #[method(name = "EnterChapterDialog", args = 0)]
    pub fn enter_chapter_dialog(self) -> ();

    #[method(name = "Final", args = 0)]
    pub fn r#final(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence {
    pub fn new(
        open_top_callback: crate::system::action::Action,
        decide_callback: crate::system::action::Action,
        close_callback: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequenceMethods>::ctor(
            this,
            open_top_callback,
            decide_callback,
            close_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_GodItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.GodItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_GodItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_GodItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_GodItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_GodItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_GodItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubFriendMenu_KizunaItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubFriendMenu.KizunaItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubFriendMenu_KizunaItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubFriendMenu_KizunaItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubFriendMenu_KizunaItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubFriendMenu_KizunaItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubFriendMenu_KizunaItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_GmapMenuMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.GmapMenuMenuItem")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct GmapMenuSequence_GmapMenu_GmapMenuMenuItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_GmapMenuMenuItem {
    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SetHelpVisible", args = 1)]
    pub fn set_help_visible(self, active: bool) -> ();

    #[method(name = "SaveMenuSelect", args = 0)]
    pub fn save_menu_select(self) -> ();

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_GmapMenuMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_GmapMenuMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_GmapMenuMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_ShopItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.ShopItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_ShopItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_ShopItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_ShopItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_ShopItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_ShopItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_InventoryItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.InventoryItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_InventoryItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_InventoryItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_InventoryItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_InventoryItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_InventoryItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_MaterialListItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.MaterialListItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_MaterialListItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_MaterialListItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_MaterialListItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_MaterialListItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_MaterialListItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapMenuSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapMenuSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapMenuSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapMenuSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapMenuSequence_Label {
    pub fn top_menu() -> Self {
        Self { value: 0 }
    }

    pub fn enter_chapter() -> Self {
        Self { value: 1 }
    }

    pub fn inventory() -> Self {
        Self { value: 2 }
    }

    pub fn ring_select() -> Self {
        Self { value: 3 }
    }

    pub fn ranking() -> Self {
        Self { value: 4 }
    }

    pub fn material_list() -> Self {
        Self { value: 5 }
    }

    pub fn save_data() -> Self {
        Self { value: 6 }
    }

    pub fn go_to_solanel() -> Self {
        Self { value: 7 }
    }

    pub fn end() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.RankingItem.ConfirmSequence.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_YesMenuItem {
    #[rename(name = "m_ResultFunc")]
    pub m_result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_YesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_YesMenuItem {
    pub fn new(
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_YesMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_YesMenuItemMethods>::ctor(
            this,
            result_func,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubSystemMenu_TutorialItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubSystemMenu.TutorialItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubSystemMenu_TutorialItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubSystemMenu_TutorialItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubSystemMenu_TutorialItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubSystemMenu_TutorialItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubSystemMenu_TutorialItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubFriendMenu.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.SubFriendMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GmapMenuSequence_GmapMenu_SubFriendMenu {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubFriendMenu {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubFriendMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubFriendMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubFriendMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.RankingItem.ConfirmSequence"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence {
    #[rename(name = "m_ResultFunc")]
    pub m_result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> ();

    #[method(name = "Confirm", args = 0)]
    pub fn confirm(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence {
    pub fn new(
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_RankingItem_ConfirmSequenceMethods>::ctor(
            this,
            result_func,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubSystemMenu_ResetItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubSystemMenu.ResetItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubSystemMenu_ResetItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubSystemMenu_ResetItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubSystemMenu_ResetItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubSystemMenu_ResetItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubSystemMenu_ResetItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubFriendMenu_RingbookItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubFriendMenu.RingbookItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubFriendMenu_RingbookItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubFriendMenu_RingbookItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubFriendMenu_RingbookItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubFriendMenu_RingbookItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubFriendMenu_RingbookItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubFriendMenu_NotebookItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubFriendMenu.NotebookItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubFriendMenu_NotebookItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubFriendMenu_NotebookItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubFriendMenu_NotebookItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubFriendMenu_NotebookItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubFriendMenu_NotebookItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_FriendItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.FriendItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_FriendItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_FriendItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_FriendItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_FriendItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_FriendItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_RankingItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.RankingItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_RankingItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_RankingItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_RankingItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_RankingItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_RankingItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GoToMapSequence.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GoToMapSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GmapMenuSequence_GoToMapSequence {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GoToMapSequence {
    #[method(name = "OpenDialog", args = 0)]
    pub fn open_dialog(self) -> ();

    #[method(name = "MoveSolanel", args = 0)]
    pub fn move_solanel(self) -> ();

    #[method(name = "MoveGmap", args = 0)]
    pub fn move_gmap(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GoToMapSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GoToMapSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GoToMapSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubSystemMenu.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.SubSystemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GmapMenuSequence_GmapMenu_SubSystemMenu {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubSystemMenu {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubSystemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubSystemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubSystemMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_GoToSolanelItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.GoToSolanelItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_GoToSolanelItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_GoToSolanelItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_GoToSolanelItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_GoToSolanelItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_GoToSolanelItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubShopMenu_WeaponShopMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubShopMenu.WeaponShopMenuItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubShopMenu_WeaponShopMenuItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubShopMenu_WeaponShopMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubShopMenu_WeaponShopMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubShopMenu_WeaponShopMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubShopMenu_WeaponShopMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GmapMenuSequence_GmapMenu {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu {
    #[method(name = "get_EnterChapterType", args = 0)]
    pub fn get_enter_chapter_type(
    ) -> crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type;

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

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "AddStartChapterItem", args = 1)]
    pub fn add_start_chapter_item(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "OnResume", args = 0)]
    pub fn on_resume(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GoToMapSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapMenuSequence_GoToMapSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapMenuSequence_GoToMapSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapMenuSequence.GoToMapSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapMenuSequence_GoToMapSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapMenuSequence_GoToMapSequence_Label {
    pub fn move_to_solanel() -> Self {
        Self { value: 0 }
    }

    pub fn move_to_gmap() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_RankingItem_JumpToRankingSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.RankingItem.JumpToRankingSequence"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct GmapMenuSequence_GmapMenu_RankingItem_JumpToRankingSequence {
    #[rename(name = "m_JumpFunc")]
    pub m_jump_func: crate::system::action::Action,
}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_RankingItem_JumpToRankingSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, jump_func: crate::system::action::Action) -> ();

    #[method(name = "JumpToRanking", args = 0)]
    pub fn jump_to_ranking(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        jump_func: crate::system::action::Action,
    ) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_RankingItem_JumpToRankingSequence {
    pub fn new(jump_func: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_RankingItem_JumpToRankingSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_RankingItem_JumpToRankingSequenceMethods>::ctor(
            this, jump_func,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubSystemMenu_ConfigItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubSystemMenu.ConfigItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubSystemMenu_ConfigItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubSystemMenu_ConfigItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubSystemMenu_ConfigItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubSystemMenu_ConfigItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubSystemMenu_ConfigItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_EnterChapterItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.EnterChapterItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_EnterChapterItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_EnterChapterItem {
    #[method(name = "get_Type", args = 0)]
    pub fn get_type() -> crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(
        value: crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        r#type: crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_EnterChapterItem {
    pub fn new(
        r#type: crate::app::gmap::enterchapteryesnodialog::EnterChapterYesNoDialog_Type,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_EnterChapterItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_EnterChapterItemMethods>::ctor(this, r#type);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubShopMenu_ItemShopMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubShopMenu.ItemShopMenuItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubShopMenu_ItemShopMenuItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubShopMenu_ItemShopMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubShopMenu_ItemShopMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubShopMenu_ItemShopMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubShopMenu_ItemShopMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SystemItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.SystemItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SystemItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SystemItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SystemItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SystemItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SystemItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SaveItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.SaveItem")]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SaveItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SaveItem {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SaveItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SaveItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SaveItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubShopMenu.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuSequence.GmapMenu.SubShopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GmapMenuSequence_GmapMenu_SubShopMenu {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubShopMenu {
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

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubShopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortiesubmenucontent::SortieSubMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubShopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubShopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_NoMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.RankingItem.ConfirmSequence.NoMenuItem"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_NoMenuItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_NoMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_NoMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    GmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_NoMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_RankingItem_ConfirmSequence_NoMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenusequence/GmapMenuSequence_GmapMenu_SubFriendMenu_RelianceItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GmapMenuSequence.GmapMenu.SubFriendMenu.RelianceItem"
)]
#[parent(crate::app::gmapmenusequence::GmapMenuSequence_GmapMenu_GmapMenuMenuItem)]
pub struct GmapMenuSequence_GmapMenu_SubFriendMenu_RelianceItem {}

#[cfg(feature = "app-gmapmenusequence")]
#[::unity2::methods]
impl GmapMenuSequence_GmapMenu_SubFriendMenu_RelianceItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmenusequence")]
impl GmapMenuSequence_GmapMenu_SubFriendMenu_RelianceItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuSequence_GmapMenu_SubFriendMenu_RelianceItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuSequence_GmapMenu_SubFriendMenu_RelianceItemMethods>::ctor(this);
        this
    }
}
