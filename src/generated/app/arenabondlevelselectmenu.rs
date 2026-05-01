
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenu/ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemNo.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondLevelSelectMenu.ConfirmDialog.ConfirmDialogItemNo"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemNo {}

#[cfg(feature = "app-arenabondlevelselectmenu")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
impl ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemNoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenu/ArenaBondLevelSelectMenu_ArenaBondLevelSelectMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondLevelSelectMenu.ArenaBondLevelSelectMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ArenaBondLevelSelectMenu_ArenaBondLevelSelectMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_DecideEventHandler,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_SelectEventHandler,
    #[rename(name = "m_ChangeUnitToPrevEventHandler")]
    pub m_change_unit_to_prev_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_ChangeGodEventHandle,
    #[rename(name = "m_ChangeUnitToNextEventHandler")]
    pub m_change_unit_to_next_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_ChangeGodEventHandle,
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenu_ArenaBondLevelSelectMenuItem {
    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_God", args = 0)]
    pub fn get_god(self) -> crate::app::godunit::GodUnit;

    #[method(name = "set_God", args = 1)]
    pub fn set_god(self, value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_GodType", args = 0)]
    pub fn get_god_type(self) -> crate::app::ringcleaningsequence::RingCleaningSequence_GodType;

    #[method(name = "set_GodType", args = 1)]
    pub fn set_god_type(
        self,
        value: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "get_LvBefore", args = 0)]
    pub fn get_lv_before(self) -> i32;

    #[method(name = "set_LvBefore", args = 1)]
    pub fn set_lv_before(self, value: i32) -> ();

    #[method(name = "get_LvAfter", args = 0)]
    pub fn get_lv_after(self) -> i32;

    #[method(name = "set_LvAfter", args = 1)]
    pub fn set_lv_after(self, value: i32) -> ();

    #[method(name = "get_GetExp", args = 0)]
    pub fn get_get_exp(self) -> i32;

    #[method(name = "set_GetExp", args = 1)]
    pub fn set_get_exp(self, value: i32) -> ();

    #[method(name = "get_UseCount", args = 0)]
    pub fn get_use_count(self) -> i32;

    #[method(name = "set_UseCount", args = 1)]
    pub fn set_use_count(self, value: i32) -> ();

    #[method(name = "get_IsTalkCap", args = 0)]
    pub fn get_is_talk_cap(self) -> bool;

    #[method(name = "set_IsTalkCap", args = 1)]
    pub fn set_is_talk_cap(self, value: bool) -> ();

    #[method(name = "get_IsNoBond", args = 0)]
    pub fn get_is_no_bond(self) -> bool;

    #[method(name = "set_IsNoBond", args = 1)]
    pub fn set_is_no_bond(self, value: bool) -> ();

    #[method(name = ".ctor", args = 11)]
    pub fn ctor(
        self,
        from_level: i32,
        to_level: i32,
        get_exp: i32,
        use_count: i32,
        train_unit: crate::app::unit::Unit,
        train_god: crate::app::godunit::GodUnit,
        train_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
impl ArenaBondLevelSelectMenu_ArenaBondLevelSelectMenuItem {
    pub fn new(
        from_level: i32,
        to_level: i32,
        get_exp: i32,
        use_count: i32,
        train_unit: crate::app::unit::Unit,
        train_god: crate::app::godunit::GodUnit,
        train_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenu_ArenaBondLevelSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenu_ArenaBondLevelSelectMenuItemMethods>::ctor(
            this,
            from_level,
            to_level,
            get_exp,
            use_count,
            train_unit,
            train_god,
            train_type,
            decide_event_handler,
            select_event_handler,
            change_unit_to_prev_event_handler,
            change_unit_to_next_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenu/ArenaBondLevelSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondLevelSelectMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondLevelSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-arenabondlevelselectmenu")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 5)]
    pub fn invoke(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        start: bool,
        get_exp: i32,
        use_count: i32,
    ) -> ();
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
impl ArenaBondLevelSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenu/ArenaBondLevelSelectMenu_ChangeGodEventHandle.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondLevelSelectMenu.ChangeGodEventHandle"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondLevelSelectMenu_ChangeGodEventHandle {}

#[cfg(feature = "app-arenabondlevelselectmenu")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenu_ChangeGodEventHandle {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
impl ArenaBondLevelSelectMenu_ChangeGodEventHandle {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenu_ChangeGodEventHandle),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenu_ChangeGodEventHandleMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenu/ArenaBondLevelSelectMenu_SelectEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondLevelSelectMenu.SelectEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondLevelSelectMenu_SelectEventHandler {}

#[cfg(feature = "app-arenabondlevelselectmenu")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(self, god: crate::app::godunit::GodUnit, from_lv: i32, to_lv: i32) -> ();
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
impl ArenaBondLevelSelectMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenu/ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ArenaBondLevelSelectMenu.ConfirmDialog.ConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemYes {
    #[rename(name = "m_DecideAction")]
    pub m_decide_action: crate::system::action::Action,
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemYes {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, decide_action: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
impl ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemYes {
    pub fn new(decide_action: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenu_ConfirmDialog_ConfirmDialogItemYesMethods>::ctor(
            this,
            decide_action,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenu/ArenaBondLevelSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondLevelSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ArenaBondLevelSelectMenu {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_DecideEventHandler,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_SelectEventHandler,
    #[rename(name = "m_ChangeGodToPrevEventHandler")]
    pub m_change_god_to_prev_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_ChangeGodEventHandle,
    #[rename(name = "m_ChangeGodToNextEventHandler")]
    pub m_change_god_to_next_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_ChangeGodEventHandle,
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenu {
    #[method(name = "CreateBind", args = 9)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::arenabondlevelselectmenucontent::ArenaBondLevelSelectMenuContent,
        selected_unit: crate::app::unit::Unit,
        selected_god: crate::app::godunit::GodUnit,
        selected_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_SelectEventHandler,
        change_god_to_prev_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
        change_god_to_next_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
    ) -> crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenabondlevelselectmenucontent::ArenaBondLevelSelectMenuContent,
        decide_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_SelectEventHandler,
        change_god_to_prev_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
        change_god_to_next_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
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

    #[method(name = "CreateMenuItem", args = 7)]
    pub fn create_menu_item(
        selected_unit: crate::app::unit::Unit,
        selected_god: crate::app::godunit::GodUnit,
        selected_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
        change_unit_to_next_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
impl ArenaBondLevelSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenabondlevelselectmenucontent::ArenaBondLevelSelectMenuContent,
        decide_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_SelectEventHandler,
        change_god_to_prev_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
        change_god_to_next_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_ChangeGodEventHandle,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            decide_event_handler,
            select_event_handler,
            change_god_to_prev_event_handler,
            change_god_to_next_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenu/ArenaBondLevelSelectMenu_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondLevelSelectMenu.ConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct ArenaBondLevelSelectMenu_ConfirmDialog {
    #[rename(name = "m_DecideAction")]
    pub m_decide_action: crate::system::action::Action,
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenu_ConfirmDialog {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        decide_action: crate::system::action::Action,
    ) -> ();

    #[method(name = "CreateBind", args = 8)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        god_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        from_lv: i32,
        to_lv: i32,
        use_count: i32,
        decide_action: crate::system::action::Action,
    ) -> ();
}

#[cfg(feature = "app-arenabondlevelselectmenu")]
impl ArenaBondLevelSelectMenu_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        decide_action: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenu_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenu_ConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
            decide_action,
        );
        this
    }
}
