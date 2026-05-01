
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_ConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "InventorySubMenu.ThrowAwayMenuItem.ConfirmDialog.ConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_ConfirmDialogItemYes {
# [rename (name = "m_Unit")] pub m_unit : crate :: app :: unit :: Unit ,
# [rename (name = "m_UnitItemIndex")] pub m_unit_item_index : i32 ,
# [rename (name = "m_DecideCallback")] pub m_decide_callback : crate :: app :: inventorysubmenu :: InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback ,
}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_ConfirmDialogItemYes {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        decide_callback : crate :: app :: inventorysubmenu :: InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_ConfirmDialogItemYes {
    pub fn new(
        text: ::unity2::Il2CppString,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        decide_callback : crate :: app :: inventorysubmenu :: InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_ConfirmDialogItemYes
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IInventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_ConfirmDialogItemYesMethods > :: ctor (this , text , unit , unit_item_index , decide_callback) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_EquipMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.EquipMenuItem")]
#[parent(crate::app::inventorysubmenu::InventorySubMenu_BaseMenuItem)]
pub struct InventorySubMenu_EquipMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_EquipMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_EquipMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_EquipMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_EquipMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "InventorySubMenu.ThrowAwayMenuItem.ConfirmDialog"
)]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        decide_callback : crate :: app :: inventorysubmenu :: InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback,
    ) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_ThrowAwayMenuItem_ConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_ReceiveMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.ReceiveMenuItem")]
#[parent(crate::app::inventorysubmenu::InventorySubMenu_BaseMenuItem)]
pub struct InventorySubMenu_ReceiveMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_ReceiveMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_ReceiveMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_ReceiveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_ReceiveMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_TakeOffMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.TakeOffMenuItem")]
#[parent(crate::app::inventorysubmenu::InventorySubMenu_BaseMenuItem)]
pub struct InventorySubMenu_TakeOffMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_TakeOffMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_TakeOffMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_TakeOffMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_TakeOffMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct InventorySubMenu {
    #[static_field]
    #[rename(name = "MenuOffsetX")]
    pub menu_offset_x: f32,
}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        parent_menu: crate::app::basicmenu::BasicMenu,
        parent_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    ) -> ();

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
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_UseMenuItem_ConfirmDialog_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "InventorySubMenu.UseMenuItem.ConfirmDialog.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct InventorySubMenu_UseMenuItem_ConfirmDialog_YesMenuItem {
    #[rename(name = "m_YesEventHandler")]
    pub m_yes_event_handler:
        crate::app::inventorysubmenu::InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler,
}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_UseMenuItem_ConfirmDialog_YesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        yes_event_handler : crate :: app :: inventorysubmenu :: InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_UseMenuItem_ConfirmDialog_YesMenuItem {
    pub fn new(
        yes_event_handler : crate :: app :: inventorysubmenu :: InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_UseMenuItem_ConfirmDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_UseMenuItem_ConfirmDialog_YesMenuItemMethods>::ctor(
            this,
            yes_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_UseMenuItem_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.UseMenuItem.ConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct InventorySubMenu_UseMenuItem_ConfirmDialog {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_UseMenuItem_ConfirmDialog {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        item_data: crate::app::itemdata::ItemData,
        yes_event_handler : crate :: app :: inventorysubmenu :: InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler,
    ) -> crate::app::inventorysubmenu::InventorySubMenu_UseMenuItem_ConfirmDialog;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        dialog_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_UseMenuItem_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        dialog_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_UseMenuItem_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_UseMenuItem_ConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
            dialog_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_ThrowAwayMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.ThrowAwayMenuItem")]
#[parent(crate::app::inventorysubmenu::InventorySubMenu_BaseMenuItem)]
pub struct InventorySubMenu_ThrowAwayMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_ThrowAwayMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "ThrowAway", args = 0)]
    pub fn throw_away(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_ThrowAwayMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_ThrowAwayMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_ThrowAwayMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "InventorySubMenu.UseMenuItem.ConfirmDialog.YesEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_UseMenuItem_ConfirmDialog_YesEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_BaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.BaseMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct InventorySubMenu_BaseMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_BaseMenuItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_BaseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_BaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_BaseMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback.md")))]
#[::unity2::class(
    namespace = "App",
    name = "InventorySubMenu.ThrowAwayMenuItem.ConfirmDialog.DecideCallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_ThrowAwayMenuItem_ConfirmDialog_DecideCallbackMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_TradeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.TradeMenuItem")]
#[parent(crate::app::inventorysubmenu::InventorySubMenu_BaseMenuItem)]
pub struct InventorySubMenu_TradeMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_TradeMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_TradeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_TradeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_TradeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_UseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.UseMenuItem")]
#[parent(crate::app::inventorysubmenu::InventorySubMenu_BaseMenuItem)]
pub struct InventorySubMenu_UseMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_UseMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnConfirmYes", args = 0)]
    pub fn on_confirm_yes(self) -> ();

    #[method(name = "UseItem", args = 0)]
    pub fn use_item(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_UseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_UseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_UseMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_StoreMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.StoreMenuItem")]
#[parent(crate::app::inventorysubmenu::InventorySubMenu_BaseMenuItem)]
pub struct InventorySubMenu_StoreMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_StoreMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_StoreMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_StoreMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_StoreMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorysubmenu/InventorySubMenu_SortMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventorySubMenu.SortMenuItem")]
#[parent(crate::app::inventorysubmenu::InventorySubMenu_BaseMenuItem)]
pub struct InventorySubMenu_SortMenuItem {}

#[cfg(feature = "app-inventorysubmenu")]
#[::unity2::methods]
impl InventorySubMenu_SortMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorysubmenu")]
impl InventorySubMenu_SortMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventorySubMenu_SortMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventorySubMenu_SortMenuItemMethods>::ctor(this);
        this
    }
}
