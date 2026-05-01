
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/particlefadeoutcomponent/ParticleFadeoutComponent.md")))]
#[::unity2::class(namespace = "Combat", name = "ParticleFadeoutComponent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ParticleFadeoutComponent {
    #[rename(name = "m_NameID")]
    pub m_name_id: i32,
    #[rename(name = "m_Elapsed")]
    pub m_elapsed: f32,
    #[rename(name = "m_Duration")]
    pub m_duration: f32,
    #[rename(name = "m_Materials")]
    pub m_materials: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::material::Material,
    >,
}

#[cfg(feature = "combat-particlefadeoutcomponent")]
#[::unity2::methods]
impl ParticleFadeoutComponent {
    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "SetLife", args = 1)]
    pub fn set_life(self, time: f32) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-particlefadeoutcomponent")]
impl ParticleFadeoutComponent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ParticleFadeoutComponent),
                ::core::stringify!(new),
            )
        });
        <Self as IParticleFadeoutComponentMethods>::ctor(this);
        this
    }
}
