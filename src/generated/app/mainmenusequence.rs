
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
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
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::app::yesnodialog::IYesNoDialog;
use crate::app::yesnodialog::YesNoDialog;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_DifficultySelectMenuSequence_Menu_LunaticMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.DifficultySelectMenuSequence.Menu.LunaticMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_DifficultySelectMenuSequence_Menu_LunaticMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_LunaticMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_LunaticMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_DifficultySelectMenuSequence_Menu_LunaticMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_DifficultySelectMenuSequence_Menu_LunaticMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GameModeSelectMenuSequence.Menu.MenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItem {
    #[rename(name = "m_GameMode")]
    pub m_game_mode: crate::app::gamemode::GameMode,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, game_mode: crate::app::gamemode::GameMode) -> ();

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItem {
    pub fn new(game_mode: crate::app::gamemode::GameMode) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItemMethods>::ctor(
            this, game_mode,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GameModeSelectMenuSequence_Menu.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GameModeSelectMenuSequence.Menu"
)]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MainMenuSequence_GameModeSelectMenuSequence_Menu {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu {
    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "CreateMenuBind", args = 2)]
    pub fn create_menu_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: mainmenusequence :: MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuContent,
    ) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GameModeSelectMenuSequence_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GameModeSelectMenuSequence_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_NoDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu.ConfirmDialog.NoDialogItem"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_NoDialogItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_NoDialogItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_NoDialogItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_NoDialogItem
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_NoDialogItemMethods > :: ctor (this ,) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GameModeSelectMenuSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GameModeSelectMenuSequence"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_GameModeSelectMenuSequence {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GameModeSelectMenuSequence {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GameModeSelectMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GameModeSelectMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GameModeSelectMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence.Menu.DayMenuItem"
)]
#[parent(
    crate::app::mainmenusequence::MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem
)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItem {
    #[method(name = "KeyCall", args = 0)]
    pub fn key_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence.Menu.MonthMenuItemContent"
)]
# [parent (crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemContent)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItemContent {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItemContent {
    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItemContent
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItemContentMethods > :: ctor (this ,) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_FinalConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.FinalConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct MainMenuSequence_FinalConfirmDialog {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_FinalConfirmDialog {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        dialog_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_FinalConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        dialog_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_FinalConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_FinalConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
            dialog_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GrowModeSelectMenuSequence_Menu_RandomMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GrowModeSelectMenuSequence.Menu.RandomMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_GrowModeSelectMenuSequence_Menu_RandomMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_RandomMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_RandomMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GrowModeSelectMenuSequence_Menu_RandomMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GrowModeSelectMenuSequence_Menu_RandomMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.NetworkServiceSelectMenuSequence.Menu.YesMenuItem"
)]
#[parent(
    crate::app::mainmenusequence::MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItem
)]
pub struct MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_YesMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_YesMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_YesMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_YesMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_NetworkServiceSelectMenuSequence_Menu_YesMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_HistoryInfo.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.HistoryInfo")]
#[parent(crate::system::object::Object)]
pub struct MainMenuSequence_HistoryInfo {
    #[static_field]
    #[rename(name = "LayoutPrefabPath")]
    pub layout_prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_LayoutPrefab")]
    pub m_layout_prefab: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Window")]
    pub m_window:
        ::unity2::Array<crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_Window>,
    #[rename(name = "m_MenuAnimator")]
    pub m_menu_animator: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_HistoryInfo {
    #[method(name = "LoadLayoutPrefabAsync", args = 0)]
    pub fn load_layout_prefab_async() -> ();

    #[method(name = "UnloadLayoutPrefab", args = 0)]
    pub fn unload_layout_prefab() -> ();

    #[method(name = "IsLoadingLayoutPrefab", args = 0)]
    pub fn is_loading_layout_prefab() -> bool;

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "GetWindow", args = 1)]
    pub fn get_window(
        self,
        info_kind: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_InfoKind,
    ) -> crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_Window;

    #[method(name = "FadeIn", args = 0)]
    pub fn fade_in(self) -> ();

    #[method(name = "FadeOut", args = 0)]
    pub fn fade_out(self) -> ();

    #[method(name = "ShowWindow", args = 1)]
    pub fn show_window(
        self,
        info_kind: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_InfoKind,
    ) -> ();

    #[method(name = "HideWindow", args = 1)]
    pub fn hide_window(
        self,
        info_kind: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_InfoKind,
    ) -> ();

    #[method(name = "SetHistoryText", args = 2)]
    pub fn set_history_text(
        self,
        info_kind: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_InfoKind,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ClearHistoryText", args = 1)]
    pub fn clear_history_text(
        self,
        info_kind: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_InfoKind,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_HistoryInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_HistoryInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_HistoryInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu"
)]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu {
    #[static_field]
    #[rename(name = "LanguageMessTable")]
    pub language_mess_table: ::unity2::Array<crate::app::language::Language_Langs>,
    #[static_field]
    #[rename(name = "LanguageVoiceTable")]
    pub language_voice_table: ::unity2::Array<crate::app::language::Language_Voices>,
    #[rename(name = "m_LangMessIndexOld")]
    pub m_lang_mess_index_old: i32,
    #[rename(name = "m_LangVoiceIndexOld")]
    pub m_lang_voice_index_old: i32,
    #[rename(name = "m_LangMessIndex")]
    pub m_lang_mess_index: i32,
    #[rename(name = "m_LangVoiceIndex")]
    pub m_lang_voice_index: i32,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu {
    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "CreateMenuBind", args = 2)]
    pub fn create_menu_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_LanguageSettingMenuSequence_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_LanguageSettingMenuSequence_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GrowModeSelectMenuSequence.Menu.MenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItem {
    #[rename(name = "m_GrowMode")]
    pub m_grow_mode: crate::app::growmode::GrowMode,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, grow_mode: crate::app::growmode::GrowMode) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItem {
    pub fn new(grow_mode: crate::app::growmode::GrowMode) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItemMethods>::ctor(
            this, grow_mode,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_HistoryInfo_Window.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.HistoryInfo.Window")]
#[parent(crate::system::object::Object)]
pub struct MainMenuSequence_HistoryInfo_Window {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CanvasGroup")]
    pub m_canvas_group: crate::unity_engine::canvasgroup::CanvasGroup,
    #[rename(name = "m_HistoryText")]
    pub m_history_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_InfoKind")]
    pub m_info_kind: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_InfoKind,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_HistoryInfo_Window {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        root_object: crate::unity_engine::gameobject::GameObject,
        info_kind: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_InfoKind,
    ) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetHistoryText", args = 1)]
    pub fn set_history_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "ClearHistoryText", args = 0)]
    pub fn clear_history_text(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_HistoryInfo_Window {
    pub fn new(
        root_object: crate::unity_engine::gameobject::GameObject,
        info_kind: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo_InfoKind,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_HistoryInfo_Window),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_HistoryInfo_WindowMethods>::ctor(this, root_object, info_kind);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence_Menu_MenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.TopMenuSequence.Menu.MenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MainMenuSequence_TopMenuSequence_Menu_MenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence_Menu_MenuItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence_Menu_MenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence_Menu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequence_Menu_MenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_AlphaFader.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.AlphaFader")]
#[parent(crate::system::object::Object)]
pub struct MainMenuSequence_AlphaFader {
    #[rename(name = "m_alpha")]
    pub m_alpha: f32,
    #[rename(name = "m_alphaFrom")]
    pub m_alpha_from: f32,
    #[rename(name = "m_alphaTo")]
    pub m_alpha_to: f32,
    #[rename(name = "m_time")]
    pub m_time: f32,
    #[rename(name = "m_duration")]
    pub m_duration: f32,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_AlphaFader {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = "Set", args = 2)]
    pub fn set(self, rot: f32, msec: f32) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> f32;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_AlphaFader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_AlphaFader),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_AlphaFaderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence_Menu_MenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.TopMenuSequence.Menu.MenuItemContent"
)]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MainMenuSequence_TopMenuSequence_Menu_MenuItemContent {
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence_Menu_MenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item: crate::app::mainmenusequence::MainMenuSequence_TopMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence_Menu_MenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence_Menu_MenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequence_Menu_MenuItemContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.DifficultySelectMenuSequence.Menu.MenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem {
    #[rename(name = "m_Difficulty")]
    pub m_difficulty: crate::app::difficulty::Difficulty,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, difficulty: crate::app::difficulty::Difficulty) -> ();

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem {
    pub fn new(difficulty: crate::app::difficulty::Difficulty) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItemMethods>::ctor(
            this, difficulty,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_DifficultySelectMenuSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.DifficultySelectMenuSequence"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_DifficultySelectMenuSequence {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_DifficultySelectMenuSequence {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_DifficultySelectMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_DifficultySelectMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_DifficultySelectMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NetworkServiceSelectMenuSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.NetworkServiceSelectMenuSequence"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_NetworkServiceSelectMenuSequence {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NetworkServiceSelectMenuSequence {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NetworkServiceSelectMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_NetworkServiceSelectMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_NetworkServiceSelectMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence_Menu_OptionMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.TopMenuSequence.Menu.OptionMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_TopMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_TopMenuSequence_Menu_OptionMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence_Menu_OptionMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence_Menu_OptionMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence_Menu_OptionMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequence_Menu_OptionMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu.MenuItemContent"
)]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent {
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ParamText")]
    pub m_param_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = "BuildTextColor", args = 0)]
    pub fn build_text_color(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContentMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GameModeSelectMenuSequence_Menu_ClassicMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GameModeSelectMenuSequence.Menu.ClassicMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_GameModeSelectMenuSequence_Menu_ClassicMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_ClassicMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_ClassicMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_GameModeSelectMenuSequence_Menu_ClassicMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GameModeSelectMenuSequence_Menu_ClassicMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence_Menu_MenuContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.TopMenuSequence.Menu.MenuContent"
)]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MainMenuSequence_TopMenuSequence_Menu_MenuContent {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence_Menu_MenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence_Menu_MenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence_Menu_MenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequence_Menu_MenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_DifficultySelectMenuSequence_Menu_HardMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.DifficultySelectMenuSequence.Menu.HardMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_DifficultySelectMenuSequence_Menu_HardMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_HardMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_HardMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_DifficultySelectMenuSequence_Menu_HardMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_DifficultySelectMenuSequence_Menu_HardMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.DifficultySelectMenuSequence.Menu.MenuContent"
)]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuContent {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_DifficultySelectMenuSequence_Menu_MenuContentMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_FinalConfirmDialog_YesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.FinalConfirmDialog.YesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MainMenuSequence_FinalConfirmDialog_YesDialogItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_FinalConfirmDialog_YesDialogItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_FinalConfirmDialog_YesDialogItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_FinalConfirmDialog_YesDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_FinalConfirmDialog_YesDialogItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_LayoutType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_LayoutType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MainMenuSequence_PlayerBirthdayInputMenuSequence_LayoutType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MainMenuSequence.PlayerBirthdayInputMenuSequence.LayoutType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MainMenuSequence_PlayerBirthdayInputMenuSequence_LayoutType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MainMenuSequence_PlayerBirthdayInputMenuSequence_LayoutType {
    pub fn jp_ch_korea() -> Self {
        Self { value: 0 }
    }

    pub fn us_eu_etc() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_OptionMenuSequence.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.OptionMenuSequence")]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_OptionMenuSequence {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_OptionMenuSequence {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "IsTitleBarEnable", args = 0)]
    pub fn is_title_bar_enable(self) -> bool;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_OptionMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_OptionMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_OptionMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_OptionMenuSequence_Menu_CopyMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.OptionMenuSequence.Menu.CopyMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_OptionMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_OptionMenuSequence_Menu_CopyMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_OptionMenuSequence_Menu_CopyMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_OptionMenuSequence_Menu_CopyMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_OptionMenuSequence_Menu_CopyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_OptionMenuSequence_Menu_CopyMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GrowModeSelectMenuSequence.Menu.MenuItemContent"
)]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItemContent {
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_HelpText")]
    pub m_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItemContent
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItemContentMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_OptionMenuSequence_Menu.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.OptionMenuSequence.Menu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MainMenuSequence_OptionMenuSequence_Menu {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_OptionMenuSequence_Menu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateMenuBind", args = 2)]
    pub fn create_menu_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_OptionMenuSequence_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_OptionMenuSequence_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_OptionMenuSequence_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mainmenusequence :: MainMenuSequence >)]
pub struct MainMenuSequence {
    #[static_field]
    #[rename(name = "GameStartSceneName")]
    pub game_start_scene_name: ::unity2::Il2CppString,
    #[rename(name = "m_PrevSequence")]
    pub m_prev_sequence: crate::app::mainmenusequence::MainMenuSequence_Label,
    #[rename(name = "m_NowSequence")]
    pub m_now_sequence: crate::app::mainmenusequence::MainMenuSequence_Label,
    #[rename(name = "m_NextSequence")]
    pub m_next_sequence: crate::app::mainmenusequence::MainMenuSequence_Label,
    #[rename(name = "m_CharacterWorkPlayerMale")]
    pub m_character_work_player_male: crate::app::mainmenusequence::MainMenuSequence_CharacterWork,
    #[rename(name = "m_CharacterWorkPlayerFemale")]
    pub m_character_work_player_female:
        crate::app::mainmenusequence::MainMenuSequence_CharacterWork,
    #[rename(name = "m_CameraWork")]
    pub m_camera_work: crate::app::mainmenusequence::MainMenuSequence_CameraWork,
    #[rename(name = "m_HistoryInfo")]
    pub m_history_info: crate::app::mainmenusequence::MainMenuSequence_HistoryInfo,
    #[rename(name = "m_saveDataHeaderReader")]
    pub m_save_data_header_reader: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader,
    #[rename(name = "m_IsFirst")]
    pub m_is_first: bool,
    #[rename(name = "m_IsContinueEnable")]
    pub m_is_continue_enable: bool,
    #[rename(name = "m_TopMenuCursorIndex")]
    pub m_top_menu_cursor_index: i32,
    #[rename(name = "m_OptionMenuCursorIndex")]
    pub m_option_menu_cursor_index: i32,
    #[rename(name = "m_Difficulty")]
    pub m_difficulty: crate::app::difficulty::Difficulty,
    #[rename(name = "m_GameMode")]
    pub m_game_mode: crate::app::gamemode::GameMode,
    #[rename(name = "m_GrowMode")]
    pub m_grow_mode: crate::app::growmode::GrowMode,
    #[rename(name = "m_IsNetworkService")]
    pub m_is_network_service: bool,
    #[rename(name = "m_IsNetworkLoginOnce")]
    pub m_is_network_login_once: bool,
    #[rename(name = "m_PlayerGender")]
    pub m_player_gender: crate::app::gender::Gender,
    #[rename(name = "m_PlayerName")]
    pub m_player_name: ::unity2::Il2CppString,
    #[rename(name = "m_DayOfPlayerBirthday")]
    pub m_day_of_player_birthday: i32,
    #[rename(name = "m_MonthOfPlayerBirthday")]
    pub m_month_of_player_birthday: i32,
    #[rename(name = "m_IsReloadLanguageMess")]
    pub m_is_reload_language_mess: bool,
    #[rename(name = "m_ReloadLanguageMess")]
    pub m_reload_language_mess: crate::app::language::Language_Langs,
    #[rename(name = "m_IsReloadLanguageVoice")]
    pub m_is_reload_language_voice: bool,
    #[rename(name = "m_ReloadLanguageVoice")]
    pub m_reload_language_voice: crate::app::language::Language_Voices,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence {
    #[method(name = "LoadGameStartScene", args = 0)]
    pub fn load_game_start_scene(self) -> ();

    #[method(name = "UnloadGameStartScene", args = 0)]
    pub fn unload_game_start_scene(self) -> ();

    #[method(name = "LoadEmptyScene", args = 0)]
    pub fn load_empty_scene(self) -> ();

    #[method(name = "JumpToNextSequence", args = 0)]
    pub fn jump_to_next_sequence(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "BgmToStartGame", args = 0)]
    pub fn bgm_to_start_game(self) -> ();

    #[method(name = "BgmToContinueGame", args = 0)]
    pub fn bgm_to_continue_game(self) -> ();

    #[method(name = "BgmToTitleLoop", args = 0)]
    pub fn bgm_to_title_loop(self) -> ();

    #[method(name = "BgmToTitle", args = 0)]
    pub fn bgm_to_title(self) -> ();

    #[method(name = "CreateDLCNewsMessage", args = 1)]
    pub fn create_dlc_news_message(self, patch_number: i32) -> ::unity2::Il2CppString;

    #[method(name = "CreateDLCNewsDialog", args = 0)]
    pub fn create_dlc_news_dialog(self) -> ();

    #[method(name = "LoadSaveDataHeader", args = 0)]
    pub fn load_save_data_header(self) -> ();

    #[method(name = "DecideIsContinueEnable", args = 0)]
    pub fn decide_is_continue_enable(self) -> ();

    #[method(name = "CreateContinueSequence", args = 0)]
    pub fn create_continue_sequence(self) -> ();

    #[method(name = "AfterContinueSequence", args = 0)]
    pub fn after_continue_sequence(self) -> ();

    #[method(name = "CreateSaveDataCopyMenu", args = 0)]
    pub fn create_save_data_copy_menu(self) -> ();

    #[method(name = "CreateSaveDataDeleteMenu", args = 0)]
    pub fn create_save_data_delete_menu(self) -> ();

    #[method(name = "ReloadLanguage", args = 0)]
    pub fn reload_language(self) -> ();

    #[method(name = "StartDLCShop", args = 0)]
    pub fn start_dlc_shop(self) -> ();

    #[method(name = "SetToTitleLoop", args = 0)]
    pub fn set_to_title_loop(self) -> ();

    #[method(name = "GetProcDesc", args = 0)]
    pub fn get_proc_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "IsGrowModeSelectEnable", args = 0)]
    pub fn is_grow_mode_select_enable(self) -> bool;

    #[method(name = "IsGrowModeRandomInLunaticEnable", args = 0)]
    pub fn is_grow_mode_random_in_lunatic_enable(self) -> bool;

    #[method(name = "GetBirthdayString", args = 2)]
    pub fn get_birthday_string(month: i32, day: i32) -> ::unity2::Il2CppString;

    #[method(name = "InitGameStart", args = 0)]
    pub fn init_game_start(self) -> ();

    #[method(name = "ExecuteGameStart", args = 0)]
    pub fn execute_game_start(self) -> ();

    #[method(name = "NetworkLogin", args = 0)]
    pub fn network_login(self) -> ();

    #[method(name = "CreateCharacter", args = 1)]
    pub fn create_character(
        self,
        pid: ::unity2::Il2CppString,
    ) -> crate::app::mainmenusequence::MainMenuSequence_CharacterWork;

    #[method(name = "SetupGameStartScene", args = 0)]
    pub fn setup_game_start_scene(self) -> ();

    #[method(name = "AfterSetupGameStartScene", args = 0)]
    pub fn after_setup_game_start_scene(self) -> ();

    #[method(name = "SetupTitleScene", args = 0)]
    pub fn setup_title_scene(self) -> ();

    #[method(name = "Persistent_PlayerSelect", args = 0)]
    pub fn persistent_player_select(self) -> ();

    #[method(name = "InitCameraZoomInToPlayer", args = 0)]
    pub fn init_camera_zoom_in_to_player(self) -> ();

    #[method(name = "WaitCameraZoomInToPlayer", args = 0)]
    pub fn wait_camera_zoom_in_to_player(self) -> ();

    #[method(name = "InitCameraZoomOutFromPlayer", args = 0)]
    pub fn init_camera_zoom_out_from_player(self) -> ();

    #[method(name = "WaitCameraZoomOutFromPlayer", args = 0)]
    pub fn wait_camera_zoom_out_from_player(self) -> ();

    #[method(name = "SetupHistoryInfo", args = 0)]
    pub fn setup_history_info(self) -> ();

    #[method(name = "FadeInHistoryInfo", args = 0)]
    pub fn fade_in_history_info(self) -> ();

    #[method(name = "FadeOutHistoryInfo", args = 0)]
    pub fn fade_out_history_info(self) -> ();

    #[method(name = "StartPlayerNameInput", args = 0)]
    pub fn start_player_name_input(self) -> ();

    #[method(name = "AfterPlayerNameInput", args = 0)]
    pub fn after_player_name_input(self) -> ();

    #[method(name = "PlayPlayerStartAnim", args = 0)]
    pub fn play_player_start_anim(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NameCheckDialogItemYes.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.NameCheckDialogItemYes")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MainMenuSequence_NameCheckDialogItemYes {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NameCheckDialogItemYes {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "ToRename", args = 0)]
    pub fn to_rename(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NameCheckDialogItemYes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_NameCheckDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_NameCheckDialogItemYesMethods>::ctor(this);
        this
    }

    pub fn new_2(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_NameCheckDialogItemYes),
                ::core::stringify!(new_2),
            )
        });
        <Self as IMainMenuSequence_NameCheckDialogItemYesMethods>::ctor_2(this, text);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence {
    #[method(name = "GetLayoutType", args = 0)]
    pub fn get_layout_type(
        self,
    ) -> crate::app::mainmenusequence::MainMenuSequence_PlayerBirthdayInputMenuSequence_LayoutType;

    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_PlayerBirthdayInputMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_PlayerBirthdayInputMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.DifficultySelectMenuSequence.Menu.MenuItemContent"
)]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItemContent {
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_HelpText")]
    pub m_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItemContent
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItemContentMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence_Menu_ContinueMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.TopMenuSequence.Menu.ContinueMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_TopMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_TopMenuSequence_Menu_ContinueMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence_Menu_ContinueMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence_Menu_ContinueMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence_Menu_ContinueMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequence_Menu_ContinueMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_OptionMenuSequence_Menu_DeleteMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.OptionMenuSequence.Menu.DeleteMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_OptionMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_OptionMenuSequence_Menu_DeleteMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_OptionMenuSequence_Menu_DeleteMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_OptionMenuSequence_Menu_DeleteMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_OptionMenuSequence_Menu_DeleteMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_OptionMenuSequence_Menu_DeleteMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_LanguageSettingMenuSequence {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_LanguageSettingMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_LanguageSettingMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu.MenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem {
    #[method(name = "GetParamName", args = 0)]
    pub fn get_param_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NameCheckDialogItemNo.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.NameCheckDialogItemNo")]
#[parent(crate::app::mainmenusequence::MainMenuSequence_NameCheckDialogItemYes)]
pub struct MainMenuSequence_NameCheckDialogItemNo {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NameCheckDialogItemNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NameCheckDialogItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_NameCheckDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_NameCheckDialogItemNoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MainMenuSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MainMenuSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MainMenuSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MainMenuSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MainMenuSequence_Label {
    pub fn none() -> Self {
        Self { value: -1 }
    }

    pub fn start() -> Self {
        Self { value: 0 }
    }

    pub fn dlc_news() -> Self {
        Self { value: 1 }
    }

    pub fn top_menu() -> Self {
        Self { value: 2 }
    }

    pub fn change_scene_to_game_start() -> Self {
        Self { value: 3 }
    }

    pub fn change_scene_to_title() -> Self {
        Self { value: 4 }
    }

    pub fn init_game_start() -> Self {
        Self { value: 5 }
    }

    pub fn player_gender_select() -> Self {
        Self { value: 6 }
    }

    pub fn camera_zoom_in_to_player() -> Self {
        Self { value: 7 }
    }

    pub fn camera_zoom_out_from_player() -> Self {
        Self { value: 8 }
    }

    pub fn player_name_input() -> Self {
        Self { value: 9 }
    }

    pub fn player_birthday_input() -> Self {
        Self { value: 10 }
    }

    pub fn difficulty_select() -> Self {
        Self { value: 11 }
    }

    pub fn game_mode_select() -> Self {
        Self { value: 12 }
    }

    pub fn grow_mode_select() -> Self {
        Self { value: 13 }
    }

    pub fn network_service_select() -> Self {
        Self { value: 14 }
    }

    pub fn network_login() -> Self {
        Self { value: 15 }
    }

    pub fn final_confirm() -> Self {
        Self { value: 16 }
    }

    pub fn execute_game_start() -> Self {
        Self { value: 17 }
    }

    pub fn r#continue() -> Self {
        Self { value: 18 }
    }

    pub fn option() -> Self {
        Self { value: 19 }
    }

    pub fn save_data_copy() -> Self {
        Self { value: 20 }
    }

    pub fn save_data_delete() -> Self {
        Self { value: 21 }
    }

    pub fn language_setting() -> Self {
        Self { value: 22 }
    }

    pub fn language_reload() -> Self {
        Self { value: 23 }
    }

    pub fn dlc_begin() -> Self {
        Self { value: 24 }
    }

    pub fn dlc_shop() -> Self {
        Self { value: 25 }
    }

    pub fn dlc_end() -> Self {
        Self { value: 26 }
    }

    pub fn to_title_loop() -> Self {
        Self { value: 27 }
    }

    pub fn to_start_game() -> Self {
        Self { value: 28 }
    }

    pub fn to_continue_game() -> Self {
        Self { value: 29 }
    }

    pub fn end() -> Self {
        Self { value: 30 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence_Menu_DLCMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.TopMenuSequence.Menu.DLCMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_TopMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_TopMenuSequence_Menu_DLCMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence_Menu_DLCMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence_Menu_DLCMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence_Menu_DLCMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequence_Menu_DLCMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GameModeSelectMenuSequence_Menu_CasualMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GameModeSelectMenuSequence.Menu.CasualMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_GameModeSelectMenuSequence_Menu_CasualMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_CasualMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_CasualMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GameModeSelectMenuSequence_Menu_CasualMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GameModeSelectMenuSequence_Menu_CasualMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence_Menu.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.TopMenuSequence.Menu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MainMenuSequence_TopMenuSequence_Menu {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence_Menu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateMenuBind", args = 2)]
    pub fn create_menu_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: mainmenusequence :: MainMenuSequence_TopMenuSequence_Menu_MenuContent,
    ) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequence_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_DifficultySelectMenuSequence_Menu_NormalMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.DifficultySelectMenuSequence.Menu.NormalMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_DifficultySelectMenuSequence_Menu_NormalMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_NormalMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu_NormalMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_DifficultySelectMenuSequence_Menu_NormalMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_DifficultySelectMenuSequence_Menu_NormalMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.NetworkServiceSelectMenuSequence.Menu.MenuContent"
)]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuContent {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuContent
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuContentMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GrowModeSelectMenuSequence_Menu_FixedMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GrowModeSelectMenuSequence.Menu.FixedMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_GrowModeSelectMenuSequence_Menu_FixedMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_FixedMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_FixedMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GrowModeSelectMenuSequence_Menu_FixedMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GrowModeSelectMenuSequence_Menu_FixedMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GrowModeSelectMenuSequence_Menu.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GrowModeSelectMenuSequence.Menu"
)]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MainMenuSequence_GrowModeSelectMenuSequence_Menu {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu {
    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "CreateMenuBind", args = 2)]
    pub fn create_menu_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: mainmenusequence :: MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent,
    ) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GrowModeSelectMenuSequence_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GrowModeSelectMenuSequence_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerGenderSelectMenuSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerGenderSelectMenuSequence"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_PlayerGenderSelectMenuSequence {
    #[rename(name = "m_Gender")]
    pub m_gender: crate::app::gender::Gender,
    #[rename(name = "m_MenuAnimator")]
    pub m_menu_animator: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerGenderSelectMenuSequence {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "InitLayoutObjectReference", args = 0)]
    pub fn init_layout_object_reference(self) -> ();

    #[method(name = "CloseWindow", args = 0)]
    pub fn close_window(self) -> ();

    #[method(name = "WaitCloseWindow", args = 0)]
    pub fn wait_close_window(self) -> bool;

    #[method(name = "SetGender", args = 1)]
    pub fn set_gender(self, gender: crate::app::gender::Gender) -> ();

    #[method(name = "GetProcDesc", args = 0)]
    pub fn get_proc_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerGenderSelectMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_PlayerGenderSelectMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_PlayerGenderSelectMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu.ConfirmDialog"
)]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        dialog_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        dialog_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
            dialog_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence.Menu.MenuContent"
)]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuContent {
    #[rename(name = "m_MonthUnitText")]
    pub m_month_unit_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_DayUnitText")]
    pub m_day_unit_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "CalcCursorMovedPosX", args = 1)]
    pub fn calc_cursor_moved_pos_x(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuContent
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuContentMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu_MessMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu.MessMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu_MessMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_MessMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParamName", args = 0)]
    pub fn get_param_name(self) -> ::unity2::Il2CppString;

    #[method(name = "KeyCall", args = 0)]
    pub fn key_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_MessMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_LanguageSettingMenuSequence_Menu_MessMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_LanguageSettingMenuSequence_Menu_MessMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GameModeSelectMenuSequence.Menu.MenuContent"
)]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuContent {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GameModeSelectMenuSequence_Menu_MenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence.Menu.MenuItemContent"
)]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemContent {
    #[rename(name = "m_ValueFrameObject")]
    pub m_value_frame_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ValueText")]
    pub m_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemContent {
    #[method(name = "GetValueFrameObject", args = 0)]
    pub fn get_value_frame_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemContent
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemContentMethods > :: ctor (this ,) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence.Menu.MenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem {
    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence.Menu.MonthMenuItem"
)]
#[parent(
    crate::app::mainmenusequence::MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem
)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItem {
# [rename (name = "m_DayMenuItem")] pub m_day_menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItem ,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        day_menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItem,
    ) -> ();

    #[method(name = "KeyCall", args = 0)]
    pub fn key_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItem {
    pub fn new(
        day_menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItem,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MonthMenuItemMethods>::ctor(
            this,
            day_menu_item,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu.MenuContent"
)]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "SetCursorColor", args = 1)]
    pub fn set_cursor_color(self, c: crate::unity_engine::color::Color) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence.Menu"
)]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu {
    #[static_field]
    #[rename(name = "MonthMin")]
    pub month_min: i32,
    #[static_field]
    #[rename(name = "MonthMax")]
    pub month_max: i32,
    #[static_field]
    #[rename(name = "DayMin")]
    pub day_min: i32,
    #[static_field]
    #[rename(name = "DayMax")]
    pub day_max: ::unity2::Array<i32>,
    #[rename(name = "m_Month")]
    pub m_month: i32,
    #[rename(name = "m_Day")]
    pub m_day: i32,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu {
    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "CreateMenuBind", args = 2)]
    pub fn create_menu_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuContent,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_PlayerBirthdayInputMenuSequence_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu.VoiceMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParamName", args = 0)]
    pub fn get_param_name(self) -> ::unity2::Il2CppString;

    #[method(name = "KeyCall", args = 0)]
    pub fn key_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_FinalConfirmDialog_NoDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.FinalConfirmDialog.NoDialogItem"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct MainMenuSequence_FinalConfirmDialog_NoDialogItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_FinalConfirmDialog_NoDialogItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_FinalConfirmDialog_NoDialogItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_FinalConfirmDialog_NoDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_FinalConfirmDialog_NoDialogItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.PlayerBirthdayInputMenuSequence.Menu.DayMenuItemContent"
)]
# [parent (crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItemContent)]
pub struct MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItemContent {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItemContent {
    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItemContent
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMainMenuSequence_PlayerBirthdayInputMenuSequence_Menu_DayMenuItemContentMethods > :: ctor (this ,) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.TopMenuSequence")]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_TopMenuSequence {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "IsTitleBarEnable", args = 0)]
    pub fn is_title_bar_enable(self) -> bool;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.NetworkServiceSelectMenuSequence.Menu.MenuItemContent"
)]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItemContent {
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItemContent
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItemContentMethods > :: ctor (this ,) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_DifficultySelectMenuSequence_Menu.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.DifficultySelectMenuSequence.Menu"
)]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MainMenuSequence_DifficultySelectMenuSequence_Menu {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu {
    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "CreateMenuBind", args = 2)]
    pub fn create_menu_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: mainmenusequence :: MainMenuSequence_DifficultySelectMenuSequence_Menu_MenuContent,
    ) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_DifficultySelectMenuSequence_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_DifficultySelectMenuSequence_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_DifficultySelectMenuSequence_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_OptionMenuSequence_Menu_MenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.OptionMenuSequence.Menu.MenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_TopMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_OptionMenuSequence_Menu_MenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_OptionMenuSequence_Menu_MenuItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_OptionMenuSequence_Menu_MenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_OptionMenuSequence_Menu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_OptionMenuSequence_Menu_MenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_OptionMenuSequence_Menu_LanguageSettingMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.OptionMenuSequence.Menu.LanguageSettingMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_OptionMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_OptionMenuSequence_Menu_LanguageSettingMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_OptionMenuSequence_Menu_LanguageSettingMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_OptionMenuSequence_Menu_LanguageSettingMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_OptionMenuSequence_Menu_LanguageSettingMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_OptionMenuSequence_Menu_LanguageSettingMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_YesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.LanguageSettingMenuSequence.Menu.ConfirmDialog.YesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_YesDialogItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_YesDialogItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_YesDialogItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_YesDialogItem
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IMainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog_YesDialogItemMethods > :: ctor (this ,) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_NoMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.NetworkServiceSelectMenuSequence.Menu.NoMenuItem"
)]
#[parent(
    crate::app::mainmenusequence::MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItem
)]
pub struct MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_NoMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_NoMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_NoMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_NoMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_NetworkServiceSelectMenuSequence_Menu_NoMenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_CharacterWork.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.CharacterWork")]
#[parent(crate::system::object::Object)]
pub struct MainMenuSequence_CharacterWork {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_CharacterWork {
    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Character", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "set_Character", args = 1)]
    pub fn set_character(self, value: crate::combat::character::Character) -> ();

    #[method(name = "get_Appearance", args = 0)]
    pub fn get_appearance(self) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "set_Appearance", args = 1)]
    pub fn set_appearance(
        self,
        value: crate::combat::characterappearance::CharacterAppearance,
    ) -> ();

    #[method(name = "get_RootAnimator", args = 0)]
    pub fn get_root_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "set_RootAnimator", args = 1)]
    pub fn set_root_animator(self, value: crate::unity_engine::animator::Animator) -> ();

    #[method(name = "get_LookAtController", args = 0)]
    pub fn get_look_at_controller(
        self,
    ) -> crate::app::eventcharacterlookatcontroller::EventCharacterLookAtController;

    #[method(name = "set_LookAtController", args = 1)]
    pub fn set_look_at_controller(
        self,
        value: crate::app::eventcharacterlookatcontroller::EventCharacterLookAtController,
    ) -> ();

    #[method(name = "get_LookAtTarget", args = 0)]
    pub fn get_look_at_target(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_LookAtTarget", args = 1)]
    pub fn set_look_at_target(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_AlphaFader", args = 0)]
    pub fn get_alpha_fader(self) -> crate::app::mainmenusequence::MainMenuSequence_AlphaFader;

    #[method(name = "set_AlphaFader", args = 1)]
    pub fn set_alpha_fader(
        self,
        value: crate::app::mainmenusequence::MainMenuSequence_AlphaFader,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        pid: ::unity2::Il2CppString,
        appearance: crate::combat::characterappearance::CharacterAppearance,
    ) -> ();

    #[method(name = "SetupByCharacter", args = 1)]
    pub fn setup_by_character(self, character: crate::combat::character::Character) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "SetPosByLocatorName", args = 1)]
    pub fn set_pos_by_locator_name(self, pos_locator_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayBodyAnim", args = 2)]
    pub fn play_body_anim(
        self,
        body_anim_name: ::unity2::Il2CppString,
        transition_duration: f32,
    ) -> ();

    #[method(name = "PlayFacialAnim", args = 1)]
    pub fn play_facial_anim(self, facial_anim_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetLookAt", args = 1)]
    pub fn set_look_at(self, target: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "FadeIn", args = 0)]
    pub fn fade_in(self) -> ();

    #[method(name = "FadeOut", args = 0)]
    pub fn fade_out(self) -> ();

    #[method(name = "ToDefault", args = 0)]
    pub fn to_default(self) -> ();

    #[method(name = "ToActive", args = 0)]
    pub fn to_active(self) -> ();

    #[method(name = "ToInactive", args = 0)]
    pub fn to_inactive(self) -> ();

    #[method(name = "ToDecide", args = 0)]
    pub fn to_decide(self) -> ();

    #[method(name = "ToNotDecide", args = 0)]
    pub fn to_not_decide(self) -> ();

    #[method(name = "ToGameStart", args = 0)]
    pub fn to_game_start(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_CharacterWork {
    pub fn new(
        pid: ::unity2::Il2CppString,
        appearance: crate::combat::characterappearance::CharacterAppearance,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_CharacterWork),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_CharacterWorkMethods>::ctor(this, pid, appearance);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.NetworkServiceSelectMenuSequence.Menu.MenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuItemMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GrowModeSelectMenuSequence.Menu.MenuContent"
)]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_TopMenuSequence_Menu_StartMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.TopMenuSequence.Menu.StartMenuItem"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_TopMenuSequence_Menu_MenuItem)]
pub struct MainMenuSequence_TopMenuSequence_Menu_StartMenuItem {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_TopMenuSequence_Menu_StartMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_TopMenuSequence_Menu_StartMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_TopMenuSequence_Menu_StartMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_TopMenuSequence_Menu_StartMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_NetworkServiceSelectMenuSequence_Menu.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.NetworkServiceSelectMenuSequence.Menu"
)]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MainMenuSequence_NetworkServiceSelectMenuSequence_Menu {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu {
    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "ReturnSequence", args = 0)]
    pub fn return_sequence() -> ();

    #[method(name = "CreateMenuBind", args = 2)]
    pub fn create_menu_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: mainmenusequence :: MainMenuSequence_NetworkServiceSelectMenuSequence_Menu_MenuContent,
    ) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_NetworkServiceSelectMenuSequence_Menu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_NetworkServiceSelectMenuSequence_Menu),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_NetworkServiceSelectMenuSequence_MenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_HistoryInfo_InfoKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MainMenuSequence_HistoryInfo_InfoKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MainMenuSequence_HistoryInfo_InfoKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MainMenuSequence.HistoryInfo.InfoKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MainMenuSequence_HistoryInfo_InfoKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MainMenuSequence_HistoryInfo_InfoKind {
    pub fn player_name() -> Self {
        Self { value: 0 }
    }

    pub fn birthday() -> Self {
        Self { value: 1 }
    }

    pub fn difficulty() -> Self {
        Self { value: 2 }
    }

    pub fn grow_mode() -> Self {
        Self { value: 3 }
    }

    pub fn network_service() -> Self {
        Self { value: 4 }
    }

    pub fn game_mode() -> Self {
        Self { value: 5 }
    }

    pub fn max() -> Self {
        Self { value: 6 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GrowModeSelectMenuSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GrowModeSelectMenuSequence"
)]
#[parent(crate::app::mainmenusequence::MainMenuSequence_MenuSequenceBase)]
pub struct MainMenuSequence_GrowModeSelectMenuSequence {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GrowModeSelectMenuSequence {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GrowModeSelectMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_GrowModeSelectMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GrowModeSelectMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_CameraWork.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.CameraWork")]
#[parent(crate::system::object::Object)]
pub struct MainMenuSequence_CameraWork {}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_CameraWork {
    #[method(name = "get_Camera", args = 0)]
    pub fn get_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "set_Camera", args = 1)]
    pub fn set_camera(self, value: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "get_Animator", args = 0)]
    pub fn get_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "set_Animator", args = 1)]
    pub fn set_animator(self, value: crate::unity_engine::animator::Animator) -> ();

    #[method(name = "get_LookAtTarget", args = 0)]
    pub fn get_look_at_target(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_LookAtTarget", args = 1)]
    pub fn set_look_at_target(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        camera: crate::unity_engine::camera::Camera,
        animator: crate::unity_engine::animator::Animator,
        look_at_target: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "IsPlayingAnim", args = 1)]
    pub fn is_playing_anim(self, anim_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "PlayAnim", args = 2)]
    pub fn play_anim(self, anim_name: ::unity2::Il2CppString, duration_msec: i32) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_CameraWork {
    pub fn new(
        camera: crate::unity_engine::camera::Camera,
        animator: crate::unity_engine::animator::Animator,
        look_at_target: crate::unity_engine::gameobject::GameObject,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_CameraWork),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_CameraWorkMethods>::ctor(this, camera, animator, look_at_target);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItemContent.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MainMenuSequence.GameModeSelectMenuSequence.Menu.MenuItemContent"
)]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItemContent {
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_HelpText")]
    pub m_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        menu_item : crate :: app :: mainmenusequence :: MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItem,
    ) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    MainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItemContent
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_GameModeSelectMenuSequence_Menu_MenuItemContentMethods>::ctor(
            this,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainmenusequence/MainMenuSequence_MenuSequenceBase.md")))]
#[::unity2::class(namespace = "App", name = "MainMenuSequence.MenuSequenceBase")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MainMenuSequence_MenuSequenceBase {
    #[rename(name = "m_LayoutPrefab")]
    pub m_layout_prefab: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-mainmenusequence")]
#[::unity2::methods]
impl MainMenuSequence_MenuSequenceBase {
    #[method(name = "GetLayoutPrefabPath", args = 0)]
    pub fn get_layout_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "IsTitleBarEnable", args = 0)]
    pub fn is_title_bar_enable(self) -> bool;

    #[method(name = "GetTitleBarName", args = 0)]
    pub fn get_title_bar_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarHelp", args = 0)]
    pub fn get_title_bar_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleBarKeyHelpId", args = 0)]
    pub fn get_title_bar_key_help_id(self) -> ::unity2::Il2CppString;

    #[method(name = "LoadLayoutPrefabAsync", args = 0)]
    pub fn load_layout_prefab_async(self) -> ();

    #[method(name = "UnloadLayoutPrefab", args = 0)]
    pub fn unload_layout_prefab(self) -> ();

    #[method(name = "IsLoadingLayoutPrefab", args = 0)]
    pub fn is_loading_layout_prefab(self) -> bool;

    #[method(name = "CreateLayoutPrefab", args = 0)]
    pub fn create_layout_prefab(self) -> ();

    #[method(name = "DestroyLayoutPrefab", args = 0)]
    pub fn destroy_layout_prefab(self) -> ();

    #[method(name = "SetTitleBarValue", args = 0)]
    pub fn set_title_bar_value(self) -> ();

    #[method(name = "OpenTitleBar", args = 0)]
    pub fn open_title_bar(self) -> ();

    #[method(name = "CreateMenuBind", args = 1)]
    pub fn create_menu_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetProcDesc", args = 0)]
    pub fn get_proc_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mainmenusequence")]
impl MainMenuSequence_MenuSequenceBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainMenuSequence_MenuSequenceBase),
                ::core::stringify!(new),
            )
        });
        <Self as IMainMenuSequence_MenuSequenceBaseMethods>::ctor(this);
        this
    }
}
