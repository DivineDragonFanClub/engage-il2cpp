
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/ps19_sendtime/PS19_SendTime.md")))]
#[::unity2::class(namespace = "Combat", name = "PS19_SendTime")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct PS19_SendTime {
    #[rename(name = "m_UniformNameID")]
    pub m_uniform_name_id: i32,
    #[rename(name = "m_Chr")]
    pub m_chr: crate::combat::character::Character,
    #[rename(name = "m_Now")]
    pub m_now: f32,
    #[rename(name = "m_EndTime")]
    pub m_end_time: f32,
    #[rename(name = "PS19List")]
    pub ps19_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::material::Material,
    >,
}

#[cfg(feature = "combat-ps19_sendtime")]
#[::unity2::methods]
impl PS19_SendTime {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Begin", args = 1)]
    pub fn begin(self, time_to_end: f32) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-ps19_sendtime")]
impl PS19_SendTime {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PS19_SendTime),
                ::core::stringify!(new),
            )
        });
        <Self as IPS19_SendTimeMethods>::ctor(this);
        this
    }
}
