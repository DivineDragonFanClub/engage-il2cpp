
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuymenu/AccessoryShopBuyMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopBuyMenu.RequestCloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopBuyMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-accessoryshopbuymenu")]
#[::unity2::methods]
impl AccessoryShopBuyMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-accessoryshopbuymenu")]
impl AccessoryShopBuyMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyMenu_RequestCloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuymenu/AccessoryShopBuyMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopBuyMenu_DecideEventHandler {}

#[cfg(feature = "app-accessoryshopbuymenu")]
#[::unity2::methods]
impl AccessoryShopBuyMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, accessory_data: crate::app::accessorydata::AccessoryData) -> ();
}

#[cfg(feature = "app-accessoryshopbuymenu")]
impl AccessoryShopBuyMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuymenu/AccessoryShopBuyMenu.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct AccessoryShopBuyMenu {
    #[rename(name = "m_AccessoryShopContentArray")]
    pub m_accessory_shop_content_array:
        ::unity2::Array<crate::app::accessoryshopcontent::AccessoryShopContent>,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::accessorydata::AccessoryData_Kinds,
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::accessoryshopbuymenu::AccessoryShopBuyMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::accessoryshopbuymenu::AccessoryShopBuyMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::accessoryshopbuymenu::AccessoryShopBuyMenu_RequestCloseEventHandler,
    #[rename(name = "m_ChangeKindEventHandler")]
    pub m_change_kind_event_handler:
        crate::app::accessoryshopbuymenu::AccessoryShopBuyMenu_ChangeKindEventHandler,
}

#[cfg(feature = "app-accessoryshopbuymenu")]
#[::unity2::methods]
impl AccessoryShopBuyMenu {
    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_object: crate::unity_engine::gameobject::GameObject,
        select_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_RequestCloseEventHandler,
        change_kind_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_ChangeKindEventHandler,
    ) -> crate::app::accessoryshopbuymenu::AccessoryShopBuyMenu;

    #[method(name = "CreateMenuItem", args = 5)]
    pub fn create_menu_item(
        shop_content_array: ::unity2::Array<crate::app::accessoryshopcontent::AccessoryShopContent>,
        show_row_num: i32,
        kind: crate::app::accessorydata::AccessoryData_Kinds,
        select_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_DecideEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::accessoryshopbuymenucontent::AccessoryShopBuyMenuContent,
        accessory_shop_content_array: ::unity2::Array<
            crate::app::accessoryshopcontent::AccessoryShopContent,
        >,
        select_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_RequestCloseEventHandler,
        change_kind_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_ChangeKindEventHandler,
    ) -> ();

    #[method(name = "RebuildMenu", args = 1)]
    pub fn rebuild_menu(self, setup_shopdata: bool) -> ();

    #[method(name = "RebuildMenuItem", args = 0)]
    pub fn rebuild_menu_item(self) -> ();

    #[method(name = "Filter", args = 2)]
    pub fn filter(
        shop_content_array: ::unity2::Array<crate::app::accessoryshopcontent::AccessoryShopContent>,
        kind: crate::app::accessorydata::AccessoryData_Kinds,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::accessoryshopcontent::AccessoryShopContent,
    >;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "EnableMenu", args = 0)]
    pub fn enable_menu(self) -> ();

    #[method(name = "DisableMenu", args = 0)]
    pub fn disable_menu(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "GetSelect", args = 0)]
    pub fn get_select(self) -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = "GetCurrentAccessoryKind", args = 0)]
    pub fn get_current_accessory_kind(self) -> crate::app::accessorydata::AccessoryData_Kinds;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "OnDoneToBuy", args = 0)]
    pub fn on_done_to_buy(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-accessoryshopbuymenu")]
impl AccessoryShopBuyMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::accessoryshopbuymenucontent::AccessoryShopBuyMenuContent,
        accessory_shop_content_array: ::unity2::Array<
            crate::app::accessoryshopcontent::AccessoryShopContent,
        >,
        select_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_RequestCloseEventHandler,
        change_kind_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_ChangeKindEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            accessory_shop_content_array,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
            change_kind_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuymenu/AccessoryShopBuyMenu_ChangeKindEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopBuyMenu.ChangeKindEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopBuyMenu_ChangeKindEventHandler {}

#[cfg(feature = "app-accessoryshopbuymenu")]
#[::unity2::methods]
impl AccessoryShopBuyMenu_ChangeKindEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, accessory_data: crate::app::accessorydata::AccessoryData) -> ();
}

#[cfg(feature = "app-accessoryshopbuymenu")]
impl AccessoryShopBuyMenu_ChangeKindEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyMenu_ChangeKindEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyMenu_ChangeKindEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuymenu/AccessoryShopBuyMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopBuyMenu_SelectEventHandler {}

#[cfg(feature = "app-accessoryshopbuymenu")]
#[::unity2::methods]
impl AccessoryShopBuyMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, accessory_data: crate::app::accessorydata::AccessoryData) -> ();
}

#[cfg(feature = "app-accessoryshopbuymenu")]
impl AccessoryShopBuyMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
