
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
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::yesnodialog::IYesNoDialog;
use crate::app::yesnodialog::YesNoDialog;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_MenuItem_UnitIconInfo.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu.MenuItem.UnitIconInfo")]
#[parent(crate::system::object::Object)]
pub struct RewindMenu_MenuItem_UnitIconInfo {}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_MenuItem_UnitIconInfo {
    #[method(name = "get_Person", args = 0)]
    pub fn get_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "set_Person", args = 1)]
    pub fn set_person(self, value: crate::app::persondata::PersonData) -> ();

    #[method(name = "get_Job", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "set_Job", args = 1)]
    pub fn set_job(self, value: crate::app::jobdata::JobData) -> ();

    #[method(name = "get_IsFemale", args = 0)]
    pub fn get_is_female(self) -> bool;

    #[method(name = "set_IsFemale", args = 1)]
    pub fn set_is_female(self, value: bool) -> ();

    #[method(name = "get_ItemKind", args = 0)]
    pub fn get_item_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "set_ItemKind", args = 1)]
    pub fn set_item_kind(self, value: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "get_IsEngage", args = 0)]
    pub fn get_is_engage(self) -> bool;

    #[method(name = "set_IsEngage", args = 1)]
    pub fn set_is_engage(self, value: bool) -> ();

    #[method(name = "get_God", args = 0)]
    pub fn get_god(self) -> crate::app::goddata::GodData;

    #[method(name = "set_God", args = 1)]
    pub fn set_god(self, value: crate::app::goddata::GodData) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        rewind_log_unit_icon: crate::app::maphistory::MapHistory_RewindLog_UnitIcon,
    ) -> ();
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_MenuItem_UnitIconInfo {
    pub fn new(
        rewind_log_unit_icon: crate::app::maphistory::MapHistory_RewindLog_UnitIcon,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_MenuItem_UnitIconInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_MenuItem_UnitIconInfoMethods>::ctor(this, rewind_log_unit_icon);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_ExecuteConfirmDialog_ItemYes.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu.ExecuteConfirmDialog.ItemYes")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct RewindMenu_ExecuteConfirmDialog_ItemYes {
    #[rename(name = "m_IsToPhaseBegin")]
    pub m_is_to_phase_begin: bool,
}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_ExecuteConfirmDialog_ItemYes {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, is_to_phase_begin: bool) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_ExecuteConfirmDialog_ItemYes {
    pub fn new(is_to_phase_begin: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_ExecuteConfirmDialog_ItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_ExecuteConfirmDialog_ItemYesMethods>::ctor(this, is_to_phase_begin);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_CancelConfirmDialog_ItemYes.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu.CancelConfirmDialog.ItemYes")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct RewindMenu_CancelConfirmDialog_ItemYes {}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_CancelConfirmDialog_ItemYes {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_CancelConfirmDialog_ItemYes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_CancelConfirmDialog_ItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_CancelConfirmDialog_ItemYesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_CancelConfirmDialog_ItemYesForChallengeMap.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RewindMenu.CancelConfirmDialog.ItemYesForChallengeMap"
)]
#[parent(crate::app::rewindmenu::RewindMenu_CancelConfirmDialog_ItemYes)]
pub struct RewindMenu_CancelConfirmDialog_ItemYesForChallengeMap {}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_CancelConfirmDialog_ItemYesForChallengeMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_CancelConfirmDialog_ItemYesForChallengeMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_CancelConfirmDialog_ItemYesForChallengeMap),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_CancelConfirmDialog_ItemYesForChallengeMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_CancelConfirmDialog_ItemNo.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu.CancelConfirmDialog.ItemNo")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct RewindMenu_CancelConfirmDialog_ItemNo {}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_CancelConfirmDialog_ItemNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_CancelConfirmDialog_ItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_CancelConfirmDialog_ItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_CancelConfirmDialog_ItemNoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_CancelConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu.CancelConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct RewindMenu_CancelConfirmDialog {}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_CancelConfirmDialog {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "CreateDialog", args = 1)]
    pub fn create_dialog(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_CancelConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_CancelConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_CancelConfirmDialogMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RewindMenu {
    #[rename(name = "m_scrollNow")]
    pub m_scroll_now: i32,
}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu {
    #[method(name = "get_CurrentActorUnit", args = 0)]
    pub fn get_current_actor_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_CurrentActorUnit", args = 1)]
    pub fn set_current_actor_unit(self, value: crate::app::unit::Unit) -> ();

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

    #[method(name = "GetForcePlayerColor", args = 0)]
    pub fn get_force_player_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetForceEnemyColor", args = 0)]
    pub fn get_force_enemy_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetForceAllyColor", args = 0)]
    pub fn get_force_ally_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "UpdateSelectAndScroll", args = 0)]
    pub fn update_select_and_scroll(self) -> ();

    #[method(name = "AdjustScrollIndex", args = 0)]
    pub fn adjust_scroll_index(self) -> ();

    #[method(name = "UpdateParts", args = 3)]
    pub fn update_parts(
        self,
        current_force_type: crate::app::force::Force_Type,
        turn: i32,
        rest_unit_num: i32,
    ) -> ();

    #[method(name = "BrightOffCurrentActorUnit", args = 0)]
    pub fn bright_off_current_actor_unit(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "CreateMenu", args = 1)]
    pub fn create_menu(
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::rewindmenu::RewindMenu;
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_ExecuteConfirmDialog_ItemNo.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu.ExecuteConfirmDialog.ItemNo")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct RewindMenu_ExecuteConfirmDialog_ItemNo {}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_ExecuteConfirmDialog_ItemNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_ExecuteConfirmDialog_ItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_ExecuteConfirmDialog_ItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_ExecuteConfirmDialog_ItemNoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_MenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu.MenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RewindMenu_MenuItem {
    #[rename(name = "m_LogIndex")]
    pub m_log_index: i32,
    #[rename(name = "m_LogText")]
    pub m_log_text: ::unity2::Il2CppString,
    #[rename(name = "m_CursorX")]
    pub m_cursor_x: i32,
    #[rename(name = "m_CursorZ")]
    pub m_cursor_z: i32,
    #[rename(name = "m_ActorMapHistoryIndex")]
    pub m_actor_map_history_index: i32,
    #[rename(name = "m_UnitIconInfo")]
    pub m_unit_icon_info: crate::app::rewindmenu::RewindMenu_MenuItem_UnitIconInfo,
    #[rename(name = "m_DieUnitIconInfo")]
    pub m_die_unit_icon_info: crate::app::rewindmenu::RewindMenu_MenuItem_UnitIconInfo,
    #[rename(name = "m_ForceType")]
    pub m_force_type: crate::app::force::Force_Type,
    #[rename(name = "m_IsPlayerPhaseBegin")]
    pub m_is_player_phase_begin: bool,
    #[rename(name = "m_prev")]
    pub m_prev: crate::app::rewindmenu::RewindMenu_MenuItem,
    #[rename(name = "m_next")]
    pub m_next: crate::app::rewindmenu::RewindMenu_MenuItem,
}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_MenuItem {
    #[method(name = ".ctor", args = 9)]
    pub fn ctor(
        self,
        log_index: i32,
        log_text: ::unity2::Il2CppString,
        cursor_x: i32,
        cursor_z: i32,
        actor_map_history_index: i32,
        unit_icon_info: crate::app::rewindmenu::RewindMenu_MenuItem_UnitIconInfo,
        die_unit_icon_info: crate::app::rewindmenu::RewindMenu_MenuItem_UnitIconInfo,
        force_type: crate::app::force::Force_Type,
        is_player_phase_begin: bool,
    ) -> ();

    #[method(name = "SetLink", args = 2)]
    pub fn set_link(
        self,
        prev: crate::app::rewindmenu::RewindMenu_MenuItem,
        next: crate::app::rewindmenu::RewindMenu_MenuItem,
    ) -> ();

    #[method(name = "GetPrev", args = 0)]
    pub fn get_prev(self) -> crate::app::rewindmenu::RewindMenu_MenuItem;

    #[method(name = "GetNext", args = 0)]
    pub fn get_next(self) -> crate::app::rewindmenu::RewindMenu_MenuItem;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetWidth", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "GetHeight", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetUnitIconInfo", args = 0)]
    pub fn get_unit_icon_info(self) -> crate::app::rewindmenu::RewindMenu_MenuItem_UnitIconInfo;

    #[method(name = "GetDieUnitIconInfo", args = 0)]
    pub fn get_die_unit_icon_info(self)
        -> crate::app::rewindmenu::RewindMenu_MenuItem_UnitIconInfo;

    #[method(name = "GetForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "IsPlayerPhaseBegin", args = 0)]
    pub fn is_player_phase_begin(self) -> bool;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_MenuItem {
    pub fn new(
        log_index: i32,
        log_text: ::unity2::Il2CppString,
        cursor_x: i32,
        cursor_z: i32,
        actor_map_history_index: i32,
        unit_icon_info: crate::app::rewindmenu::RewindMenu_MenuItem_UnitIconInfo,
        die_unit_icon_info: crate::app::rewindmenu::RewindMenu_MenuItem_UnitIconInfo,
        force_type: crate::app::force::Force_Type,
        is_player_phase_begin: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_MenuItemMethods>::ctor(
            this,
            log_index,
            log_text,
            cursor_x,
            cursor_z,
            actor_map_history_index,
            unit_icon_info,
            die_unit_icon_info,
            force_type,
            is_player_phase_begin,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenu/RewindMenu_ExecuteConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenu.ExecuteConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct RewindMenu_ExecuteConfirmDialog {}

#[cfg(feature = "app-rewindmenu")]
#[::unity2::methods]
impl RewindMenu_ExecuteConfirmDialog {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "CreateDialog", args = 2)]
    pub fn create_dialog(super_: crate::app::procinst::ProcInst, is_to_phase_begin: bool) -> ();
}

#[cfg(feature = "app-rewindmenu")]
impl RewindMenu_ExecuteConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenu_ExecuteConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenu_ExecuteConfirmDialogMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
