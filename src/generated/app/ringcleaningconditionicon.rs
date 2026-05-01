
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningconditionicon/RingCleaningConditionIcon.md")))]
#[::unity2::class(namespace = "App", name = "RingCleaningConditionIcon")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RingCleaningConditionIcon {
    #[rename(name = "Gauge")]
    pub gauge: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_GaugeAnimator")]
    pub m_gauge_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_GaugeController")]
    pub m_gauge_controller: crate::app::ringcleaninggaugecontroller::RingCleaningGaugeController,
}

#[cfg(feature = "app-ringcleaningconditionicon")]
#[::unity2::methods]
impl RingCleaningConditionIcon {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "PlayIconAnim", args = 2)]
    pub fn play_icon_anim(self, old_dirty: i32, new_dirty: i32) -> ();

    #[method(name = "ResetParam", args = 0)]
    pub fn reset_param(self) -> ();

    #[method(name = "SetPosition", args = 1)]
    pub fn set_position(self, x: f32) -> ();

    #[method(name = "SetBoolAnimators", args = 2)]
    pub fn set_bool_animators(self, name: ::unity2::Il2CppString, value: bool) -> ();

    #[method(name = "PlayAnimators", args = 1)]
    pub fn play_animators(self, state_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringcleaningconditionicon")]
impl RingCleaningConditionIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningConditionIcon),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningConditionIconMethods>::ctor(this);
        this
    }
}
