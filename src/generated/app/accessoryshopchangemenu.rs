
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangemenu/AccessoryShopChangeMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopChangeMenu.RequestCloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopChangeMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-accessoryshopchangemenu")]
#[::unity2::methods]
impl AccessoryShopChangeMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-accessoryshopchangemenu")]
impl AccessoryShopChangeMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeMenu_RequestCloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangemenu/AccessoryShopChangeMenu_ChangeKindEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopChangeMenu.ChangeKindEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopChangeMenu_ChangeKindEventHandler {}

#[cfg(feature = "app-accessoryshopchangemenu")]
#[::unity2::methods]
impl AccessoryShopChangeMenu_ChangeKindEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, accessory_data: crate::app::accessorydata::AccessoryData) -> ();
}

#[cfg(feature = "app-accessoryshopchangemenu")]
impl AccessoryShopChangeMenu_ChangeKindEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeMenu_ChangeKindEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeMenu_ChangeKindEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangemenu/AccessoryShopChangeMenu.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopChangeMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct AccessoryShopChangeMenu {
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::accessorydata::AccessoryData_Kinds,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::accessoryshopchangemenu::AccessoryShopChangeMenu_RequestCloseEventHandler,
    #[rename(name = "m_ChangeKindEventHandler")]
    pub m_change_kind_event_handler:
        crate::app::accessoryshopchangemenu::AccessoryShopChangeMenu_ChangeKindEventHandler,
}

#[cfg(feature = "app-accessoryshopchangemenu")]
#[::unity2::methods]
impl AccessoryShopChangeMenu {
    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_object: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_DecideEventHandler,
        request_close_event_handler : crate :: app :: accessoryshopchangemenu :: AccessoryShopChangeMenu_RequestCloseEventHandler,
        change_kind_event_handler : crate :: app :: accessoryshopchangemenu :: AccessoryShopChangeMenu_ChangeKindEventHandler,
    ) -> crate::app::accessoryshopchangemenu::AccessoryShopChangeMenu;

    #[method(name = "CreateMenuItem", args = 5)]
    pub fn create_menu_item(
        show_row_num: i32,
        kind: crate::app::accessorydata::AccessoryData_Kinds,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_DecideEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::accessoryshopchangemenucontent::AccessoryShopChangeMenuContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_DecideEventHandler,
        request_close_event_handler : crate :: app :: accessoryshopchangemenu :: AccessoryShopChangeMenu_RequestCloseEventHandler,
        change_kind_event_handler : crate :: app :: accessoryshopchangemenu :: AccessoryShopChangeMenu_ChangeKindEventHandler,
    ) -> ();

    #[method(name = "RebuildMenu", args = 1)]
    pub fn rebuild_menu(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu_2(self) -> ();

    #[method(name = "RebuildMenuItem", args = 0)]
    pub fn rebuild_menu_item(self) -> ();

    #[method(name = "Filter", args = 2)]
    pub fn filter(
        kind: crate::app::accessorydata::AccessoryData_Kinds,
        unit: crate::app::unit::Unit,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::accessorydata::AccessoryData>;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

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

    #[method(name = "GetSelectedAccessoryData", args = 0)]
    pub fn get_selected_accessory_data(self) -> crate::app::accessorydata::AccessoryData;

    #[method(name = "GetCurrentAccessoryKind", args = 0)]
    pub fn get_current_accessory_kind(self) -> crate::app::accessorydata::AccessoryData_Kinds;

    #[method(name = "UpdateDecision", args = 1)]
    pub fn update_decision(
        self,
        unit_accessory_list: crate::app::unitaccessorylist::UnitAccessoryList,
    ) -> ();

    #[method(name = "UpdateCursorZOrder", args = 0)]
    pub fn update_cursor_z_order(self) -> ();

    #[method(name = "PrepareToChangeUnit", args = 0)]
    pub fn prepare_to_change_unit(self) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-accessoryshopchangemenu")]
impl AccessoryShopChangeMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::accessoryshopchangemenucontent::AccessoryShopChangeMenuContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_DecideEventHandler,
        request_close_event_handler : crate :: app :: accessoryshopchangemenu :: AccessoryShopChangeMenu_RequestCloseEventHandler,
        change_kind_event_handler : crate :: app :: accessoryshopchangemenu :: AccessoryShopChangeMenu_ChangeKindEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            unit,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
            change_kind_event_handler,
        );
        this
    }
}
