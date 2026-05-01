
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubminimaplayer/HubMiniMapLayer.md")))]
#[::unity2::class(namespace = "App", name = "HubMiniMapLayer")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubMiniMapLayer {
    #[rename(name = "m_Layer")]
    pub m_layer: i32,
    #[rename(name = "m_EnvSoundID")]
    pub m_env_sound_id: ::unity2::Il2CppString,
}

#[cfg(feature = "app-hubminimaplayer")]
#[::unity2::methods]
impl HubMiniMapLayer {
    #[method(name = "OnTriggerEnter", args = 1)]
    pub fn on_trigger_enter(self, other: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "OnTriggerExit", args = 1)]
    pub fn on_trigger_exit(self, other: crate::unity_engine::collider::Collider) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubminimaplayer")]
impl HubMiniMapLayer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMiniMapLayer),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMiniMapLayerMethods>::ctor(this);
        this
    }
}
