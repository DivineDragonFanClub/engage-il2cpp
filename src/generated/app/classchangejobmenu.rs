
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
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchangejobmenu/ClassChangeJobMenu_ConfirmDialog_ConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ClassChangeJobMenu.ConfirmDialog.ConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct ClassChangeJobMenu_ConfirmDialog_ConfirmDialogItemYes {
    #[rename(name = "ACallCallback")]
    pub a_call_callback: crate::system::action::Action,
}

#[cfg(feature = "app-classchangejobmenu")]
#[::unity2::methods]
impl ClassChangeJobMenu_ConfirmDialog_ConfirmDialogItemYes {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        a_call_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-classchangejobmenu")]
impl ClassChangeJobMenu_ConfirmDialog_ConfirmDialogItemYes {
    pub fn new(
        text: ::unity2::Il2CppString,
        a_call_callback: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeJobMenu_ConfirmDialog_ConfirmDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeJobMenu_ConfirmDialog_ConfirmDialogItemYesMethods>::ctor(
            this,
            text,
            a_call_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchangejobmenu/ClassChangeJobMenu.md")))]
#[::unity2::class(namespace = "App", name = "ClassChangeJobMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ClassChangeJobMenu {
    #[static_field]
    #[rename(name = "c_JobSortMax")]
    pub c_job_sort_max: i32,
    #[static_field]
    #[rename(name = "s_IsSortJobSort")]
    pub s_is_sort_job_sort: bool,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::classchangeroot::ClassChangeRoot,
}

#[cfg(feature = "app-classchangejobmenu")]
#[::unity2::methods]
impl ClassChangeJobMenu {
    #[method(name = "Create", args = 2)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        class_change_root: crate::app::classchangeroot::ClassChangeRoot,
    ) -> crate::app::classchangejobmenu::ClassChangeJobMenu;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        root: crate::app::classchangeroot::ClassChangeRoot,
    ) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SortMenuItem", args = 0)]
    pub fn sort_menu_item(self) -> ();

    #[method(name = "SetUnitInfoAll", args = 0)]
    pub fn set_unit_info_all(self) -> ();

    #[method(name = "SetUnitInfoAll", args = 1)]
    pub fn set_unit_info_all_2(
        self,
        after_change_job_data: crate::app::classchange::ClassChange_ChangeJobData,
    ) -> ();

    #[method(name = "SetUnitInfoAfter", args = 2)]
    pub fn set_unit_info_after(
        self,
        unit_before: crate::app::unit::Unit,
        data: crate::app::classchange::ClassChange_ChangeJobData,
    ) -> ();

    #[method(name = "SetJobDetails", args = 1)]
    pub fn set_job_details(self, data: crate::app::classchange::ClassChange_ChangeJobData) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit() -> crate::app::unit::Unit;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-classchangejobmenu")]
impl ClassChangeJobMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        root: crate::app::classchangeroot::ClassChangeRoot,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeJobMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeJobMenuMethods>::ctor(this, menu_item_list, root);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchangejobmenu/ClassChangeJobMenu_ClassChangeJobMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ClassChangeJobMenu.ClassChangeJobMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ClassChangeJobMenu_ClassChangeJobMenuItem {
    #[rename(name = "m_JobData")]
    pub m_job_data: crate::app::classchange::ClassChange_ChangeJobData,
    #[rename(name = "m_Attribute")]
    pub m_attribute: crate::app::basicmenuitem::BasicMenuItem_Attribute,
}

#[cfg(feature = "app-classchangejobmenu")]
#[::unity2::methods]
impl ClassChangeJobMenu_ClassChangeJobMenuItem {
    #[method(name = "GetJobData", args = 0)]
    pub fn get_job_data(self) -> crate::app::classchange::ClassChange_ChangeJobData;

    #[method(name = "SetAttribute", args = 0)]
    pub fn set_attribute(self) -> ();

    #[method(name = "IsCanChange", args = 0)]
    pub fn is_can_change(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, job_data: crate::app::classchange::ClassChange_ChangeJobData) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-classchangejobmenu")]
impl ClassChangeJobMenu_ClassChangeJobMenuItem {
    pub fn new(job_data: crate::app::classchange::ClassChange_ChangeJobData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeJobMenu_ClassChangeJobMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeJobMenu_ClassChangeJobMenuItemMethods>::ctor(this, job_data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchangejobmenu/ClassChangeJobMenu_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "ClassChangeJobMenu.ConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct ClassChangeJobMenu_ConfirmDialog {
    #[static_field]
    #[rename(name = "m_data")]
    pub m_data: crate::app::classchange::ClassChange_ChangeJobData,
}

#[cfg(feature = "app-classchangejobmenu")]
#[::unity2::methods]
impl ClassChangeJobMenu_ConfirmDialog {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::basicmenu::BasicMenu,
        data: crate::app::classchange::ClassChange_ChangeJobData,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();
}

#[cfg(feature = "app-classchangejobmenu")]
impl ClassChangeJobMenu_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeJobMenu_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeJobMenu_ConfirmDialogMethods>::ctor(this, menu_item_list);
        this
    }
}
