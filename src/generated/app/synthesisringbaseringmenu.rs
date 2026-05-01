
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/synthesisringbaseringmenu/SynthesisRingBaseRingMenu_CloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SynthesisRingBaseRingMenu.CloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct SynthesisRingBaseRingMenu_CloseEventHandler {}

#[cfg(feature = "app-synthesisringbaseringmenu")]
#[::unity2::methods]
impl SynthesisRingBaseRingMenu_CloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-synthesisringbaseringmenu")]
impl SynthesisRingBaseRingMenu_CloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SynthesisRingBaseRingMenu_CloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as ISynthesisRingBaseRingMenu_CloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/synthesisringbaseringmenu/SynthesisRingBaseRingMenu.md")))]
#[::unity2::class(namespace = "App", name = "SynthesisRingBaseRingMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SynthesisRingBaseRingMenu {
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_CloseEventHandler")]
    pub m_close_event_handler:
        crate::app::synthesisringbaseringmenu::SynthesisRingBaseRingMenu_CloseEventHandler,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::ringmenuitem::RingMenuItem_DecideEventHandler,
}

#[cfg(feature = "app-synthesisringbaseringmenu")]
#[::unity2::methods]
impl SynthesisRingBaseRingMenu {
    #[method(name = "get_m_GodUnitList", args = 0)]
    pub fn get_m_god_unit_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>;

    #[method(name = "set_m_GodUnitList", args = 1)]
    pub fn set_m_god_unit_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>,
    ) -> ();

    #[method(name = "get_m_GodUnitIndex", args = 0)]
    pub fn get_m_god_unit_index(self) -> i32;

    #[method(name = "set_m_GodUnitIndex", args = 1)]
    pub fn set_m_god_unit_index(self, value: i32) -> ();

    #[method(name = "get_m_GodUnit", args = 0)]
    pub fn get_m_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "set_m_GodUnit", args = 1)]
    pub fn set_m_god_unit(self, value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_object: crate::unity_engine::gameobject::GameObject,
        initial_god_unit_index: i32,
        initial_menu_select: crate::app::basicmenuselect::BasicMenuSelect,
        select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::ringmenuitem::RingMenuItem_DecideEventHandler,
        request_close_event_handler : crate :: app :: synthesisringbaseringmenu :: SynthesisRingBaseRingMenu_CloseEventHandler,
    ) -> crate::app::synthesisringbaseringmenu::SynthesisRingBaseRingMenu;

    #[method(name = "CreateMenuItemList", args = 3)]
    pub fn create_menu_item_list(
        gid: ::unity2::Il2CppString,
        select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::ringmenuitem::RingMenuItem_DecideEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content : crate :: app :: synthesisringbaseringmenucontent :: SynthesisRingBaseRingMenuContent,
        god_unit_list: crate::system::collections::generic::list_1::List_1<
            crate::app::godunit::GodUnit,
        >,
        initial_god_unit_index: i32,
        select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::ringmenuitem::RingMenuItem_DecideEventHandler,
        request_close_event_handler : crate :: app :: synthesisringbaseringmenu :: SynthesisRingBaseRingMenu_CloseEventHandler,
    ) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "RebuildMenuItem", args = 0)]
    pub fn rebuild_menu_item(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "EnableMenu", args = 0)]
    pub fn enable_menu(self) -> ();

    #[method(name = "DisableMenu", args = 0)]
    pub fn disable_menu(self) -> ();

    #[method(name = "GetSelect", args = 0)]
    pub fn get_select(self) -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-synthesisringbaseringmenu")]
impl SynthesisRingBaseRingMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content : crate :: app :: synthesisringbaseringmenucontent :: SynthesisRingBaseRingMenuContent,
        god_unit_list: crate::system::collections::generic::list_1::List_1<
            crate::app::godunit::GodUnit,
        >,
        initial_god_unit_index: i32,
        select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::ringmenuitem::RingMenuItem_DecideEventHandler,
        request_close_event_handler : crate :: app :: synthesisringbaseringmenu :: SynthesisRingBaseRingMenu_CloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SynthesisRingBaseRingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISynthesisRingBaseRingMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            god_unit_list,
            initial_god_unit_index,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
        );
        this
    }
}
