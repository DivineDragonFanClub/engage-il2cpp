
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
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenu_DecideEventHandler {}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_DecideItemEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenu.DecideItemEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenu_DecideItemEventHandler {}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu_DecideItemEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, item_count: i32, total_value: i32) -> ();
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu_DecideItemEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu_DecideItemEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenu_DecideItemEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_KindIndicator.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ShopSellMenu_KindIndicator {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ShopSellMenu_KindIndicator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ShopSellMenu.KindIndicator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShopSellMenu_KindIndicator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ShopSellMenu_KindIndicator {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn first() -> Self {
        Self { value: 1 }
    }

    pub fn last() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenu_SelectEventHandler {}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit_item: crate::app::unititem::UnitItem) -> ();
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_SwitchDetailDisplaywayEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ShopSellMenu.SwitchDetailDisplaywayEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenu_SwitchDetailDisplaywayEventHandler {}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu_SwitchDetailDisplaywayEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu_SwitchDetailDisplaywayEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu_SwitchDetailDisplaywayEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenu_SwitchDetailDisplaywayEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenu.RequestCloseEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenu_RequestCloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_CancelItemEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenu.CancelItemEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenu_CancelItemEventHandler {}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu_CancelItemEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, item_count: i32, total_value: i32) -> ();
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu_CancelItemEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu_CancelItemEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenu_CancelItemEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_ChangeUnitToPrevEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenu.ChangeUnitToPrevEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenu_ChangeUnitToPrevEventHandler {}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu_ChangeUnitToPrevEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu_ChangeUnitToPrevEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu_ChangeUnitToPrevEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenu_ChangeUnitToPrevEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ShopSellMenu {
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::itemdata::ItemData_Kinds,
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_DecidedItemFlag")]
    pub m_decided_item_flag: ::unity2::Array<bool>,
    #[rename(name = "m_DecidedItemList")]
    pub m_decided_item_list:
        crate::system::collections::generic::list_1::List_1<crate::app::unititem::UnitItem>,
    #[rename(name = "m_TotalValue")]
    pub m_total_value: i32,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
    #[rename(name = "m_DecideItemEventHandler")]
    pub m_decide_item_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideItemEventHandler,
    #[rename(name = "m_CancelItemEventHandler")]
    pub m_cancel_item_event_handler: crate::app::shopsellmenu::ShopSellMenu_CancelItemEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::shopsellmenu::ShopSellMenu_RequestCloseEventHandler,
    #[rename(name = "m_ChangeUnitToPrevEventHandler")]
    pub m_change_unit_to_prev_event_handler:
        crate::app::shopsellmenu::ShopSellMenu_ChangeUnitToPrevEventHandler,
    #[rename(name = "m_ChangeUnitToNextEventHandler")]
    pub m_change_unit_to_next_event_handler:
        crate::app::shopsellmenu::ShopSellMenu_ChangeUnitToNextEventHandler,
    #[rename(name = "m_SwitchDetailDisplaywayEventHandler")]
    pub m_switch_detail_displayway_event_handler:
        crate::app::shopsellmenu::ShopSellMenu_SwitchDetailDisplaywayEventHandler,
}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu {
    #[method(name = "CreateBind", args = 11)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_object: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
        decide_item_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideItemEventHandler,
        cancel_item_event_handler: crate::app::shopsellmenu::ShopSellMenu_CancelItemEventHandler,
        decide_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_RequestCloseEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToNextEventHandler,
        switch_detail_displayway_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_SwitchDetailDisplaywayEventHandler,
    ) -> crate::app::shopsellmenu::ShopSellMenu;

    #[method(name = "BuildMenuItem", args = 10)]
    pub fn build_menu_item(
        unit: crate::app::unit::Unit,
        show_row_num: i32,
        kind: crate::app::itemdata::ItemData_Kinds,
        decided_item: ::unity2::Array<bool>,
        select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
        decide_item_event_handler : crate :: app :: shopsellmenuitem :: ShopSellMenuItem_DecideItemEventHandler,
        cancel_item_event_handler : crate :: app :: shopsellmenuitem :: ShopSellMenuItem_CancelItemEventHandler,
        decide_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToNextEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 11)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::shopsellmenucontent::ShopSellMenuContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
        decide_item_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideItemEventHandler,
        cancel_item_event_handler: crate::app::shopsellmenu::ShopSellMenu_CancelItemEventHandler,
        decide_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_RequestCloseEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToNextEventHandler,
        switch_detail_displayway_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_SwitchDetailDisplaywayEventHandler,
    ) -> ();

    #[method(name = "RebuildMenu", args = 1)]
    pub fn rebuild_menu(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu_2(self) -> ();

    #[method(name = "RebuildMenuItem", args = 0)]
    pub fn rebuild_menu_item(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSelect", args = 0)]
    pub fn get_select(self) -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = "EnableMenu", args = 1)]
    pub fn enable_menu(self, indicator: crate::app::shopsellmenu::ShopSellMenu_KindIndicator)
        -> ();

    #[method(name = "MoveFrontCursorFrom", args = 1)]
    pub fn move_front_cursor_from(self, from_menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "DisableMenu", args = 0)]
    pub fn disable_menu(self) -> ();

    #[method(name = "GetDecidedList", args = 1)]
    pub fn get_decided_list(
        self,
        max: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::unititem::UnitItem>;

    #[method(name = "GetTotalValue", args = 0)]
    pub fn get_total_value(self) -> i32;

    #[method(name = "ClearDecided", args = 0)]
    pub fn clear_decided(self) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDecideItem", args = 1)]
    pub fn on_decide_item(self, item_index: i32) -> ();

    #[method(name = "OnCancelItem", args = 1)]
    pub fn on_cancel_item(self, item_index: i32) -> ();

    #[method(name = "SellMulti", args = 0)]
    pub fn sell_multi(self) -> ();

    #[method(name = "Compare", args = 2)]
    pub fn compare(
        lhs: crate::app::basicmenuitem::BasicMenuItem,
        rhs: crate::app::basicmenuitem::BasicMenuItem,
    ) -> i32;
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::shopsellmenucontent::ShopSellMenuContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
        decide_item_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideItemEventHandler,
        cancel_item_event_handler: crate::app::shopsellmenu::ShopSellMenu_CancelItemEventHandler,
        decide_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_RequestCloseEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToNextEventHandler,
        switch_detail_displayway_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_SwitchDetailDisplaywayEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            unit,
            select_event_handler,
            decide_item_event_handler,
            cancel_item_event_handler,
            decide_event_handler,
            request_close_event_handler,
            change_unit_to_prev_event_handler,
            change_unit_to_next_event_handler,
            switch_detail_displayway_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenu/ShopSellMenu_ChangeUnitToNextEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenu.ChangeUnitToNextEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenu_ChangeUnitToNextEventHandler {}

#[cfg(feature = "app-shopsellmenu")]
#[::unity2::methods]
impl ShopSellMenu_ChangeUnitToNextEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-shopsellmenu")]
impl ShopSellMenu_ChangeUnitToNextEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenu_ChangeUnitToNextEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenu_ChangeUnitToNextEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
