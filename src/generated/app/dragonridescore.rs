
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridescore/DragonRideScore.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideScore")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DragonRideScore {
    #[rename(name = "scoreText")]
    pub score_text: crate::unity_engine::ui::text::Text,
}

#[cfg(feature = "app-dragonridescore")]
#[::unity2::methods]
impl DragonRideScore {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonridescore")]
impl DragonRideScore {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideScore),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideScoreMethods>::ctor(this);
        this
    }
}
