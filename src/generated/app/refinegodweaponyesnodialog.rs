
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponyesnodialog/RefineGodWeaponYesNoDialog.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponYesNoDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct RefineGodWeaponYesNoDialog {}

#[cfg(feature = "app-refinegodweaponyesnodialog")]
#[::unity2::methods]
impl RefineGodWeaponYesNoDialog {
    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        god_weapon_refine_data: crate::app::godweaponrefinedata::GodWeaponRefineData,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        current_level: i32,
        yes_event_handler : crate :: app :: refinegodweaponyesnodialog :: RefineGodWeaponYesNoDialog_YesEventHandler,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        god_weapon_refine_data: crate::app::godweaponrefinedata::GodWeaponRefineData,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        yes_event_handler : crate :: app :: refinegodweaponyesnodialog :: RefineGodWeaponYesNoDialog_YesEventHandler,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = "CreateBindForChangeSkill", args = 6)]
    pub fn create_bind_for_change_skill(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        god_weapon_refine_data: crate::app::godweaponrefinedata::GodWeaponRefineData,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        yes_event_handler : crate :: app :: refinegodweaponyesnodialog :: RefineGodWeaponYesNoDialog_YesEventHandler,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();
}

#[cfg(feature = "app-refinegodweaponyesnodialog")]
impl RefineGodWeaponYesNoDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponYesNoDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponYesNoDialogMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponyesnodialog/RefineGodWeaponYesNoDialog_YesEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponYesNoDialog.YesEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineGodWeaponYesNoDialog_YesEventHandler {}

#[cfg(feature = "app-refinegodweaponyesnodialog")]
#[::unity2::methods]
impl RefineGodWeaponYesNoDialog_YesEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refinegodweaponyesnodialog")]
impl RefineGodWeaponYesNoDialog_YesEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponYesNoDialog_YesEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponYesNoDialog_YesEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponyesnodialog/RefineGodWeaponYesNoDialog_YesMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponYesNoDialog.YesMenuItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct RefineGodWeaponYesNoDialog_YesMenuItem {
    #[rename(name = "m_YesEventHandler")]
    pub m_yes_event_handler:
        crate::app::refinegodweaponyesnodialog::RefineGodWeaponYesNoDialog_YesEventHandler,
}

#[cfg(feature = "app-refinegodweaponyesnodialog")]
#[::unity2::methods]
impl RefineGodWeaponYesNoDialog_YesMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        message: ::unity2::Il2CppString,
        yes_event_handler : crate :: app :: refinegodweaponyesnodialog :: RefineGodWeaponYesNoDialog_YesEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refinegodweaponyesnodialog")]
impl RefineGodWeaponYesNoDialog_YesMenuItem {
    pub fn new(
        message: ::unity2::Il2CppString,
        yes_event_handler : crate :: app :: refinegodweaponyesnodialog :: RefineGodWeaponYesNoDialog_YesEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponYesNoDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponYesNoDialog_YesMenuItemMethods>::ctor(
            this,
            message,
            yes_event_handler,
        );
        this
    }
}
