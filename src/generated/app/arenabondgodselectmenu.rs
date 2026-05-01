
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
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondGodSelectMenu.ConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct ArenaBondGodSelectMenu_ConfirmDialog {}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu_ConfirmDialog {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        decide_action: crate::system::action::Action,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenu_ConfirmDialogMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu_ChangeUnitEventHandle.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondGodSelectMenu.ChangeUnitEventHandle"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondGodSelectMenu_ChangeUnitEventHandle {}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu_ChangeUnitEventHandle {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu_ChangeUnitEventHandle {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu_ChangeUnitEventHandle),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenu_ChangeUnitEventHandleMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu_ArenaBondEmblemSelectMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondGodSelectMenu.ArenaBondEmblemSelectMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ArenaBondGodSelectMenu_ArenaBondEmblemSelectMenuItem {
    #[rename(name = "Unit")]
    pub unit: crate::app::unit::Unit,
    #[rename(name = "BondLv")]
    pub bond_lv: i32,
    #[rename(name = "MaxBondLv")]
    pub max_bond_lv: i32,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_DecideEventHandler,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_SelectEventHandler,
    #[rename(name = "m_ChangeUnitToPrevEventHandler")]
    pub m_change_unit_to_prev_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_ChangeUnitEventHandle,
    #[rename(name = "m_ChangeUnitToNextEventHandler")]
    pub m_change_unit_to_next_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_ChangeUnitEventHandle,
    #[rename(name = "m_StartHelpEventHandler")]
    pub m_start_help_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_StartHelpEventHandler,
}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu_ArenaBondEmblemSelectMenuItem {
    #[method(name = "get_God", args = 0)]
    pub fn get_god(self) -> crate::app::godunit::GodUnit;

    #[method(name = "set_God", args = 1)]
    pub fn set_god(self, value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_ChangeType", args = 0)]
    pub fn get_change_type(self) -> crate::app::ringcleaningsequence::RingCleaningSequence_GodType;

    #[method(name = "set_ChangeType", args = 1)]
    pub fn set_change_type(
        self,
        value: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "get_IsSelectable", args = 0)]
    pub fn get_is_selectable(self) -> bool;

    #[method(name = "set_IsSelectable", args = 1)]
    pub fn set_is_selectable(self, value: bool) -> ();

    #[method(name = "get_IsTalkable", args = 0)]
    pub fn get_is_talkable(self) -> bool;

    #[method(name = "set_IsTalkable", args = 1)]
    pub fn set_is_talkable(self, value: bool) -> ();

    #[method(name = ".ctor", args = 8)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        start_help_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_StartHelpEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu_ArenaBondEmblemSelectMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        start_help_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_StartHelpEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu_ArenaBondEmblemSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenu_ArenaBondEmblemSelectMenuItemMethods>::ctor(
            this,
            unit,
            god,
            r#type,
            decide_event_handler,
            select_event_handler,
            change_unit_to_prev_event_handler,
            change_unit_to_next_event_handler,
            start_help_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondGodSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ArenaBondGodSelectMenu {
    #[rename(name = "m_Root")]
    pub m_root: crate::app::arenabondgodselectroot::ArenaBondGodSelectRoot,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_DecideEventHandler,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_SelectEventHandler,
    #[rename(name = "m_ChangeUnitToPrevEventHandler")]
    pub m_change_unit_to_prev_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_ChangeUnitEventHandle,
    #[rename(name = "m_ChangeUnitToNextEventHandler")]
    pub m_change_unit_to_next_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_ChangeUnitEventHandle,
    #[rename(name = "m_StartHelpEventHandler")]
    pub m_start_help_event_handler:
        crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu_StartHelpEventHandler,
}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu {
    #[method(name = "CreateBind", args = 11)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::arenabondgodselectmenucontent::ArenaBondGodSelectMenuContent,
        root: crate::app::arenabondgodselectroot::ArenaBondGodSelectRoot,
        selected_unit: crate::app::unit::Unit,
        default_god: crate::app::godunit::GodUnit,
        default_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        start_help_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_StartHelpEventHandler,
    ) -> crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu;

    #[method(name = ".ctor", args = 8)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenabondgodselectmenucontent::ArenaBondGodSelectMenuContent,
        root: crate::app::arenabondgodselectroot::ArenaBondGodSelectRoot,
        decide_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        start_help_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_StartHelpEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "RebuildMenu", args = 3)]
    pub fn rebuild_menu(
        self,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "SetActiveForReliance", args = 4)]
    pub fn set_active_for_reliance(
        self,
        is_active: bool,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "CreateMenuItem", args = 9)]
    pub fn create_menu_item(
        unit: crate::app::unit::Unit,
        default_god: crate::app::godunit::GodUnit,
        default_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        default_index: i32,
        decide_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        start_help_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_StartHelpEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenabondgodselectmenucontent::ArenaBondGodSelectMenuContent,
        root: crate::app::arenabondgodselectroot::ArenaBondGodSelectRoot,
        decide_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_ChangeUnitEventHandle,
        start_help_event_handler : crate :: app :: arenabondgodselectmenu :: ArenaBondGodSelectMenu_StartHelpEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            root,
            decide_event_handler,
            select_event_handler,
            change_unit_to_prev_event_handler,
            change_unit_to_next_event_handler,
            start_help_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemNo.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondGodSelectMenu.ConfirmDialog.ConfirmDialogItemNo"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemNo {}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemNoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondGodSelectMenu.ConfirmDialog.ConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemYes {
    #[rename(name = "m_DecideAction")]
    pub m_decide_action: crate::system::action::Action,
}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemYes {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, decide_action: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemYes {
    pub fn new(decide_action: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenu_ConfirmDialog_ConfirmDialogItemYesMethods>::ctor(
            this,
            decide_action,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondGodSelectMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondGodSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu_StartHelpEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondGodSelectMenu.StartHelpEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondGodSelectMenu_StartHelpEventHandler {}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu_StartHelpEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        super_: crate::app::procinst::ProcInst,
        god: crate::app::goddata::GodData,
        bond_lv: i32,
    ) -> ();
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu_StartHelpEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu_StartHelpEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenu_StartHelpEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectmenu/ArenaBondGodSelectMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondGodSelectMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondGodSelectMenu_SelectEventHandler {}

#[cfg(feature = "app-arenabondgodselectmenu")]
#[::unity2::methods]
impl ArenaBondGodSelectMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();
}

#[cfg(feature = "app-arenabondgodselectmenu")]
impl ArenaBondGodSelectMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
