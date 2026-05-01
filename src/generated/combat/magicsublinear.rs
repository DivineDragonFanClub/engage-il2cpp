
use crate::combat::magicsub::IMagicSub;
use crate::combat::magicsub::MagicSub;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/magicsublinear/MagicSubLinear.md")))]
#[::unity2::class(namespace = "Combat", name = "MagicSubLinear")]
#[parent(crate::combat::magicsub::MagicSub)]
pub struct MagicSubLinear {
    #[rename(name = "m_TargetNode")]
    pub m_target_node: crate::unity_engine::transform::Transform,
    #[rename(name = "m_StartPos")]
    pub m_start_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_EndPos")]
    pub m_end_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Forward")]
    pub m_forward: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Distance")]
    pub m_distance: f32,
    #[rename(name = "m_FlyingTime")]
    pub m_flying_time: f32,
    #[rename(name = "m_Elapsed")]
    pub m_elapsed: f32,
}

#[cfg(feature = "combat-magicsublinear")]
#[::unity2::methods]
impl MagicSubLinear {
    #[method(name = "Setup", args = 3)]
    pub fn setup(
        self,
        chr: crate::combat::character::Character,
        initial_start_pos: crate::unity_engine::vector3::Vector3,
        initial_end_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-magicsublinear")]
impl MagicSubLinear {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MagicSubLinear),
                ::core::stringify!(new),
            )
        });
        <Self as IMagicSubLinearMethods>::ctor(this);
        this
    }
}
