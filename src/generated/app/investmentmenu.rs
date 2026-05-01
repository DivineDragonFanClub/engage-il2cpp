
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmenu/InvestmentMenu_InvestmentMenuItem_InvestmentSubMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "InvestmentMenu.InvestmentMenuItem.InvestmentSubMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct InvestmentMenu_InvestmentMenuItem_InvestmentSubMenuItem {
    #[rename(name = "m_useCost")]
    pub m_use_cost: i32,
    #[rename(name = "m_parent")]
    pub m_parent: crate::app::investmentmenu::InvestmentMenu_InvestmentMenuItem,
}

#[cfg(feature = "app-investmentmenu")]
#[::unity2::methods]
impl InvestmentMenu_InvestmentMenuItem_InvestmentSubMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        cost: i32,
        parent: crate::app::investmentmenu::InvestmentMenu_InvestmentMenuItem,
    ) -> ();

    #[method(name = "SetTextColor", args = 2)]
    pub fn set_text_color(self, color: crate::unity_engine::color::Color, b_inactive: bool) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-investmentmenu")]
impl InvestmentMenu_InvestmentMenuItem_InvestmentSubMenuItem {
    pub fn new(
        cost: i32,
        parent: crate::app::investmentmenu::InvestmentMenu_InvestmentMenuItem,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMenu_InvestmentMenuItem_InvestmentSubMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMenu_InvestmentMenuItem_InvestmentSubMenuItemMethods>::ctor(
            this, cost, parent,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmenu/InvestmentMenu_InvestmentMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentMenu.InvestmentMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct InvestmentMenu_InvestmentMenuItem {
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-investmentmenu")]
#[::unity2::methods]
impl InvestmentMenu_InvestmentMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, index: i32) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetNationData", args = 0)]
    pub fn get_nation_data(self) -> crate::app::hubnationdata::HubNationData;

    #[method(name = "GetNextCost", args = 0)]
    pub fn get_next_cost(self) -> i32;

    #[method(name = "Add", args = 1)]
    pub fn add(self, add_cost: i32) -> ();
}

#[cfg(feature = "app-investmentmenu")]
impl InvestmentMenu_InvestmentMenuItem {
    pub fn new(index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMenu_InvestmentMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMenu_InvestmentMenuItemMethods>::ctor(this, index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmenu/InvestmentMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct InvestmentMenu_DecideEventHandler {}

#[cfg(feature = "app-investmentmenu")]
#[::unity2::methods]
impl InvestmentMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, index: i32) -> ();
}

#[cfg(feature = "app-investmentmenu")]
impl InvestmentMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmenu/InvestmentMenu.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct InvestmentMenu {}

#[cfg(feature = "app-investmentmenu")]
#[::unity2::methods]
impl InvestmentMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::investmentmenucontent::InvestmentMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SelectNation", args = 1)]
    pub fn select_nation(self, index: i32) -> ();

    #[method(name = "UpdateNation", args = 1)]
    pub fn update_nation(self, index: i32) -> ();
}

#[cfg(feature = "app-investmentmenu")]
impl InvestmentMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::investmentmenucontent::InvestmentMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
