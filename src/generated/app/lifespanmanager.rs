
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/lifespanmanager/LifespanManager.md")))]
#[::unity2::class(namespace = "App", name = "LifespanManager")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct LifespanManager {
    #[rename(name = "m_Life")]
    pub m_life: f32,
    #[rename(name = "m_Time")]
    pub m_time: f32,
}

#[cfg(feature = "app-lifespanmanager")]
#[::unity2::methods]
impl LifespanManager {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-lifespanmanager")]
impl LifespanManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LifespanManager),
                ::core::stringify!(new),
            )
        });
        <Self as ILifespanManagerMethods>::ctor(this);
        this
    }
}
