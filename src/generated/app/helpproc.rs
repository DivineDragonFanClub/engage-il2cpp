
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/helpproc/HelpProc_EventHandler.md")))]
#[::unity2::class(namespace = "App", name = "HelpProc.EventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct HelpProc_EventHandler {}

#[cfg(feature = "app-helpproc")]
#[::unity2::methods]
impl HelpProc_EventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-helpproc")]
impl HelpProc_EventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HelpProc_EventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IHelpProc_EventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/helpproc/HelpProc.md")))]
#[::unity2::class(namespace = "App", name = "HelpProc")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: helpproc :: HelpProc >)]
pub struct HelpProc {
    #[static_field]
    #[rename(name = "m_isTempGod")]
    pub m_is_temp_god: bool,
    #[static_field]
    #[rename(name = "m_TempGod")]
    pub m_temp_god: crate::app::godunit::GodUnit,
    #[static_field]
    #[rename(name = "m_isTempCommon")]
    pub m_is_temp_common: bool,
    #[static_field]
    #[rename(name = "m_TempCommon")]
    pub m_temp_common: crate::app::unitring::UnitRing,
    #[static_field]
    #[rename(name = "m_isTempUnit")]
    pub m_is_temp_unit: bool,
    #[static_field]
    #[rename(name = "m_TempUnit")]
    pub m_temp_unit: crate::app::unit::Unit,
    #[rename(name = "m_helpManager")]
    pub m_help_manager: crate::app::helpmanager::HelpManager,
    #[rename(name = "m_helpParamSetter")]
    pub m_help_param_setter: crate::app::helpparamsetter::HelpParamSetter,
    #[static_field]
    #[rename(name = "m_enter")]
    pub m_enter: crate::app::helpproc::HelpProc_EventHandler,
    #[static_field]
    #[rename(name = "m_exit")]
    pub m_exit: crate::app::helpproc::HelpProc_EventHandler,
    #[static_field]
    #[rename(name = "m_isDisable")]
    pub m_is_disable: bool,
}

#[cfg(feature = "app-helpproc")]
#[::unity2::methods]
impl HelpProc {
    #[method(name = "Begin", args = 0)]
    pub fn begin(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CommitMapPanelTarget", args = 0)]
    pub fn commit_map_panel_target(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "Enable", args = 0)]
    pub fn enable() -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable() -> ();

    #[method(name = "CreateBind", args = 10)]
    pub fn create_bind(
        parent: crate::app::procinst::ProcInst,
        target_object: crate::unity_engine::gameobject::GameObject,
        enter: crate::app::helpproc::HelpProc_EventHandler,
        exit: crate::app::helpproc::HelpProc_EventHandler,
        is_temp_god: bool,
        god: crate::app::godunit::GodUnit,
        is_temp_ring: bool,
        ring: crate::app::unitring::UnitRing,
        is_temp_unit: bool,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "CreateBindUnitStatus", args = 4)]
    pub fn create_bind_unit_status(
        parent: crate::app::procinst::ProcInst,
        enter: crate::app::helpproc::HelpProc_EventHandler,
        exit: crate::app::helpproc::HelpProc_EventHandler,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "CreateBindUnitInfo", args = 3)]
    pub fn create_bind_unit_info(
        parent: crate::app::procinst::ProcInst,
        enter: crate::app::helpproc::HelpProc_EventHandler,
        exit: crate::app::helpproc::HelpProc_EventHandler,
    ) -> ();

    #[method(name = "CraeteBindEngageLink", args = 6)]
    pub fn craete_bind_engage_link(
        parent: crate::app::procinst::ProcInst,
        enter: crate::app::helpproc::HelpProc_EventHandler,
        exit: crate::app::helpproc::HelpProc_EventHandler,
        target_object: crate::unity_engine::gameobject::GameObject,
        god: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "CreateBindGodRingSelect", args = 4)]
    pub fn create_bind_god_ring_select(
        parent: crate::app::procinst::ProcInst,
        target_object: crate::unity_engine::gameobject::GameObject,
        god: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "CreateBindCommonRingSelect", args = 4)]
    pub fn create_bind_common_ring_select(
        parent: crate::app::procinst::ProcInst,
        target_object: crate::unity_engine::gameobject::GameObject,
        ring: crate::app::unitring::UnitRing,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "CreateBindShopUnitSelect", args = 2)]
    pub fn create_bind_shop_unit_select(
        parent: crate::app::procinst::ProcInst,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-helpproc")]
impl HelpProc {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HelpProc),
                ::core::stringify!(new),
            )
        });
        <Self as IHelpProcMethods>::ctor(this);
        this
    }
}
