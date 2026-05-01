
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponselectmanager/RefineGodWeaponSelectManager_ReturnEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineGodWeaponSelectManager.ReturnEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineGodWeaponSelectManager_ReturnEventHandler {}

#[cfg(feature = "app-refinegodweaponselectmanager")]
#[::unity2::methods]
impl RefineGodWeaponSelectManager_ReturnEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        result: crate::app::basicmenu::BasicMenu_Result,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::unititem::UnitItem,
        scroll_index: i32,
    ) -> ();
}

#[cfg(feature = "app-refinegodweaponselectmanager")]
impl RefineGodWeaponSelectManager_ReturnEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponSelectManager_ReturnEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponSelectManager_ReturnEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponselectmanager/RefineGodWeaponSelectManager.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponSelectManager")]
#[parent(crate::system::object::Object)]
pub struct RefineGodWeaponSelectManager {
    #[rename(name = "m_ReturnEventHandler")]
    pub m_return_event_handler:
        crate::app::refinegodweaponselectmanager::RefineGodWeaponSelectManager_ReturnEventHandler,
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::refinegodweaponselectmenu::RefineGodWeaponSelectMenu,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
}

#[cfg(feature = "app-refinegodweaponselectmanager")]
#[::unity2::methods]
impl RefineGodWeaponSelectManager {
    #[method(name = "Create", args = 6)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
        initial_god_unit: crate::app::godunit::GodUnit,
        initial_god_weapon: crate::app::itemdata::ItemData,
        initial_scroll_index: i32,
        return_event_handler : crate :: app :: refinegodweaponselectmanager :: RefineGodWeaponSelectManager_ReturnEventHandler,
    ) -> crate::app::refinegodweaponselectmanager::RefineGodWeaponSelectManager;

    #[method(name = "CreateForReset", args = 6)]
    pub fn create_for_reset(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
        initial_god_unit: crate::app::godunit::GodUnit,
        initial_god_weapon: crate::app::itemdata::ItemData,
        initial_scroll_index: i32,
        return_event_handler : crate :: app :: refinegodweaponselectmanager :: RefineGodWeaponSelectManager_ReturnEventHandler,
    ) -> crate::app::refinegodweaponselectmanager::RefineGodWeaponSelectManager;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        super_: crate::app::procinst::ProcInst,
        root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
        initial_god_unit: crate::app::godunit::GodUnit,
        initial_god_weapon: crate::app::itemdata::ItemData,
        initial_scroll_index: i32,
        return_event_handler : crate :: app :: refinegodweaponselectmanager :: RefineGodWeaponSelectManager_ReturnEventHandler,
    ) -> ();

    #[method(name = "OnSelect", args = 2)]
    pub fn on_select(
        self,
        god_unit: crate::app::godunit::GodUnit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "OnDecide", args = 2)]
    pub fn on_decide(
        self,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "OnRequestClose", args = 0)]
    pub fn on_request_close(self) -> ();
}

#[cfg(feature = "app-refinegodweaponselectmanager")]
impl RefineGodWeaponSelectManager {
    pub fn new(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
        initial_god_unit: crate::app::godunit::GodUnit,
        initial_god_weapon: crate::app::itemdata::ItemData,
        initial_scroll_index: i32,
        return_event_handler : crate :: app :: refinegodweaponselectmanager :: RefineGodWeaponSelectManager_ReturnEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponSelectManager),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponSelectManagerMethods>::ctor(
            this,
            super_,
            root,
            initial_god_unit,
            initial_god_weapon,
            initial_scroll_index,
            return_event_handler,
        );
        this
    }
}
