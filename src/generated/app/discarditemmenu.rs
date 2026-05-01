
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenu/DiscardItemMenu_DiscardItemEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DiscardItemMenu.DiscardItemEmptyMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct DiscardItemMenu_DiscardItemEmptyMenuItem {}

#[cfg(feature = "app-discarditemmenu")]
#[::unity2::methods]
impl DiscardItemMenu_DiscardItemEmptyMenuItem {
    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetBlankText", args = 0)]
    pub fn get_blank_text(self) -> ::unity2::Il2CppString;

    #[method(name = "IsVisibleItemIconOnBlank", args = 0)]
    pub fn is_visible_item_icon_on_blank(self) -> bool;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-discarditemmenu")]
impl DiscardItemMenu_DiscardItemEmptyMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenu_DiscardItemEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenu_DiscardItemEmptyMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenu/DiscardItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "DiscardItemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct DiscardItemMenu {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_AddedUnitItem")]
    pub m_added_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::discarditemroot::DiscardItemRoot,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::itemdata::ItemData_Kinds,
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_Uncancellable")]
    pub m_uncancellable: bool,
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::app::discarditemmenu::DiscardItemMenu_DecideCallback,
    #[rename(name = "m_CancelCallback")]
    pub m_cancel_callback: crate::app::discarditemmenu::DiscardItemMenu_CancelCallback,
}

#[cfg(feature = "app-discarditemmenu")]
#[::unity2::methods]
impl DiscardItemMenu {
    #[method(name = "get_m_AllMenuItemNum", args = 0)]
    pub fn get_m_all_menu_item_num(self) -> i32;

    #[method(name = "set_m_AllMenuItemNum", args = 1)]
    pub fn set_m_all_menu_item_num(self, value: i32) -> ();

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        added_unit_item: crate::app::unititem::UnitItem,
        uncancellable: bool,
        decide_callback: crate::app::discarditemmenu::DiscardItemMenu_DecideCallback,
        cancel_callback: crate::app::discarditemmenu::DiscardItemMenu_CancelCallback,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "CreateMenuItem", args = 2)]
    pub fn create_menu_item(
        kind: crate::app::itemdata::ItemData_Kinds,
        added_unit_item: crate::app::unititem::UnitItem,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "Compare", args = 2)]
    pub fn compare(
        lhs: crate::app::basicmenuitem::BasicMenuItem,
        rhs: crate::app::basicmenuitem::BasicMenuItem,
    ) -> i32;

    #[method(name = ".ctor", args = 8)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        root: crate::app::discarditemroot::DiscardItemRoot,
        unit: crate::app::unit::Unit,
        added_unit_item: crate::app::unititem::UnitItem,
        uncancellable: bool,
        decide_callback: crate::app::discarditemmenu::DiscardItemMenu_DecideCallback,
        cancel_callback: crate::app::discarditemmenu::DiscardItemMenu_CancelCallback,
    ) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetAddedUnitItem", args = 0)]
    pub fn get_added_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetSelect", args = 0)]
    pub fn get_select(self) -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "DiscardItem", args = 2)]
    pub fn discard_item(self, transporter_is_selected: bool, transporter_item_index: i32) -> ();
}

#[cfg(feature = "app-discarditemmenu")]
impl DiscardItemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        root: crate::app::discarditemroot::DiscardItemRoot,
        unit: crate::app::unit::Unit,
        added_unit_item: crate::app::unititem::UnitItem,
        uncancellable: bool,
        decide_callback: crate::app::discarditemmenu::DiscardItemMenu_DecideCallback,
        cancel_callback: crate::app::discarditemmenu::DiscardItemMenu_CancelCallback,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            root,
            unit,
            added_unit_item,
            uncancellable,
            decide_callback,
            cancel_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenu/DiscardItemMenu_ConfirmDialog_YesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DiscardItemMenu.ConfirmDialog.YesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct DiscardItemMenu_ConfirmDialog_YesDialogItem {
    #[rename(name = "m_YesEventHandler")]
    pub m_yes_event_handler:
        crate::app::discarditemmenu::DiscardItemMenu_ConfirmDialog_YesEventHandler,
}

#[cfg(feature = "app-discarditemmenu")]
#[::unity2::methods]
impl DiscardItemMenu_ConfirmDialog_YesDialogItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        yes_event_handler : crate :: app :: discarditemmenu :: DiscardItemMenu_ConfirmDialog_YesEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-discarditemmenu")]
impl DiscardItemMenu_ConfirmDialog_YesDialogItem {
    pub fn new(
        yes_event_handler : crate :: app :: discarditemmenu :: DiscardItemMenu_ConfirmDialog_YesEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenu_ConfirmDialog_YesDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenu_ConfirmDialog_YesDialogItemMethods>::ctor(
            this,
            yes_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenu/DiscardItemMenu_DiscardItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DiscardItemMenu.DiscardItemMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct DiscardItemMenu_DiscardItemMenuItem {}

#[cfg(feature = "app-discarditemmenu")]
#[::unity2::methods]
impl DiscardItemMenu_DiscardItemMenuItem {
    #[method(name = "get_m_OwnerItemIndex", args = 0)]
    pub fn get_m_owner_item_index(self) -> i32;

    #[method(name = "set_m_OwnerItemIndex", args = 1)]
    pub fn set_m_owner_item_index(self, value: i32) -> ();

    #[method(name = "get_m_SortValue", args = 0)]
    pub fn get_m_sort_value(self) -> i64;

    #[method(name = "set_m_SortValue", args = 1)]
    pub fn set_m_sort_value(self, value: i64) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, owner_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetRecieverUnit", args = 0)]
    pub fn get_reciever_unit(self) -> crate::app::unit::Unit;
}

#[cfg(feature = "app-discarditemmenu")]
impl DiscardItemMenu_DiscardItemMenuItem {
    pub fn new(owner_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenu_DiscardItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenu_DiscardItemMenuItemMethods>::ctor(this, owner_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenu/DiscardItemMenu_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "DiscardItemMenu.ConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct DiscardItemMenu_ConfirmDialog {}

#[cfg(feature = "app-discarditemmenu")]
#[::unity2::methods]
impl DiscardItemMenu_ConfirmDialog {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        discarding_unit_item: crate::app::unititem::UnitItem,
        decide_callback: crate::app::discarditemmenu::DiscardItemMenu_ConfirmDialog_YesEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();
}

#[cfg(feature = "app-discarditemmenu")]
impl DiscardItemMenu_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenu_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenu_ConfirmDialogMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenu/DiscardItemMenu_ConfirmDialog_YesEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DiscardItemMenu.ConfirmDialog.YesEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct DiscardItemMenu_ConfirmDialog_YesEventHandler {}

#[cfg(feature = "app-discarditemmenu")]
#[::unity2::methods]
impl DiscardItemMenu_ConfirmDialog_YesEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-discarditemmenu")]
impl DiscardItemMenu_ConfirmDialog_YesEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenu_ConfirmDialog_YesEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenu_ConfirmDialog_YesEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenu/DiscardItemMenu_CancelCallback.md")))]
#[::unity2::class(namespace = "App", name = "DiscardItemMenu.CancelCallback")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct DiscardItemMenu_CancelCallback {}

#[cfg(feature = "app-discarditemmenu")]
#[::unity2::methods]
impl DiscardItemMenu_CancelCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-discarditemmenu")]
impl DiscardItemMenu_CancelCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenu_CancelCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenu_CancelCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenu/DiscardItemMenu_DecideCallback.md")))]
#[::unity2::class(namespace = "App", name = "DiscardItemMenu.DecideCallback")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct DiscardItemMenu_DecideCallback {}

#[cfg(feature = "app-discarditemmenu")]
#[::unity2::methods]
impl DiscardItemMenu_DecideCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, transporter_is_selected: bool, transporter_item_index: i32) -> ();
}

#[cfg(feature = "app-discarditemmenu")]
impl DiscardItemMenu_DecideCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenu_DecideCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenu_DecideCallbackMethods>::ctor(this, object, method);
        this
    }
}
