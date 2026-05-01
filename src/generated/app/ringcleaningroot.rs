
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningroot/RingCleaningRoot.md")))]
#[::unity2::class(namespace = "App", name = "RingCleaningRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RingCleaningRoot {
    #[rename(name = "m_ConditionIcon")]
    pub m_condition_icon: crate::app::ringcleaningconditionicon::RingCleaningConditionIcon,
    #[rename(name = "m_GaugeController")]
    pub m_gauge_controller: crate::app::ringcleaninggaugecontroller::RingCleaningGaugeController,
    #[rename(name = "m_Cloth")]
    pub m_cloth: crate::app::ringcleaningcloth::RingCleaningCloth,
    #[rename(name = "m_Ring")]
    pub m_ring: crate::app::ringcleaningringcontroller::RingCleaningRingController,
}

#[cfg(feature = "app-ringcleaningroot")]
#[::unity2::methods]
impl RingCleaningRoot {
    #[method(name = "get_GaugeController", args = 0)]
    pub fn get_gauge_controller(
        self,
    ) -> crate::app::ringcleaninggaugecontroller::RingCleaningGaugeController;

    #[method(name = "get_ConditionIcon", args = 0)]
    pub fn get_condition_icon(
        self,
    ) -> crate::app::ringcleaningconditionicon::RingCleaningConditionIcon;

    #[method(name = "Init", args = 3)]
    pub fn init(
        self,
        god: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
        ring_controller: crate::app::ringcleaningringcontroller::RingCleaningRingController,
    ) -> ();

    #[method(name = "UpdateDirty", args = 1)]
    pub fn update_dirty(self, new_dirty: i32) -> ();

    #[method(name = "IsFinishCleaning", args = 0)]
    pub fn is_finish_cleaning(self) -> bool;

    #[method(name = "GetTextureDirty", args = 1)]
    pub fn get_texture_dirty(self, dirty: i32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringcleaningroot")]
impl RingCleaningRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningRootMethods>::ctor(this);
        this
    }
}
