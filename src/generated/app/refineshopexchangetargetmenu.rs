
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangetargetmenu/RefineShopExchangeTargetMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopExchangeTargetMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefineShopExchangeTargetMenu {
    #[rename(name = "m_CloseEventHandler")]
    pub m_close_event_handler:
        crate::app::refineshopexchangetargetmenu::RefineShopExchangeTargetMenu_CloseEventHandler,
}

#[cfg(feature = "app-refineshopexchangetargetmenu")]
#[::unity2::methods]
impl RefineShopExchangeTargetMenu {
    #[method(name = "get_m_MenuItemIsEnoughMaterial", args = 0)]
    pub fn get_m_menu_item_is_enough_material(self) -> bool;

    #[method(name = "set_m_MenuItemIsEnoughMaterial", args = 1)]
    pub fn set_m_menu_item_is_enough_material(self, value: bool) -> ();

    #[method(name = "get_m_MenuItemIsMaxMaterial", args = 0)]
    pub fn get_m_menu_item_is_max_material(self) -> bool;

    #[method(name = "set_m_MenuItemIsMaxMaterial", args = 1)]
    pub fn set_m_menu_item_is_max_material(self, value: bool) -> ();

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: refineshopexchangetargetmenucontent :: RefineShopExchangeTargetMenuContent,
        initial_target_material_id: ::unity2::Il2CppString,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
        close_event_handler : crate :: app :: refineshopexchangetargetmenu :: RefineShopExchangeTargetMenu_CloseEventHandler,
    ) -> crate::app::refineshopexchangetargetmenu::RefineShopExchangeTargetMenu;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content : crate :: app :: refineshopexchangetargetmenucontent :: RefineShopExchangeTargetMenuContent,
        initial_select_index: i32,
        close_event_handler : crate :: app :: refineshopexchangetargetmenu :: RefineShopExchangeTargetMenu_CloseEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "UpdateContent", args = 1)]
    pub fn update_content(self, menu_item_is_enabled: bool) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-refineshopexchangetargetmenu")]
impl RefineShopExchangeTargetMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content : crate :: app :: refineshopexchangetargetmenucontent :: RefineShopExchangeTargetMenuContent,
        initial_select_index: i32,
        close_event_handler : crate :: app :: refineshopexchangetargetmenu :: RefineShopExchangeTargetMenu_CloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeTargetMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeTargetMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_select_index,
            close_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangetargetmenu/RefineShopExchangeTargetMenu_CloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopExchangeTargetMenu.CloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopExchangeTargetMenu_CloseEventHandler {}

#[cfg(feature = "app-refineshopexchangetargetmenu")]
#[::unity2::methods]
impl RefineShopExchangeTargetMenu_CloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refineshopexchangetargetmenu")]
impl RefineShopExchangeTargetMenu_CloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeTargetMenu_CloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeTargetMenu_CloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}
