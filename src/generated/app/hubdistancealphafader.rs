
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdistancealphafader/HubDistanceAlphaFader.md")))]
#[::unity2::class(namespace = "App", name = "HubDistanceAlphaFader")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubDistanceAlphaFader {
    #[rename(name = "m_distanecCurve")]
    pub m_distanec_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_propetyToID")]
    pub m_propety_to_id: i32,
    #[rename(name = "m_renderer")]
    pub m_renderer: crate::unity_engine::renderer::Renderer,
}

#[cfg(feature = "app-hubdistancealphafader")]
#[::unity2::methods]
impl HubDistanceAlphaFader {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubdistancealphafader")]
impl HubDistanceAlphaFader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubDistanceAlphaFader),
                ::core::stringify!(new),
            )
        });
        <Self as IHubDistanceAlphaFaderMethods>::ctor(this);
        this
    }
}
