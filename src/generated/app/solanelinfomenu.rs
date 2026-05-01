
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenu/SolanelInfoMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct SolanelInfoMenu_DecideEventHandler {}

#[cfg(feature = "app-solanelinfomenu")]
#[::unity2::methods]
impl SolanelInfoMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::solanelinfomenu::SolanelInfoMenu_InfoResult) -> ();
}

#[cfg(feature = "app-solanelinfomenu")]
impl SolanelInfoMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenu/SolanelInfoMenu.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SolanelInfoMenu {}

#[cfg(feature = "app-solanelinfomenu")]
#[::unity2::methods]
impl SolanelInfoMenu {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::solanelinfomenu::SolanelInfoMenu_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::solanelinfomenu::SolanelInfoMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        event_handler: crate::app::solanelinfomenu::SolanelInfoMenu_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::solanelinfomenucontent::SolanelInfoMenuContent,
        event_handler: crate::app::solanelinfomenu::SolanelInfoMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SetAreaDetail", args = 1)]
    pub fn set_area_detail(self, data: crate::app::hubareadata::HubAreaData) -> ();

    #[method(name = "RunFastTravel", args = 1)]
    pub fn run_fast_travel(self, hub_area_data: crate::app::hubareadata::HubAreaData) -> ();

    #[method(name = "GetSpriteAtlasManager", args = 0)]
    pub fn get_sprite_atlas_manager(self)
        -> crate::app::spriteatlasmanager_2::SpriteAtlasManager_2;
}

#[cfg(feature = "app-solanelinfomenu")]
impl SolanelInfoMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::solanelinfomenucontent::SolanelInfoMenuContent,
        event_handler: crate::app::solanelinfomenu::SolanelInfoMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuMethods>::ctor(this, menu_item_list, menu_content, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenu/SolanelInfoMenu_InfoResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SolanelInfoMenu_InfoResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SolanelInfoMenu_InfoResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SolanelInfoMenu.InfoResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SolanelInfoMenu_InfoResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SolanelInfoMenu_InfoResult {
    pub fn jump() -> Self {
        Self { value: 0 }
    }

    pub fn cancel() -> Self {
        Self { value: 1 }
    }
}
