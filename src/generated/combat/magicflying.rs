
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/magicflying/MagicFlying.md")))]
#[::unity2::class(namespace = "Combat", name = "MagicFlying")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MagicFlying {
    #[rename(name = "m_StartPos")]
    pub m_start_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_EndPos")]
    pub m_end_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_ArrivalTime")]
    pub m_arrival_time: f32,
    #[rename(name = "easeType")]
    pub ease_type: crate::app::curve::Curve_Type,
    #[rename(name = "easePower")]
    pub ease_power: i32,
}

#[cfg(feature = "combat-magicflying")]
#[::unity2::methods]
impl MagicFlying {
    #[method(name = "SetEase", args = 2)]
    pub fn set_ease(self, ease_type: crate::app::curve::Curve_Type, ease_power: i32) -> ();

    #[method(name = "Shoot", args = 3)]
    pub fn shoot(
        self,
        start_pos: crate::unity_engine::vector3::Vector3,
        end_pos: crate::unity_engine::vector3::Vector3,
        arrival_time: f32,
    ) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "ChangeStartEndPos", args = 2)]
    pub fn change_start_end_pos(
        self,
        start_pos: crate::unity_engine::vector3::Vector3,
        end_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetPositionToEnd_IfUnShoot", args = 1)]
    pub fn set_position_to_end_if_un_shoot(
        self,
        target_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-magicflying")]
impl MagicFlying {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MagicFlying),
                ::core::stringify!(new),
            )
        });
        <Self as IMagicFlyingMethods>::ctor(this);
        this
    }
}
