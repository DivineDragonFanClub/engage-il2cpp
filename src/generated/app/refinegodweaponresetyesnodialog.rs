
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponresetyesnodialog/RefineGodWeaponResetYesNoDialog.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponResetYesNoDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct RefineGodWeaponResetYesNoDialog {}

#[cfg(feature = "app-refinegodweaponresetyesnodialog")]
#[::unity2::methods]
impl RefineGodWeaponResetYesNoDialog {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        yes_event_handler : crate :: app :: refinegodweaponresetyesnodialog :: RefineGodWeaponResetYesNoDialog_YesEventHandler,
    ) -> crate::app::yesnodialog::YesNoDialog;

    #[method(name = "CreateBindForSkill", args = 5)]
    pub fn create_bind_for_skill(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        yes_event_handler : crate :: app :: refinegodweaponresetyesnodialog :: RefineGodWeaponResetYesNoDialog_YesEventHandler,
    ) -> crate::app::yesnodialog::YesNoDialog;

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

#[cfg(feature = "app-refinegodweaponresetyesnodialog")]
impl RefineGodWeaponResetYesNoDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponResetYesNoDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponResetYesNoDialogMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponresetyesnodialog/RefineGodWeaponResetYesNoDialog_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineGodWeaponResetYesNoDialog.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct RefineGodWeaponResetYesNoDialog_YesMenuItem {
# [rename (name = "m_YesEventHandler")] pub m_yes_event_handler : crate :: app :: refinegodweaponresetyesnodialog :: RefineGodWeaponResetYesNoDialog_YesEventHandler ,
}

#[cfg(feature = "app-refinegodweaponresetyesnodialog")]
#[::unity2::methods]
impl RefineGodWeaponResetYesNoDialog_YesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        yes_event_handler : crate :: app :: refinegodweaponresetyesnodialog :: RefineGodWeaponResetYesNoDialog_YesEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refinegodweaponresetyesnodialog")]
impl RefineGodWeaponResetYesNoDialog_YesMenuItem {
    pub fn new(
        yes_event_handler : crate :: app :: refinegodweaponresetyesnodialog :: RefineGodWeaponResetYesNoDialog_YesEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponResetYesNoDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponResetYesNoDialog_YesMenuItemMethods>::ctor(
            this,
            yes_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponresetyesnodialog/RefineGodWeaponResetYesNoDialog_YesEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineGodWeaponResetYesNoDialog.YesEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineGodWeaponResetYesNoDialog_YesEventHandler {}

#[cfg(feature = "app-refinegodweaponresetyesnodialog")]
#[::unity2::methods]
impl RefineGodWeaponResetYesNoDialog_YesEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refinegodweaponresetyesnodialog")]
impl RefineGodWeaponResetYesNoDialog_YesEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponResetYesNoDialog_YesEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponResetYesNoDialog_YesEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}
