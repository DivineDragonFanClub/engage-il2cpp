
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenatopmenu/ArenaTopMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaTopMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaTopMenu_DecideEventHandler {}

#[cfg(feature = "app-arenatopmenu")]
#[::unity2::methods]
impl ArenaTopMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, go_next: bool, index: i32) -> ();
}

#[cfg(feature = "app-arenatopmenu")]
impl ArenaTopMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaTopMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaTopMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenatopmenu/ArenaTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "ArenaTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ArenaTopMenu {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::arenatopmenu::ArenaTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-arenatopmenu")]
#[::unity2::methods]
impl ArenaTopMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        default_index: i32,
        decide_event_handler: crate::app::arenatopmenu::ArenaTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenatopmenucontent::ArenaTopMenuContent,
        decide_event_handler: crate::app::arenatopmenu::ArenaTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ToggleNormal", args = 1)]
    pub fn toggle_normal(self, index: i32) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-arenatopmenu")]
impl ArenaTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenatopmenucontent::ArenaTopMenuContent,
        decide_event_handler: crate::app::arenatopmenu::ArenaTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaTopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenatopmenu/ArenaTopMenu_TrainingMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ArenaTopMenu.TrainingMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ArenaTopMenu_TrainingMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::arenatopmenu::ArenaTopMenu_DecideEventHandler,
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_IsActive")]
    pub m_is_active: bool,
}

#[cfg(feature = "app-arenatopmenu")]
#[::unity2::methods]
impl ArenaTopMenu_TrainingMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::arenatopmenu::ArenaTopMenu_DecideEventHandler,
        index: i32,
        is_active: bool,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-arenatopmenu")]
impl ArenaTopMenu_TrainingMenuItem {
    pub fn new(
        decide_event_handler: crate::app::arenatopmenu::ArenaTopMenu_DecideEventHandler,
        index: i32,
        is_active: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaTopMenu_TrainingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaTopMenu_TrainingMenuItemMethods>::ctor(
            this,
            decide_event_handler,
            index,
            is_active,
        );
        this
    }
}
