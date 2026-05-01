
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponparammenu/RefineGodWeaponParamMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineGodWeaponParamMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineGodWeaponParamMenu_DecideEventHandler {}

#[cfg(feature = "app-refinegodweaponparammenu")]
#[::unity2::methods]
impl RefineGodWeaponParamMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        refine_data: crate::app::godweaponrefinedata::GodWeaponRefineData,
        refine_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        current_level: i32,
        sid: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "app-refinegodweaponparammenu")]
impl RefineGodWeaponParamMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponParamMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponParamMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponparammenu/RefineGodWeaponParamMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineGodWeaponParamMenu.RequestCloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineGodWeaponParamMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-refinegodweaponparammenu")]
#[::unity2::methods]
impl RefineGodWeaponParamMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refinegodweaponparammenu")]
impl RefineGodWeaponParamMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponParamMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponParamMenu_RequestCloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponparammenu/RefineGodWeaponParamMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponParamMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefineGodWeaponParamMenu {
    #[static_field]
    #[rename(name = "m_MenuItemOrder")]
    pub m_menu_item_order:
        ::unity2::Array<crate::app::godweaponrefinedata::GodWeaponRefineData_Kind>,
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::refinegodweaponparammenu::RefineGodWeaponParamMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refinegodweaponparammenu::RefineGodWeaponParamMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::refinegodweaponparammenu::RefineGodWeaponParamMenu_RequestCloseEventHandler,
}

#[cfg(feature = "app-refinegodweaponparammenu")]
#[::unity2::methods]
impl RefineGodWeaponParamMenu {
    #[method(name = "get_m_GodUnit", args = 0)]
    pub fn get_m_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "set_m_GodUnit", args = 1)]
    pub fn set_m_god_unit(self, value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_m_GodWeaponList", args = 0)]
    pub fn get_m_god_weapon_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = "set_m_GodWeaponList", args = 1)]
    pub fn set_m_god_weapon_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>,
    ) -> ();

    #[method(name = "get_m_GodWeaponIndex", args = 0)]
    pub fn get_m_god_weapon_index(self) -> i32;

    #[method(name = "set_m_GodWeaponIndex", args = 1)]
    pub fn set_m_god_weapon_index(self, value: i32) -> ();

    #[method(name = "get_m_GodWeapon", args = 0)]
    pub fn get_m_god_weapon(self) -> crate::app::itemdata::ItemData;

    #[method(name = "set_m_GodWeapon", args = 1)]
    pub fn set_m_god_weapon(self, value: crate::app::itemdata::ItemData) -> ();

    #[method(name = "get_m_RefineOrReset", args = 0)]
    pub fn get_m_refine_or_reset(self) -> bool;

    #[method(name = "set_m_RefineOrReset", args = 1)]
    pub fn set_m_refine_or_reset(self, value: bool) -> ();

    #[method(name = "CreateBind", args = 8)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::refinegodweaponparammenucontent::RefineGodWeaponParamMenuContent,
        god_unit: crate::app::godunit::GodUnit,
        initial_god_weapon: crate::app::itemdata::ItemData,
        refine_or_reset: bool,
        select_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_RequestCloseEventHandler,
    ) -> crate::app::refinegodweaponparammenu::RefineGodWeaponParamMenu;

    #[method(name = "CreateMenuItem", args = 6)]
    pub fn create_menu_item(
        god_weapon: crate::app::itemdata::ItemData,
        god_unit: crate::app::godunit::GodUnit,
        refine_or_reset: bool,
        select_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_RequestCloseEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 9)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
        initial_god_weapon_index: i32,
        refine_or_reset: bool,
        select_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSelect", args = 0)]
    pub fn get_select(self) -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refinegodweaponparammenu")]
impl RefineGodWeaponParamMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
        initial_god_weapon_index: i32,
        refine_or_reset: bool,
        select_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponParamMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponParamMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            god_unit,
            god_weapon_list,
            initial_god_weapon_index,
            refine_or_reset,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponparammenu/RefineGodWeaponParamMenu_SelectEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineGodWeaponParamMenu.SelectEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineGodWeaponParamMenu_SelectEventHandler {}

#[cfg(feature = "app-refinegodweaponparammenu")]
#[::unity2::methods]
impl RefineGodWeaponParamMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        refine_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        refine_or_reset: bool,
    ) -> ();
}

#[cfg(feature = "app-refinegodweaponparammenu")]
impl RefineGodWeaponParamMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponParamMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponParamMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
