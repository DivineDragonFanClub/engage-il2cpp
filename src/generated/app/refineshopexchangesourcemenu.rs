
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangesourcemenu/RefineShopExchangeSourceMenu_CloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopExchangeSourceMenu.CloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopExchangeSourceMenu_CloseEventHandler {}

#[cfg(feature = "app-refineshopexchangesourcemenu")]
#[::unity2::methods]
impl RefineShopExchangeSourceMenu_CloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refineshopexchangesourcemenu")]
impl RefineShopExchangeSourceMenu_CloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeSourceMenu_CloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeSourceMenu_CloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangesourcemenu/RefineShopExchangeSourceMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopExchangeSourceMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefineShopExchangeSourceMenu {
    #[rename(name = "m_TargetMaterialId")]
    pub m_target_material_id: ::unity2::Il2CppString,
    #[rename(name = "m_SourceMaterialId")]
    pub m_source_material_id: ::unity2::Il2CppString,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
    #[rename(name = "m_CloseEventHandler")]
    pub m_close_event_handler:
        crate::app::refineshopexchangesourcemenu::RefineShopExchangeSourceMenu_CloseEventHandler,
}

#[cfg(feature = "app-refineshopexchangesourcemenu")]
#[::unity2::methods]
impl RefineShopExchangeSourceMenu {
    #[method(name = "get_m_TargetMaterialData", args = 0)]
    pub fn get_m_target_material_data(
        self,
    ) -> crate::app::itemrefineexchangedata::ItemRefineExchangeData;

    #[method(name = "set_m_TargetMaterialData", args = 1)]
    pub fn set_m_target_material_data(
        self,
        value: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
    ) -> ();

    #[method(name = "get_m_SourceMaterialData", args = 0)]
    pub fn get_m_source_material_data(
        self,
    ) -> crate::app::itemrefineexchangedata::ItemRefineExchangeData;

    #[method(name = "set_m_SourceMaterialData", args = 1)]
    pub fn set_m_source_material_data(
        self,
        value: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
    ) -> ();

    #[method(name = "get_m_TargetMaterialRate", args = 0)]
    pub fn get_m_target_material_rate(self) -> i32;

    #[method(name = "set_m_TargetMaterialRate", args = 1)]
    pub fn set_m_target_material_rate(self, value: i32) -> ();

    #[method(name = "get_m_SourceMaterialRate", args = 0)]
    pub fn get_m_source_material_rate(self) -> i32;

    #[method(name = "set_m_SourceMaterialRate", args = 1)]
    pub fn set_m_source_material_rate(self, value: i32) -> ();

    #[method(name = "get_m_EnoughSourceCount", args = 0)]
    pub fn get_m_enough_source_count(self) -> bool;

    #[method(name = "set_m_EnoughSourceCount", args = 1)]
    pub fn set_m_enough_source_count(self, value: bool) -> ();

    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: refineshopexchangesourcemenucontent :: RefineShopExchangeSourceMenuContent,
        target_material_id: ::unity2::Il2CppString,
        initial_source_material_id: ::unity2::Il2CppString,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
        close_event_handler : crate :: app :: refineshopexchangesourcemenu :: RefineShopExchangeSourceMenu_CloseEventHandler,
    ) -> crate::app::refineshopexchangesourcemenu::RefineShopExchangeSourceMenu;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content : crate :: app :: refineshopexchangesourcemenucontent :: RefineShopExchangeSourceMenuContent,
        target_material_id: ::unity2::Il2CppString,
        initial_select_index: i32,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        close_event_handler : crate :: app :: refineshopexchangesourcemenu :: RefineShopExchangeSourceMenu_CloseEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelectMenuItem", args = 1)]
    pub fn on_select_menu_item(self, material_id: ::unity2::Il2CppString) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CalcRate", args = 0)]
    pub fn calc_rate(self) -> ();
}

#[cfg(feature = "app-refineshopexchangesourcemenu")]
impl RefineShopExchangeSourceMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content : crate :: app :: refineshopexchangesourcemenucontent :: RefineShopExchangeSourceMenuContent,
        target_material_id: ::unity2::Il2CppString,
        initial_select_index: i32,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        close_event_handler : crate :: app :: refineshopexchangesourcemenu :: RefineShopExchangeSourceMenu_CloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeSourceMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeSourceMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            target_material_id,
            initial_select_index,
            select_event_handler,
            close_event_handler,
        );
        this
    }
}
