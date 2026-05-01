
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/reinscontroller/ReinsController.md")))]
#[::unity2::class(namespace = "Combat", name = "ReinsController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ReinsController {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "m_bInitialized")]
    pub m_b_initialized: bool,
    #[rename(name = "m_LastPos")]
    pub m_last_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "names")]
    pub names: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "transforms")]
    pub transforms: ::unity2::Array<crate::unity_engine::transform::Transform>,
    #[rename(name = "WorldOffset")]
    pub world_offset: crate::unity_engine::vector3::Vector3,
    #[rename(name = "FrameLatencyCompensation")]
    pub frame_latency_compensation: bool,
}

#[cfg(feature = "combat-reinscontroller")]
#[::unity2::methods]
impl ReinsController {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "SetWeight", args = 1)]
    pub fn set_weight(self, weight: f32) -> f32;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "MyStart", args = 0)]
    pub fn my_start(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-reinscontroller")]
impl ReinsController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReinsController),
                ::core::stringify!(new),
            )
        });
        <Self as IReinsControllerMethods>::ctor(this);
        this
    }
}
