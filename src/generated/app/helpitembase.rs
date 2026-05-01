
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/helpitembase/HelpItemBase.md")))]
#[::unity2::class(namespace = "App", name = "HelpItemBase")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HelpItemBase {
    #[rename(name = "m_isTempGod")]
    pub m_is_temp_god: bool,
    #[rename(name = "m_tempGod")]
    pub m_temp_god: crate::app::godunit::GodUnit,
    #[rename(name = "m_isTempRing")]
    pub m_is_temp_ring: bool,
    #[rename(name = "m_tempRing")]
    pub m_temp_ring: crate::app::unitring::UnitRing,
    #[rename(name = "m_isTempUnit")]
    pub m_is_temp_unit: bool,
    #[rename(name = "m_tempUnit")]
    pub m_temp_unit: crate::app::unit::Unit,
    #[rename(name = "m_startItemPriority")]
    pub m_start_item_priority: i32,
    #[rename(name = "m_HelpItemType")]
    pub m_help_item_type: crate::app::helpmanager::HelpManager_HelpItemType,
}

#[cfg(feature = "app-helpitembase")]
#[::unity2::methods]
impl HelpItemBase {
    #[method(name = "get_StartItemPriority", args = 0)]
    pub fn get_start_item_priority(self) -> i32;

    #[method(name = "get_HelpItemType", args = 0)]
    pub fn get_help_item_type(self) -> crate::app::helpmanager::HelpManager_HelpItemType;

    #[method(name = "get_StartItemConstPriority", args = 0)]
    pub fn get_start_item_const_priority(self) -> i32;

    #[method(name = "SetContents", args = 1)]
    pub fn set_contents(self, setter: crate::app::helpparamsetter::HelpParamSetter) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "get_CurrentUnit", args = 0)]
    pub fn get_current_unit(self) -> crate::app::unit::Unit;

    #[method(name = "SetTempGod", args = 2)]
    pub fn set_temp_god(self, is_temp_god: bool, god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "IsTempGod", args = 0)]
    pub fn is_temp_god(self) -> bool;

    #[method(name = "GetTempGod", args = 0)]
    pub fn get_temp_god(self) -> crate::app::godunit::GodUnit;

    #[method(name = "SetTempRing", args = 2)]
    pub fn set_temp_ring(self, is_temp_ring: bool, ring: crate::app::unitring::UnitRing) -> ();

    #[method(name = "IsTempRing", args = 0)]
    pub fn is_temp_ring(self) -> bool;

    #[method(name = "GetTempRing", args = 0)]
    pub fn get_temp_ring(self) -> crate::app::unitring::UnitRing;

    #[method(name = "SetTempUnit", args = 2)]
    pub fn set_temp_unit(self, is_temp_unit: bool, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-helpitembase")]
impl HelpItemBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HelpItemBase),
                ::core::stringify!(new),
            )
        });
        <Self as IHelpItemBaseMethods>::ctor(this);
        this
    }
}
