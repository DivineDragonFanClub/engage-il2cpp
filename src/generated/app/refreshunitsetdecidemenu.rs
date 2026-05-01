
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
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitsetdecidemenu/RefreshUnitSetDecideMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefreshUnitSetDecideMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshUnitSetDecideMenu_DecideEventHandler {}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
#[::unity2::methods]
impl RefreshUnitSetDecideMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        result: crate::app::refreshunitsetdecidemenu::RefreshUnitSetDecideMenu_Result2,
    ) -> ();
}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
impl RefreshUnitSetDecideMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSetDecideMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSetDecideMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitsetdecidemenu/RefreshUnitSetDecideMenu_RefreshUnitSetDecideMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefreshUnitSetDecideMenu.RefreshUnitSetDecideMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefreshUnitSetDecideMenu_RefreshUnitSetDecideMenuItem {
    #[rename(name = "m_Enabled")]
    pub m_enabled: bool,
    #[rename(name = "m_Usabled")]
    pub m_usabled: bool,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refreshunitsetdecidemenu::RefreshUnitSetDecideMenu_DecideEventHandler,
}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
#[::unity2::methods]
impl RefreshUnitSetDecideMenu_RefreshUnitSetDecideMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        enabled: bool,
        usabled: bool,
        decide_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "UpdateEnabled", args = 2)]
    pub fn update_enabled(self, enabled: bool, usabled: bool) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
impl RefreshUnitSetDecideMenu_RefreshUnitSetDecideMenuItem {
    pub fn new(
        enabled: bool,
        usabled: bool,
        decide_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSetDecideMenu_RefreshUnitSetDecideMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSetDecideMenu_RefreshUnitSetDecideMenuItemMethods>::ctor(
            this,
            enabled,
            usabled,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitsetdecidemenu/RefreshUnitSetDecideMenu_KeyDownEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefreshUnitSetDecideMenu.KeyDownEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshUnitSetDecideMenu_KeyDownEventHandler {}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
#[::unity2::methods]
impl RefreshUnitSetDecideMenu_KeyDownEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
impl RefreshUnitSetDecideMenu_KeyDownEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSetDecideMenu_KeyDownEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSetDecideMenu_KeyDownEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitsetdecidemenu/RefreshUnitSetDecideMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RefreshUnitSetDecideMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RefreshUnitSetDecideMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RefreshUnitSetDecideMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RefreshUnitSetDecideMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RefreshUnitSetDecideMenu_Result2 {
    pub fn ok() -> Self {
        Self { value: 0 }
    }

    pub fn cancel() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitsetdecidemenu/RefreshUnitSetDecideMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSetDecideMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefreshUnitSetDecideMenu {
    #[rename(name = "m_KeyUpEventHandler")]
    pub m_key_up_event_handler:
        crate::app::refreshunitsetdecidemenu::RefreshUnitSetDecideMenu_KeyUpEventHandler,
    #[rename(name = "m_KeyDownEventHandler")]
    pub m_key_down_event_handler:
        crate::app::refreshunitsetdecidemenu::RefreshUnitSetDecideMenu_KeyDownEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refreshunitsetdecidemenu::RefreshUnitSetDecideMenu_DecideEventHandler,
    #[rename(name = "m_Closed")]
    pub m_closed: bool,
}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
#[::unity2::methods]
impl RefreshUnitSetDecideMenu {
    #[method(name = "Create", args = 7)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::refreshunitsetdecidemenucontent::RefreshUnitSetDecideMenuContent,
        enabled: bool,
        usabled: bool,
        key_up_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_KeyUpEventHandler,
        key_down_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_KeyDownEventHandler,
        decide_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_DecideEventHandler,
    ) -> crate::app::refreshunitsetdecidemenu::RefreshUnitSetDecideMenu;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::refreshunitsetdecidemenucontent::RefreshUnitSetDecideMenuContent,
        usabled: bool,
        key_up_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_KeyUpEventHandler,
        key_down_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_KeyDownEventHandler,
        decide_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateMenuItemList", args = 3)]
    pub fn create_menu_item_list(
        enabled: bool,
        usabled: bool,
        decide_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_DecideEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "SetTextColor", args = 1)]
    pub fn set_text_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "UpdateEnabled", args = 2)]
    pub fn update_enabled(self, enabled: bool, usabled: bool) -> ();

    #[method(name = "FocusThisMenu", args = 0)]
    pub fn focus_this_menu(self) -> ();

    #[method(name = "UnfocusThisMenu", args = 0)]
    pub fn unfocus_this_menu(self) -> ();

    #[method(name = "IsFocusedThisMenu", args = 0)]
    pub fn is_focused_this_menu(self) -> bool;

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "LCall", args = 0)]
    pub fn l_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
impl RefreshUnitSetDecideMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::refreshunitsetdecidemenucontent::RefreshUnitSetDecideMenuContent,
        usabled: bool,
        key_up_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_KeyUpEventHandler,
        key_down_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_KeyDownEventHandler,
        decide_event_handler : crate :: app :: refreshunitsetdecidemenu :: RefreshUnitSetDecideMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSetDecideMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSetDecideMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            usabled,
            key_up_event_handler,
            key_down_event_handler,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitsetdecidemenu/RefreshUnitSetDecideMenu_KeyUpEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSetDecideMenu.KeyUpEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshUnitSetDecideMenu_KeyUpEventHandler {}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
#[::unity2::methods]
impl RefreshUnitSetDecideMenu_KeyUpEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refreshunitsetdecidemenu")]
impl RefreshUnitSetDecideMenu_KeyUpEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSetDecideMenu_KeyUpEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSetDecideMenu_KeyUpEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
