
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/muscledifficultmenu/MuscleDifficultMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "MuscleDifficultMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MuscleDifficultMenu_DecideEventHandler {}

#[cfg(feature = "app-muscledifficultmenu")]
#[::unity2::methods]
impl MuscleDifficultMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::muscle_exercise::level::Level) -> ();
}

#[cfg(feature = "app-muscledifficultmenu")]
impl MuscleDifficultMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleDifficultMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleDifficultMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/muscledifficultmenu/MuscleDifficultMenu_MuscleDifficultMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MuscleDifficultMenu.MuscleDifficultMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MuscleDifficultMenu_MuscleDifficultMenuItem {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::muscledifficultmenu::MuscleDifficultMenu_DecideEventHandler,
    #[rename(name = "m_IsEnable")]
    pub m_is_enable: bool,
}

#[cfg(feature = "app-muscledifficultmenu")]
#[::unity2::methods]
impl MuscleDifficultMenu_MuscleDifficultMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        event_handler: crate::app::muscledifficultmenu::MuscleDifficultMenu_DecideEventHandler,
        enable: bool,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-muscledifficultmenu")]
impl MuscleDifficultMenu_MuscleDifficultMenuItem {
    pub fn new(
        name: ::unity2::Il2CppString,
        event_handler: crate::app::muscledifficultmenu::MuscleDifficultMenu_DecideEventHandler,
        enable: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleDifficultMenu_MuscleDifficultMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleDifficultMenu_MuscleDifficultMenuItemMethods>::ctor(
            this,
            name,
            event_handler,
            enable,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/muscledifficultmenu/MuscleDifficultMenu.md")))]
#[::unity2::class(namespace = "App", name = "MuscleDifficultMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MuscleDifficultMenu {
    #[rename(name = "m_EnableChecker")]
    pub m_enable_checker: ::unity2::Array<bool>,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::muscledifficultmenu::MuscleDifficultMenu_DecideEventHandler,
    #[rename(name = "m_PastDeside")]
    pub m_past_deside: i32,
}

#[cfg(feature = "app-muscledifficultmenu")]
#[::unity2::methods]
impl MuscleDifficultMenu {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        decide_event_handler : crate :: app :: muscledifficultmenu :: MuscleDifficultMenu_DecideEventHandler,
        enable_array: ::unity2::Array<bool>,
        past_deside: i32,
    ) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        past_deside: i32,
        initial_select: i32,
        decide_event_handler : crate :: app :: muscledifficultmenu :: MuscleDifficultMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-muscledifficultmenu")]
impl MuscleDifficultMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        decide_event_handler : crate :: app :: muscledifficultmenu :: MuscleDifficultMenu_DecideEventHandler,
        enable_array: ::unity2::Array<bool>,
        past_deside: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleDifficultMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleDifficultMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            decide_event_handler,
            enable_array,
            past_deside,
        );
        this
    }
}
