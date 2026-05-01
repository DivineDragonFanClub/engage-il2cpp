
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::basicmenuitemcontent::BasicMenuItemContent;
use crate::app::basicmenuitemcontent::IBasicMenuItemContent;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_Menu.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.Menu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RingListSkillMenu_Menu {
    #[rename(name = "m_CancelEventHandler")]
    pub m_cancel_event_handler: crate::app::ringlistskillmenu::RingListSkillMenu_CancelEventHandler,
    #[rename(name = "m_IsBindParent")]
    pub m_is_bind_parent: bool,
}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_Menu {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        cancel: crate::app::ringlistskillmenu::RingListSkillMenu_CancelEventHandler,
        is_bind_parent: bool,
    ) -> ();

    #[method(name = "CreateMenu", args = 6)]
    pub fn create_menu(
        super_: crate::app::procinst::ProcInst,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        ms: crate::app::basicmenuselect::BasicMenuSelect,
        cancel: crate::app::ringlistskillmenu::RingListSkillMenu_CancelEventHandler,
        is_bind_parent: bool,
    ) -> crate::app::ringlistskillmenu::RingListSkillMenu_Menu;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetDisplayIndex", args = 0)]
    pub fn get_display_index(self) -> i32;
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        cancel: crate::app::ringlistskillmenu::RingListSkillMenu_CancelEventHandler,
        is_bind_parent: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            cancel,
            is_bind_parent,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_MenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.MenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RingListSkillMenu_MenuItem {
    #[rename(name = "m_Root")]
    pub m_root: crate::app::ringlistskillmenu::RingListSkillMenu,
}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_MenuItem {
    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_IsGotSKill", args = 0)]
    pub fn get_is_got_s_kill(self) -> bool;

    #[method(name = "set_IsGotSKill", args = 1)]
    pub fn set_is_got_s_kill(self, value: bool) -> ();

    #[method(name = "get_IsEnableGotIcon", args = 0)]
    pub fn get_is_enable_got_icon(self) -> bool;

    #[method(name = "get_IsEnableUnitGotIcon", args = 0)]
    pub fn get_is_enable_unit_got_icon(self) -> bool;

    #[method(name = "get_FromLv", args = 0)]
    pub fn get_from_lv(self) -> i32;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        is_got_skill: bool,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetWidth", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "GetHeight", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_MenuItem {
    pub fn new(
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        is_got_skill: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_MenuItemMethods>::ctor(this, root, level, is_got_skill);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_MenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.MenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct RingListSkillMenu_MenuItemContent {
    #[rename(name = "m_Level")]
    pub m_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_LevelText")]
    pub m_level_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconImage")]
    pub m_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_IconFrameImage")]
    pub m_icon_frame_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GotCheckImage")]
    pub m_got_check_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_IsDisable")]
    pub m_is_disable: bool,
    #[rename(name = "m_IsInitialized")]
    pub m_is_initialized: bool,
}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_MenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item: crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem,
    ) -> ();

    #[method(name = "SetDisable", args = 1)]
    pub fn set_disable(self, disable: bool) -> ();

    #[method(name = "BuildTextColor", args = 0)]
    pub fn build_text_color(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_MenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_MenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_MenuItemContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_MenuItem_EngageItem.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.MenuItem_EngageItem")]
#[parent(crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem)]
pub struct RingListSkillMenu_MenuItem_EngageItem {}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_MenuItem_EngageItem {
    #[method(name = "get_Item", args = 0)]
    pub fn get_item(self) -> crate::app::itemdata::ItemData;

    #[method(name = "set_Item", args = 1)]
    pub fn set_item(self, value: crate::app::itemdata::ItemData) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        item_data: crate::app::itemdata::ItemData,
        is_got_skill: bool,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_MenuItem_EngageItem {
    pub fn new(
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        item_data: crate::app::itemdata::ItemData,
        is_got_skill: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_MenuItem_EngageItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_MenuItem_EngageItemMethods>::ctor(
            this,
            root,
            level,
            item_data,
            is_got_skill,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_MenuItem_ExtraSkill_SkillType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RingListSkillMenu_MenuItem_ExtraSkill_SkillType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RingListSkillMenu_MenuItem_ExtraSkill_SkillType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RingListSkillMenu.MenuItem_ExtraSkill.SkillType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RingListSkillMenu_MenuItem_ExtraSkill_SkillType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RingListSkillMenu_MenuItem_ExtraSkill_SkillType {
    pub fn unlock_skill_inheritance() -> Self {
        Self { value: 0 }
    }

    pub fn add_engage_turn_limit() -> Self {
        Self { value: 1 }
    }

    pub fn sub_engage_count_limit() -> Self {
        Self { value: 2 }
    }

    pub fn max() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_MenuContent.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.MenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct RingListSkillMenu_MenuContent {}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_MenuContent {
    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_MenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_MenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_MenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_ItemHelpWindow.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.ItemHelpWindow")]
#[parent(crate::system::object::Object)]
pub struct RingListSkillMenu_ItemHelpWindow {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RootAnimator")]
    pub m_root_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_ItemDetailSetter")]
    pub m_item_detail_setter: crate::app::itemmenudetailsetter::ItemMenuDetailSetter,
}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_ItemHelpWindow {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetItemData", args = 1)]
    pub fn set_item_data(self, item_data: crate::app::itemdata::ItemData) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_ItemHelpWindow {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_ItemHelpWindow),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_ItemHelpWindowMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_MenuItem_Skill.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.MenuItem_Skill")]
#[parent(crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem)]
pub struct RingListSkillMenu_MenuItem_Skill {}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_MenuItem_Skill {
    #[method(name = "get_Skill", args = 0)]
    pub fn get_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "set_Skill", args = 1)]
    pub fn set_skill(self, value: crate::app::skilldata::SkillData) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        skill_data: crate::app::skilldata::SkillData,
        is_got_skill: bool,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_MenuItem_Skill {
    pub fn new(
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        skill_data: crate::app::skilldata::SkillData,
        is_got_skill: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_MenuItem_Skill),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_MenuItem_SkillMethods>::ctor(
            this,
            root,
            level,
            skill_data,
            is_got_skill,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_MenuItem_WeaponTalent.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.MenuItem_WeaponTalent")]
#[parent(crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem)]
pub struct RingListSkillMenu_MenuItem_WeaponTalent {
    #[static_field]
    #[rename(name = "ItemKindTable")]
    pub item_kind_table: ::unity2::Array<crate::app::itemdata::ItemData_Kinds>,
    #[static_field]
    #[rename(name = "NameMidTable")]
    pub name_mid_table: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "HelpMidTable")]
    pub help_mid_table: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_MenuItem_WeaponTalent {
    #[method(name = "get_ItemKindTableIndex", args = 0)]
    pub fn get_item_kind_table_index(self) -> i32;

    #[method(name = "set_ItemKindTableIndex", args = 1)]
    pub fn set_item_kind_table_index(self, value: i32) -> ();

    #[method(name = "GetSprite", args = 0)]
    pub fn get_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetSkillName", args = 0)]
    pub fn get_skill_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSkillHelp", args = 0)]
    pub fn get_skill_help(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        is_got_skill: bool,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_MenuItem_WeaponTalent {
    pub fn new(
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        is_got_skill: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_MenuItem_WeaponTalent),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_MenuItem_WeaponTalentMethods>::ctor(
            this,
            root,
            level,
            item_kind,
            is_got_skill,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_CancelEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.CancelEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RingListSkillMenu_CancelEventHandler {}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_CancelEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_CancelEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_CancelEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_CancelEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_SkillHelpWindow.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.SkillHelpWindow")]
#[parent(crate::system::object::Object)]
pub struct RingListSkillMenu_SkillHelpWindow {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RootAnimator")]
    pub m_root_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_HelpText")]
    pub m_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_SkillHelpWindow {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetSkillData", args = 1)]
    pub fn set_skill_data(
        self,
        menu_item: crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem_Skill,
    ) -> ();

    #[method(name = "SetSkillData", args = 1)]
    pub fn set_skill_data_2(
        self,
        menu_item: crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem_ExtraSkill,
    ) -> ();

    #[method(name = "SetItemKind", args = 1)]
    pub fn set_item_kind(
        self,
        menu_item: crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem_WeaponTalent,
    ) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_SkillHelpWindow {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_SkillHelpWindow),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_SkillHelpWindowMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu")]
#[parent(crate::system::object::Object)]
pub struct RingListSkillMenu {
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::ringlistskillmenu::RingListSkillMenu_Menu,
    #[rename(name = "m_menuContent")]
    pub m_menu_content: crate::app::basicmenucontent::BasicMenuContent,
}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu {
    #[method(name = "get_RootObject", args = 0)]
    pub fn get_root_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_ItemHelpWdw", args = 0)]
    pub fn get_item_help_wdw(
        self,
    ) -> crate::app::ringlistskillmenu::RingListSkillMenu_ItemHelpWindow;

    #[method(name = "set_ItemHelpWdw", args = 1)]
    pub fn set_item_help_wdw(
        self,
        value: crate::app::ringlistskillmenu::RingListSkillMenu_ItemHelpWindow,
    ) -> ();

    #[method(name = "get_SkillHelpWdw", args = 0)]
    pub fn get_skill_help_wdw(
        self,
    ) -> crate::app::ringlistskillmenu::RingListSkillMenu_SkillHelpWindow;

    #[method(name = "set_SkillHelpWdw", args = 1)]
    pub fn set_skill_help_wdw(
        self,
        value: crate::app::ringlistskillmenu::RingListSkillMenu_SkillHelpWindow,
    ) -> ();

    #[method(name = "get_IsEnableGotIcon", args = 0)]
    pub fn get_is_enable_got_icon(self) -> bool;

    #[method(name = "set_IsEnableGotIcon", args = 1)]
    pub fn set_is_enable_got_icon(self, value: bool) -> ();

    #[method(name = "get_IsEnableUnitGotIcon", args = 0)]
    pub fn get_is_enable_unit_got_icon(self) -> bool;

    #[method(name = "set_IsEnableUnitGotIcon", args = 1)]
    pub fn set_is_enable_unit_got_icon(self, value: bool) -> ();

    #[method(name = "get_FromLv", args = 0)]
    pub fn get_from_lv(self) -> i32;

    #[method(name = "set_FromLv", args = 1)]
    pub fn set_from_lv(self, value: i32) -> ();

    #[method(name = "GetMenu", args = 0)]
    pub fn get_menu(self) -> crate::app::ringlistskillmenu::RingListSkillMenu_Menu;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "CreateGodGrowthMenuItem", args = 7)]
    pub fn create_god_growth_menu_item(
        god_data: crate::app::goddata::GodData,
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        from_lv: i32,
        to_lv: i32,
        max_bond_level: i32,
        out_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        is_ring_select_menu: bool,
    ) -> ();

    #[method(name = "CreateMenu", args = 8)]
    pub fn create_menu(
        self,
        super_: crate::app::procinst::ProcInst,
        is_bind_parent: bool,
        is_enable_got_icon: bool,
        is_enable_unit_got_icon: bool,
        god_page_data: crate::app::ringlistsequence::RingListSequence_GodPageData,
        from_lv: i32,
        cancel: crate::app::ringlistskillmenu::RingListSkillMenu_CancelEventHandler,
        is_ring_select_menu: bool,
    ) -> bool;

    #[method(name = "Delete", args = 0)]
    pub fn delete(self) -> ();

    #[method(name = "IncSuspend", args = 0)]
    pub fn inc_suspend(self) -> ();

    #[method(name = "DecSuspend", args = 0)]
    pub fn dec_suspend(self) -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenuMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringlistskillmenu/RingListSkillMenu_MenuItem_ExtraSkill.md")))]
#[::unity2::class(namespace = "App", name = "RingListSkillMenu.MenuItem_ExtraSkill")]
#[parent(crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem)]
pub struct RingListSkillMenu_MenuItem_ExtraSkill {
    #[static_field]
    #[rename(name = "SpriteNameTable")]
    pub sprite_name_table: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "NameMidTable")]
    pub name_mid_table: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "HelpMidTable")]
    pub help_mid_table: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-ringlistskillmenu")]
#[::unity2::methods]
impl RingListSkillMenu_MenuItem_ExtraSkill {
    #[method(name = "get_Type", args = 0)]
    pub fn get_type(
        self,
    ) -> crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem_ExtraSkill_SkillType;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(
        self,
        value: crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem_ExtraSkill_SkillType,
    ) -> ();

    #[method(name = "GetSprite", args = 0)]
    pub fn get_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetSkillName", args = 0)]
    pub fn get_skill_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSkillHelp", args = 0)]
    pub fn get_skill_help(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        skill_type: crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem_ExtraSkill_SkillType,
        is_got_skill: bool,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ringlistskillmenu")]
impl RingListSkillMenu_MenuItem_ExtraSkill {
    pub fn new(
        root: crate::app::ringlistskillmenu::RingListSkillMenu,
        level: i32,
        skill_type: crate::app::ringlistskillmenu::RingListSkillMenu_MenuItem_ExtraSkill_SkillType,
        is_got_skill: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingListSkillMenu_MenuItem_ExtraSkill),
                ::core::stringify!(new),
            )
        });
        <Self as IRingListSkillMenu_MenuItem_ExtraSkillMethods>::ctor(
            this,
            root,
            level,
            skill_type,
            is_got_skill,
        );
        this
    }
}
