
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiiboaccessorymenu/AmiiboAccessoryMenu.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboAccessoryMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct AmiiboAccessoryMenu {}

#[cfg(feature = "app-amiiboaccessorymenu")]
#[::unity2::methods]
impl AmiiboAccessoryMenu {
    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit() -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(value: crate::app::unit::Unit) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::amiiboaccessorymenucontent::AmiiboAccessoryMenuContent,
    ) -> ();

    #[method(name = "OnResume", args = 0)]
    pub fn on_resume(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetPrevUnit", args = 2)]
    pub fn get_prev_unit(
        unit: crate::app::unit::Unit,
        data: crate::app::accessorydata::AccessoryData,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetNextUnit", args = 2)]
    pub fn get_next_unit(
        unit: crate::app::unit::Unit,
        data: crate::app::accessorydata::AccessoryData,
    ) -> crate::app::unit::Unit;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "SetMenuInfo", args = 1)]
    pub fn set_menu_info(self, data: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "Buy", args = 0)]
    pub fn buy(self) -> ();

    #[method(name = "ExchangeRemainTicket", args = 0)]
    pub fn exchange_remain_ticket(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-amiiboaccessorymenu")]
impl AmiiboAccessoryMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::amiiboaccessorymenucontent::AmiiboAccessoryMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboAccessoryMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboAccessoryMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
